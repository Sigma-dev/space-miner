use bevy::{core_pipeline::bloom::*, prelude::*};

use crate::{follow_entity::FollowEntity, level_manager::LevelManager};

pub fn spawn_camera(level_manager: &mut LevelManager, ship: Entity) -> Entity {
    let mut projection = OrthographicProjection::default_2d();
    projection.scale = 1.;

    level_manager.spawn((
        Camera2d,
        Camera {
            hdr: true,
            ..default()
        },
        projection,
        Bloom {
            intensity: 0.07,
            low_frequency_boost: 1.,
            low_frequency_boost_curvature: 0.9,
            high_pass_frequency: 0.5,
            composite_mode: BloomCompositeMode::Additive,
            prefilter: BloomPrefilter {
                threshold: 0.1,
                threshold_softness: 1.,
            },
            ..default()
        },
        FollowEntity::new(ship, 10.),
    ))
}
