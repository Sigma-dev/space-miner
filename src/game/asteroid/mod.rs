use std::time::Duration;

use avian2d::prelude::{
    Collider, ExternalForce, Mass, RigidBody, SpatialQuery, SpatialQueryFilter,
};
use bevy::{prelude::*, time::common_conditions::on_timer};
use ore::{get_lines_for_ore, AsteroidOre, AsteroidOrePlugin, OreType};

use crate::{
    audio_manager::{AudioManager, PlayAudio2D},
    game::ship::Ship,
    health::{Death, Health},
    level_manager::LevelScoped,
    line_group::LineGroup,
    rand::random_range,
    system_param::LineRenderer,
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
pub struct Asteroid;

pub enum AsteroidContent {
    Empty,
    Ore(LineGroup),
}

fn spawn_random_asteroid(
    mut line_renderer: LineRenderer,
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
    if random > 0.8 && radius > 50. {
        spawn_asteroid(
            &mut line_renderer,
            radius,
            world_pos,
            dir,
            AsteroidContent::Ore(get_lines_for_ore(OreType::Crystal)),
            &spatial_query,
        );
    } else {
        spawn_asteroid(
            &mut line_renderer,
            radius,
            world_pos,
            dir,
            AsteroidContent::Empty,
            &spatial_query,
        );
    }
}

fn spawn_asteroid(
    line_renderer: &mut LineRenderer,
    radius: f32,
    world_pos: Vec2,
    dir: Vec2,
    content: AsteroidContent,
    spatial_query: &SpatialQuery,
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
    if !spatial_query
        .shape_intersections(&collider, world_pos, 0., &SpatialQueryFilter::default())
        .is_empty()
    {
        return;
    }
    let asteroid = line_renderer.spawn(
        lines,
        (
            Transform::from_translation(world_pos.extend(0.)),
            RigidBody::Dynamic,
            Mass(1.),
            collider,
            ExternalForce::new(dir * 15. * 1000.).with_persistence(false),
            Health::new_destroy_on_death(10.),
            Asteroid,
            LevelScoped,
        ),
    );
    if let Some(ore) = maybe_ore {
        line_renderer.commands.entity(asteroid).insert(ore);
    }
}

fn handle_asteroid_destroyed(
    mut death_e: EventReader<Death>,
    mut audio_manager: AudioManager,
    asteroid_q: Query<&Asteroid>,
) {
    for event in death_e.read() {
        if let Ok(_asteroid) = asteroid_q.get(event.entity) {
            audio_manager.play_sound(PlayAudio2D::new_once("sounds/destroy.wav".to_owned()));
        }
    }
}
