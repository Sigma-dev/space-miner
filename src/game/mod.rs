use asteroid::AsteroidPlugin;
use bevy::prelude::*;
use ship::ShipPlugin;
use ui::UIPlugin;

pub mod asteroid;
pub mod camera;
pub mod ship;
pub mod ui;

pub struct GamePlugin;

#[derive(PartialEq, Eq)]
pub enum GameState {
    PLAYING,
    GAMEOVER,
}

#[derive(Resource)]
pub struct CurrentGameState(pub GameState);

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((ShipPlugin, AsteroidPlugin, UIPlugin))
            .insert_resource(CurrentGameState(GameState::PLAYING));
    }
}
