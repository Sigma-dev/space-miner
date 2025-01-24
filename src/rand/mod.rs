use bevy::math::Vec2;
use rand::{distributions::uniform::{SampleRange, SampleUniform}, Rng};

pub fn random_vec2_range<T: SampleRange<f32> + Clone>(range: T) -> Vec2 {
    Vec2::new(
        random_range(range.clone()),
        random_range(range),
    )
}

pub fn random_range<T, R: SampleRange<T>>(range: R) -> T
where
T: SampleUniform,
R: SampleRange<T>
{
    rand::thread_rng().gen_range(range)
}