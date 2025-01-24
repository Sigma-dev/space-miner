use bevy::prelude::*;

#[derive(Component)]
pub struct DelayedDespawn {
    pub duration: f32,
    creation_time: f32,
}

impl DelayedDespawn {
    pub fn new(creation_time: f32, duration: f32) -> DelayedDespawn {
        DelayedDespawn {
            creation_time,
            duration,
        }
    }
}

pub struct DelayedDespawnPlugin;

impl Plugin for DelayedDespawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_despawn);
    }
}

fn handle_despawn(
    mut commands: Commands,
    time: Res<Time>,
    destroy_q: Query<(Entity, &DelayedDespawn)>,
) {
    for (entity, destroy) in destroy_q.iter() {
        if time.elapsed_secs() > destroy.creation_time + destroy.duration {
            commands.entity(entity).despawn();
        }
    }
}
