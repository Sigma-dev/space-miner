use bevy::prelude::*;

use crate::{audio_manager::AudioManager, blink::Blink};

use super::ThrustersToggle;

#[derive(Component)]
pub struct ThrustersVisuals;

pub fn thrusters_plugin(app: &mut App) {
    app.add_systems(Update, toggle_thrusters);
}

fn toggle_thrusters(
    mut audio_manager: AudioManager,
    mut thrusters_e: EventReader<ThrustersToggle>,
    mut thrusters_q: Query<&mut Blink, With<ThrustersVisuals>>,
) {
    for event in thrusters_e.read() {
        for mut thrusters in thrusters_q.iter_mut() {
            thrusters.enabled = event.enabled;
            audio_manager.toggle_audio(
                &"sounds/thrusters.wav".to_string(),
                event.enabled,
                Some(1.),
            );
        }
    }
}
