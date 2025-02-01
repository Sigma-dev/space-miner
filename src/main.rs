use audio_manager::{AudioManagerPlugin, AudioManagerResource};
use avian2d::{prelude::Gravity, PhysicsPlugins};
use bevy::asset::AssetMetaCheck;
use bevy::{prelude::*, window::PresentMode};
use blink::BlinkPlugin;
use camera_shake::CameraShakePlugin;
use delayed_despawn::DelayedDespawnPlugin;
use follow_entity::FollowEntityPlugin;
use game::camera::spawn_camera;
use game::ui::spawn_ui;
use game::GamePlugin;
use game::{ship::spawn_ship, CurrentGameState, GameState};
use health::HealthPlugin;
use level_manager::{LevelManager, LevelManagerPlugin, LevelReset};
use line_renderer::*;

pub mod audio_manager;
pub mod bevy_utils;
mod blink;
mod camera_shake;
pub mod delayed_despawn;
mod follow_entity;
mod game;
pub mod health;
mod level_manager;
pub mod line_renderer;
mod rand;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        present_mode: PresentMode::AutoNoVsync,
                        ..default()
                    }),
                    ..default()
                })
                .set(AssetPlugin {
                    meta_check: AssetMetaCheck::Never,
                    ..default()
                }),
            PhysicsPlugins::default(),
        ))
        .add_plugins((
            AudioManagerPlugin,
            LineRendererPlugin,
            HealthPlugin,
            DelayedDespawnPlugin,
            BlinkPlugin,
            FollowEntityPlugin,
            GamePlugin,
            LevelManagerPlugin,
            CameraShakePlugin,
        ))
        .insert_resource(ClearColor(Color::srgb(0.05, 0.05, 0.05)))
        .insert_resource(Gravity(Vec2::ZERO))
        .insert_resource(AudioManagerResource::new(0.01))
        .add_systems(Startup, setup)
        .add_systems(Update, handle_reset)
        .add_systems(Update, level_setup.run_if(on_event::<LevelReset>))
        .run();
}

fn setup(mut level_manager: LevelManager) {
    level_manager.reset();
}

fn handle_reset(keys: Res<ButtonInput<KeyCode>>, mut level_manager: LevelManager) {
    if keys.just_pressed(KeyCode::KeyR) {
        level_manager.reset();
    }
}

fn level_setup(
    mut commands: Commands,
    mut level_manager: LevelManager,
    mut game_state: ResMut<CurrentGameState>,
) {
    game_state.0 = GameState::PLAYING;
    let ship = spawn_ship(&mut commands);
    let camera = spawn_camera(&mut level_manager, ship);
    spawn_ui(&mut commands, camera);
}
