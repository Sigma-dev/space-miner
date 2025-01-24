use bevy::{ecs::system::*, prelude::*};

#[derive(Component)]
pub struct LevelScoped;

#[derive(Event, Debug)]
pub struct LevelReset;

#[derive(SystemParam)]
pub struct LevelManager<'w, 's> {
    #[doc(hidden)]
    pub commands: Commands<'w, 's>,
    #[doc(hidden)]
    scoped: Query<'w, 's, Entity, With<LevelScoped>>,
    #[doc(hidden)]
    reset_writer: EventWriter<'w, LevelReset>,
}

impl<'w, 's> LevelManager<'w, 's> {
    pub fn spawn<T: Bundle>(&mut self, bundle: T) -> Entity {
        self.commands.spawn((bundle, LevelScoped)).id()
    }

    pub fn reset(&mut self) {
        for entity in self.scoped.iter() {
            self.commands.entity(entity).despawn_recursive();
        }
        self.reset_writer.send(LevelReset);
    }
}

pub struct LevelManagerPlugin;

impl Plugin for LevelManagerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<LevelReset>();
    }
}
