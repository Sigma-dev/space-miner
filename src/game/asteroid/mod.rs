use std::time::Duration;

use avian2d::prelude::{
    Collider, LinearVelocity, Mass, RigidBody, SpatialQuery, SpatialQueryFilter,
};
use bevy::{prelude::*, time::common_conditions::on_timer};
use ore::{get_lines_for_ore, AsteroidOre, AsteroidOrePlugin, OreType};

use crate::{
    audio_manager::{AudioManager, PlayAudio2D},
    camera_shake::ShakeCamera,
    game::ship::Ship,
    health::{Death, Health},
    level_manager::LevelScoped,
    line_group::LineGroup,
    line_mesh::LineMesh,
    rand::random_range,
};

pub mod ore;
pub struct AsteroidPlugin;

impl Plugin for AsteroidPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(AsteroidOrePlugin).add_systems(
            Update,
            (
                spawn_random_asteroid.run_if(on_timer(Duration::from_secs_f32(0.5))),
                handle_asteroid_destroyed,
            ),
        );
    }
}

#[derive(Component)]
pub struct Asteroid {
    pub radius: f32,
}

impl Asteroid {
    pub fn new(radius: f32) -> Asteroid {
        Asteroid { radius }
    }
}

pub enum AsteroidContent {
    Empty,
    Ore(LineGroup),
}

fn spawn_random_asteroid(
    mut commands: Commands,
    ship_q: Query<&Transform, With<Ship>>,
    spatial_query: SpatialQuery,
) {
    let Ok(ship_transform) = ship_q.get_single() else {
        return;
    };
    let ship_pos = ship_transform.translation.xy();
    let pos = Circle::default().sample_boundary(&mut rand::thread_rng());
    let radius = random_range(30.0..80.);
    let world_pos = ship_pos + (pos * 2000.);
    let dir = (ship_pos - world_pos).normalize();
    let random = random_range(0.0..1.0);
    {
        spawn_asteroid(
            &mut commands,
            radius,
            world_pos,
            dir * random_range(150.0..250.),
            if random > 0.8 && radius > 50. {
                AsteroidContent::Ore(get_lines_for_ore(OreType::Crystal))
            } else {
                AsteroidContent::Empty
            },
            Some(&spatial_query),
        );
    }
}

fn spawn_asteroid(
    commands: &mut Commands,
    radius: f32,
    world_pos: Vec2,
    velocity: Vec2,
    content: AsteroidContent,
    maybe_spatial_query: Option<&SpatialQuery>,
) {
    let mut asteroid_shape = LineGroup::generate_random_circle(radius, 8, 5.);
    let shape_as_line = asteroid_shape.to_unique_points_looped();
    let mut maybe_ore = None;
    let lines = match content {
        AsteroidContent::Empty => asteroid_shape,
        AsteroidContent::Ore(ore) => {
            let amount = random_range(6..8);
            asteroid_shape.extend(ore.scatter_circle(radius / 2., amount, 10., true));
            maybe_ore = Some(AsteroidOre::new(OreType::Crystal, amount));
            asteroid_shape
        }
    };
    let collider = Collider::polyline(shape_as_line, None);
    if maybe_spatial_query.is_some_and(|spatial_query| {
        !spatial_query
            .shape_intersections(&collider, world_pos, 0., &SpatialQueryFilter::default())
            .is_empty()
    }) {
        return;
    }
    let asteroid = commands
        .spawn((
            LineMesh(lines),
            Transform::from_translation(world_pos.extend(0.)),
            RigidBody::Dynamic,
            Mass(1.),
            collider,
            LinearVelocity(velocity),
            Health::new_destroy_on_death(10.),
            Asteroid::new(radius),
            LevelScoped,
        ))
        .id();
    if let Some(ore) = maybe_ore {
        commands.entity(asteroid).insert(ore);
    }
}

fn handle_asteroid_destroyed(
    mut commands: Commands,
    mut death_e: EventReader<Death>,
    mut shake_w: EventWriter<ShakeCamera>,
    mut audio_manager: AudioManager,
    asteroid_q: Query<(&Transform, &LinearVelocity, &Asteroid, Option<&AsteroidOre>)>,
) {
    for event in death_e.read() {
        if let Ok((transform, velocity, asteroid, maybe_ore)) = asteroid_q.get(event.entity) {
            audio_manager.play_sound(PlayAudio2D::new_once("sounds/destroy.wav".to_owned()));
            shake_w.send(ShakeCamera::new(0.4));
            if asteroid.radius > 50. && maybe_ore.is_none() {
                let orthogonal = velocity.normalize().perp() * asteroid.radius / 2.;
                spawn_asteroid(
                    &mut commands,
                    asteroid.radius / 2.,
                    transform.translation.xy() + orthogonal,
                    **velocity,
                    AsteroidContent::Empty,
                    None,
                );
                spawn_asteroid(
                    &mut commands,
                    asteroid.radius / 2.,
                    transform.translation.xy() - orthogonal,
                    **velocity,
                    AsteroidContent::Empty,
                    None,
                );
            }
        }
    }
}
