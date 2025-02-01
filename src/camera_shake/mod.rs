use bevy::{math::FloatPow, prelude::*};

use crate::rand::random_smooth;

#[derive(Component)]
#[require(Transform, Visibility)]
pub struct CameraShakeParent {
    trauma: f32,
    frequency: f32,
    decay_rate: f32,
    max_trauma: f32,
    translational_amplitude: f32,
    rotational_amplitude: f32,
}

impl CameraShakeParent {
    pub fn new(
        frequency: f32,
        decay_rate: f32,
        max_trauma: f32,
        translational_amplitude: f32,
        rotational_amplitude: f32,
    ) -> CameraShakeParent {
        CameraShakeParent {
            trauma: 0.,
            decay_rate,
            frequency,
            max_trauma,
            translational_amplitude,
            rotational_amplitude,
        }
    }
}

#[derive(Event)]
pub struct ShakeCamera {
    pub trauma: f32,
}

impl ShakeCamera {
    pub fn new(trauma: f32) -> ShakeCamera {
        ShakeCamera { trauma }
    }
}

pub struct CameraShakePlugin;

impl Plugin for CameraShakePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ShakeCamera>()
            .add_systems(Update, handle_shake)
            .add_systems(Update, handle_shake_events);
    }
}

fn handle_shake(time: Res<Time>, mut shake_q: Query<(&mut Transform, &mut CameraShakeParent)>) {
    for (mut transform, mut shake) in shake_q.iter_mut() {
        if shake.trauma < 0. {
            continue;
        }
        let mult = shake.trauma.cubed();
        transform.translation.x = random_smooth(time.elapsed_secs() * shake.frequency)
            * shake.translational_amplitude
            * mult;
        transform.translation.y = random_smooth((time.elapsed_secs() + 15.) * shake.frequency)
            * shake.translational_amplitude
            * mult;
        transform.rotation = Quat::from_axis_angle(
            Vec3::Z,
            random_smooth(time.elapsed_secs() * shake.frequency)
                * shake.rotational_amplitude
                * mult,
        );
        shake.trauma -= shake.decay_rate * time.delta_secs();
    }
}

fn handle_shake_events(
    mut shake_r: EventReader<ShakeCamera>,
    mut shake_q: Query<&mut CameraShakeParent>,
) {
    for shake_e in shake_r.read() {
        for mut shake in shake_q.iter_mut() {
            shake.trauma += shake_e.trauma;
            shake.trauma = shake.trauma.min(shake.max_trauma);
        }
    }
}
