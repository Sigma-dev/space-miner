use bevy::prelude::*;

pub struct FollowEntityPlugin;

impl Plugin for FollowEntityPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreUpdate, handle_followers);
    }
}

#[derive(Component)]
pub struct FollowEntity {
    following: Entity,
    rate: f32,
}

impl FollowEntity {
    pub fn new(following: Entity, rate: f32) -> FollowEntity {
        FollowEntity { following, rate }
    }
}

fn handle_followers(
    time: Res<Time>,
    mut lerps_q: Query<(&mut Transform, &FollowEntity)>,
    transforms_q: Query<&Transform, Without<FollowEntity>>,
) {
    for (mut transform, lerp) in lerps_q.iter_mut() {
        let Ok(target_transform) = transforms_q.get(lerp.following) else {
            continue;
        };
        let new_pos = transform.translation.xy().lerp(
            target_transform.translation.xy(),
            lerp.rate * time.delta_secs(),
        );
        transform.translation = new_pos.extend(transform.translation.z);
    }
}
