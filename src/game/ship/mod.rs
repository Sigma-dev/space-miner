use avian2d::prelude::{
    AngularVelocity, Collider, CollisionStarted, ExternalForce, LinearVelocity, Mass, RigidBody,
    Sensor, TransformInterpolation,
};
use bevy::prelude::*;
use inventory::{inventory_plugin, Inventory};
use laser::laser_plugin;
use thrusters::{thrusters_plugin, ThrustersVisuals};

use crate::{
    audio_manager::{AudioManager, PlayAudio2D},
    bevy_utils::query_double,
    blink::Blink,
    game::asteroid::Asteroid,
    health::{DamageTaken, Death, Health, HealthHitInvincibilityTime, HealthManager},
    level_manager::LevelScoped,
    line_group::LineGroup,
    rand::{random_range, random_vec2_range},
    system_param::LineRenderer,
};

use super::{CurrentGameState, GameState};

pub mod inventory;
mod laser;
mod thrusters;

pub struct ShipPlugin;

impl Plugin for ShipPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((laser_plugin, thrusters_plugin, inventory_plugin))
            .add_event::<ThrustersToggle>()
            .add_event::<FireLaser>()
            .add_systems(FixedUpdate, fixed_update)
            .add_systems(
                Update,
                (
                    update,
                    asteroid_collisions,
                    ship_blink,
                    ship_hurt,
                    ship_death,
                ),
            );
    }
}

#[derive(Component)]
pub struct Ship {
    pub(crate) thruster_power: f32,
    pub(crate) rotation_power: f32,
    is_thrusting: bool,
}

#[derive(Event)]
pub struct ThrustersToggle {
    enabled: bool,
}

#[derive(Event)]
pub struct FireLaser {
    position: Vec2,
    direction: Dir2,
}

fn fixed_update(
    mut ship_q: Query<(&mut Transform, &Ship, &mut ExternalForce)>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    for (mut transform, ship, mut force) in ship_q.iter_mut() {
        if ship.is_thrusting {
            force.set_force(transform.up().xy() * ship.thruster_power);
        }
        let rotation_input = -(keys.pressed(KeyCode::KeyA) as i32 as f32)
            + (keys.pressed(KeyCode::KeyD) as i32 as f32);
        transform.rotate_local_z(-rotation_input * ship.rotation_power);
    }
}

fn update(
    keys: Res<ButtonInput<KeyCode>>,
    buttons: Res<ButtonInput<MouseButton>>,
    mut ship_q: Query<(&Transform, &mut Ship)>,
    mut thrusters_e: EventWriter<ThrustersToggle>,
    mut laser_e: EventWriter<FireLaser>,
) {
    for (transform, mut ship) in ship_q.iter_mut() {
        if keys.just_pressed(KeyCode::KeyW) {
            ship.is_thrusting = true;
            thrusters_e.send(ThrustersToggle { enabled: true });
        }
        if keys.just_released(KeyCode::KeyW) {
            ship.is_thrusting = false;
            thrusters_e.send(ThrustersToggle { enabled: false });
        }
        if buttons.just_pressed(MouseButton::Left) {
            let forward = transform.local_y().xy();
            let spawn_pos = transform.translation.xy() + forward * 30.;
            laser_e.send(FireLaser {
                position: spawn_pos,
                direction: Dir2::new(forward).unwrap(),
            });
        }
    }
}

pub fn spawn_ship(commands: &mut Commands, line_renderer: &mut LineRenderer) -> Entity {
    let (ship_shape, thruster_shape) = get_ship_and_thrusters_shape(3);
    let ship = line_renderer.spawn(
        ship_shape,
        (
            RigidBody::Dynamic,
            Collider::circle(10.),
            ExternalForce::default().with_persistence(false),
            Mass(1.),
            Sensor,
            TransformInterpolation,
            Health::new(30.),
            HealthHitInvincibilityTime::new(2.),
            Blink::new(10., false, Visibility::Inherited),
            Ship {
                thruster_power: 600.,
                rotation_power: 0.075,
                is_thrusting: false,
            },
            Inventory::new(10),
            LevelScoped,
        ),
    );

    let thrusters = line_renderer.spawn(
        thruster_shape,
        (ThrustersVisuals, Blink::new(15., false, Visibility::Hidden)),
    );

    commands.entity(ship).add_child(thrusters);
    return ship;
}

