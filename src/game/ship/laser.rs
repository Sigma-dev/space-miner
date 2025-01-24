use avian2d::{
    math::PI,
    prelude::{Collider, CollisionStarted, LinearDamping, LinearVelocity, RigidBody, Sensor},
};
use bevy::prelude::*;

use crate::{
    audio_manager::{AudioManager, PlayAudio2D},
    bevy_utils::query_double,
    delayed_despawn::DelayedDespawn,
    game::asteroid::Asteroid,
    health::HealthManager,
    level_manager::LevelScoped,
    line::Line,
    line_group::LineGroup,
    line_mesh::LineMesh,
    rand::random_range,
};

use super::FireLaser;

#[derive(Component)]
pub struct Laser {
    speed: f32,
}

pub fn laser_plugin(app: &mut App) {
    app.add_systems(
        Update,
        (handle_laser_spawning, handle_lasers, asteroid_collisions),
    );
}

fn handle_lasers(time: Res<Time>, mut lasers_q: Query<(&mut Transform, &Laser)>) {
    for (mut transform, laser) in lasers_q.iter_mut() {
        let dir = transform.local_y().as_vec3();
        transform.translation += dir * laser.speed * time.delta_secs() * 60.;
    }
}

pub fn handle_laser_spawning(
    mut commands: Commands,
    mut laser_e: EventReader<FireLaser>,
    mut audio_manager: AudioManager,
    time: Res<Time>,
) {
    for event in laser_e.read() {
        let position = event.position;
        let direction = event.direction;
        let angle = direction.y.atan2(direction.x) - PI / 2.;
        let transform = Transform::from_translation(position.extend(0.))
            .with_rotation(Quat::from_rotation_z(angle)); //.looking_at(target, Vec3::new(0., 0., 1.));
        let lines = LineGroup::new(vec![Line::new(Vec2::ZERO, Vec2::new(0., 20.))]);
        commands.spawn((
            LineMesh(lines),
            transform,
            Collider::capsule(5., 40.),
            Sensor,
            Laser { speed: 20. },
            LevelScoped,
            DelayedDespawn::new(time.elapsed_secs(), 5.),
        ));
        audio_manager
            .play_sound(PlayAudio2D::new_once("sounds/laser.wav".to_owned()).with_volume(1.5));
    }
}

pub fn asteroid_collisions(
    mut commands: Commands,
    mut collision_event_reader: EventReader<CollisionStarted>,
    laser_q: Query<(Entity, &Transform), With<Laser>>,
    asteroid_q: Query<(Entity, &Transform), With<Asteroid>>,
    mut health_manager: HealthManager,
    time: Res<Time>,
) {
    for CollisionStarted(e1, e2) in collision_event_reader.read() {
        let Some(((laser, laser_transfrom), (asteroid, _asteroid_transform))) =
            query_double(&laser_q, &asteroid_q, *e1, *e2)
        else {
            continue;
        };
        commands.entity(laser).try_despawn();
        for _ in 0..random_range(4..7) {
            let pos = laser_transfrom.translation;
            let rotation = laser_transfrom.rotation
                * Quat::from_axis_angle(
                    Vec3::Z,
                    (180. as f32 + random_range(-60.0..60.)).to_radians(),
                );
            let lines = LineGroup::from_line(Line::new(Vec2::new(0., 0.), Vec2::new(0., 10.)))
                .scaled(random_range(0.5..1.1));
            commands.spawn((
                LineMesh(lines),
                Transform::from_translation(pos).with_rotation(rotation),
                RigidBody::Dynamic,
                LinearVelocity((rotation.mul_vec3(Vec3::Y) * random_range(300.0..1000.)).xy()),
                LinearDamping(5.),
                DelayedDespawn::new(time.elapsed_secs(), random_range(0.05..0.2)),
            ));
        }
        health_manager.damage(asteroid, 10.);
    }
}
