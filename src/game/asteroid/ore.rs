use avian2d::prelude::{
    Collider, Collision, ExternalForce, LinearDamping, Mass, RigidBody, Sensor,
};
use bevy::prelude::*;

use crate::{
    audio_manager::{AudioManager, PlayAudio2D},
    bevy_utils::query_double_mut,
    game::ship::{inventory::Inventory, Ship},
    health::Death,
    level_manager::LevelScoped,
    line_group::LineGroup,
    rand::{random_range, random_vec2_range},
    system_param::LineRenderer,
};

use super::Asteroid;

#[derive(Component)]
pub struct AsteroidOre {
    ore_type: OreType,
    amount: u32,
}

impl AsteroidOre {
    pub fn new(ore_type: OreType, amount: u32) -> AsteroidOre {
        AsteroidOre { ore_type, amount }
    }
}

#[derive(Component)]
pub struct Ore;

#[derive(Clone, Copy)]
pub enum OreType {
    Crystal,
}

pub struct AsteroidOrePlugin;

impl Plugin for AsteroidOrePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (handle_asteroid_destroyed, handle_ore_pickup));
    }
}

fn handle_asteroid_destroyed(
    mut line_renderer: LineRenderer,
    mut death_e: EventReader<Death>,
    asteroid_q: Query<(&Transform, &Asteroid, &AsteroidOre)>,
) {
    for event in death_e.read() {
        if let Ok((transform, _asteroid, ore)) = asteroid_q.get(event.entity) {
            for _ in 0..ore.amount {
                let lines = get_lines_for_ore(ore.ore_type);
                let collider = Collider::polyline(lines.to_unique_points_looped(), None);
                let ore = line_renderer.spawn(
                    lines,
                    (
                        RigidBody::Dynamic,
                        ExternalForce::new(random_vec2_range(-3000.0..3000.0))
                            .with_persistence(false),
                        LinearDamping(2.),
                        Mass(0.1),
                        collider,
                        Transform::from_translation(
                            transform.translation + random_vec2_range(-50.0..50.).extend(0.),
                        )
                        .with_rotation(Quat::from_rotation_z(
                            (random_range(0.0..360.0) as f32).to_radians(),
                        )),
                        Ore,
                        LevelScoped,
                    ),
                );
                let zone = line_renderer
                    .commands
                    .spawn((Sensor, Collider::circle(200.)))
                    .id();
                line_renderer.commands.entity(ore).add_child(zone);
            }
        }
    }
}

pub fn handle_ore_pickup(
    mut commands: Commands,
    mut audio_manager: AudioManager,
    mut collision_event_reader: EventReader<Collision>,
    mut ore_q: Query<(Entity, &Transform, &mut ExternalForce, &Ore)>,
    mut ship_q: Query<(&Transform, &mut Inventory), With<Ship>>,
    mut parent_q: Query<&Parent>,
) {
    let mut already = Vec::new();

    for Collision(contacts) in collision_event_reader.read() {
        let Some(((ship_transform, mut ship_inventory), parent)) = query_double_mut(
            &mut ship_q,
            &mut parent_q,
            contacts.entity1,
            contacts.entity2,
        ) else {
            continue;
        };
        let Ok((ore_entity, ore_transform, mut force, _ore)) = ore_q.get_mut(**parent) else {
            continue;
        };

        let ore_amount = 1;
        if !ship_inventory.can_add(ore_amount) {
            continue;
        };
        let diff = ship_transform.translation - ore_transform.translation;
        let dir = diff.normalize().xy();
        force.set_force(dir * 200.);

        if ore_transform
            .translation
            .distance(ship_transform.translation)
            < 20.
        {
            if already.contains(&ore_entity) {
                continue;
            }
            if !ship_inventory.try_add(ore_amount) {
                continue;
            }
            already.push(ore_entity);
            commands.entity(ore_entity).try_despawn();
            audio_manager.play_sound(PlayAudio2D::new_once("sounds/pickup.wav").with_volume(0.2));
        }
    }
}

pub fn get_lines_for_ore(ore: OreType) -> LineGroup {
    match ore {
        OreType::Crystal => LineGroup::generate_continuous_closed(vec![
            Vec2::new(-6., 0.),
            Vec2::new(0., 10.),
            Vec2::new(6., 0.),
            Vec2::new(0., -10.),
        ]),
    }
}