pub fn ship_blink(
    time: Res<Time>,
    mut ship_q: Query<(&HealthHitInvincibilityTime, &mut Blink), With<Ship>>,
) {
    for (invincibility, mut blink) in ship_q.iter_mut() {
        if invincibility
            .last_hit_time
            .is_some_and(|last| time.elapsed_secs() < last + invincibility.invincibility_time)
        {
            blink.enabled = true
        } else {
            blink.enabled = false
        }
    }
}

pub fn ship_hurt(
    mut audio_manager: AudioManager,
    mut damage_r: EventReader<DamageTaken>,
    ship_q: Query<Entity, With<Ship>>,
    mut line_renderer: LineRenderer,
) {
    for damage in damage_r.read() {
        if let Ok(ship) = ship_q.get(damage.entity) {
            audio_manager.play_sound(PlayAudio2D::new_once("sounds/hurt.wav".to_string()));
            line_renderer.update(
                ship,
                get_ship_and_thrusters_shape((damage.new_hp / 10.) as u32).0,
            );
        }
    }
}

pub fn ship_death(
    mut commands: Commands,
    mut audio_manager: AudioManager,
    mut death_r: EventReader<Death>,
    ship_q: Query<(Entity, &Transform, &LinearVelocity), With<Ship>>,
    mut line_renderer: LineRenderer,
    mut game_state: ResMut<CurrentGameState>,
) {
    for death in death_r.read() {
        audio_manager.play_sound(PlayAudio2D::new_once("sounds/ship_destroy.wav"));
        audio_manager.toggle_audio_off("sounds/thrusters.wav");
        if let Ok((ship, transform, ship_velocity)) = ship_q.get(death.entity) {
            let lines = line_renderer.get_lines(ship);
            for line in lines.get_lines() {
                line_renderer.spawn(
                    LineGroup::from_line(*line),
                    (
                        transform.clone(),
                        RigidBody::Dynamic,
                        Mass(1.),
                        LinearVelocity(ship_velocity.0 + random_vec2_range(-100.0..100.)),
                        AngularVelocity(random_range(-1.0..1.)),
                        LevelScoped,
                    ),
                );
            }
            commands.entity(ship).despawn_recursive();
            game_state.0 = GameState::GAMEOVER;
        }
    }
}

pub fn asteroid_collisions(
    mut collision_event_reader: EventReader<CollisionStarted>,
    ship_q: Query<Entity, With<Ship>>,
    asteroid_q: Query<Entity, With<Asteroid>>,
    mut health_manager: HealthManager,
) {
    for CollisionStarted(e1, e2) in collision_event_reader.read() {
        let Some((ship, _asteroid)) = query_double(&ship_q, &asteroid_q, *e1, *e2) else {
            continue;
        };
        health_manager.damage(ship, 10.);
    }
}

pub fn get_ship_and_thrusters_shape(remaining_hp: u32) -> (LineGroup, LineGroup) {
    let width = 15.;
    let length = 30.;
    let reactor_width = 10.;
    let bottom_left_corner = Vec2::new(-width, -length);
    let bottom_right_corner = Vec2::new(width, -length);
    let front_corner = Vec2::new(0., length);
    let left_slope_dir = front_corner - bottom_left_corner;
    let right_slope_dir = front_corner - bottom_right_corner;

    let mut points = vec![
        bottom_left_corner,
        front_corner,
        bottom_right_corner,
        Vec2::new(width - reactor_width, -length),
        Vec2::new(width - 12., -length + 10.),
        Vec2::new(-width + 12., -length + 10.),
        Vec2::new(-width + reactor_width, -length),
    ];
    if remaining_hp <= 2 {
        points.splice(
            1..1,
            [
                bottom_left_corner + left_slope_dir * 0.6,
                bottom_left_corner + left_slope_dir * 0.65 + Vec2::new(5., 0.),
                bottom_left_corner + left_slope_dir * 0.7,
            ],
        );
    }
    if remaining_hp <= 1 {
        points.splice(
            5..5,
            [
                bottom_right_corner + right_slope_dir * 0.5,
                bottom_right_corner + right_slope_dir * 0.4 + Vec2::new(-7., 0.),
                bottom_right_corner + right_slope_dir * 0.3,
            ],
        );
    }

    let thrusters = LineGroup::generate_continuous(vec![
        Vec2::new(-width, -length),
        Vec2::new(-width + reactor_width / 2., -length - 20.),
        Vec2::new(-width + reactor_width, -length),
    ])
    .concat(LineGroup::generate_continuous(vec![
        Vec2::new(width, -length),
        Vec2::new(width - reactor_width / 2., -length - 20.),
        Vec2::new(width - reactor_width, -length),
    ]));

    return (LineGroup::generate_continuous_closed(points), thrusters);
}
