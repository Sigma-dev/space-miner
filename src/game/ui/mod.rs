use bevy::prelude::*;

use crate::{
    blink::Blink, game::GameState, line::Line, line_group::LineGroup, line_mesh::LineMesh,
};

use super::{ship::inventory::InventoryUpdate, CurrentGameState};
pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                init_storage,
                handle_storage_line,
                handle_lerp,
                handle_death_ui,
            ),
        );
    }
}

#[derive(Component)]
pub struct StorageLine {
    max_offset: f32,
    target_pos: f32,
}

#[derive(Component)]
pub struct DeathUI;

impl StorageLine {
    pub fn new(max_offset: f32) -> StorageLine {
        StorageLine {
            max_offset,
            target_pos: 0.,
        }
    }
}

pub fn spawn_ui(commands: &mut Commands, camera: Entity) {
    let storage_height = 150.;
    let storage_width = 20.;
    let storage_ui_lines = LineGroup::generate_continuous_closed(vec![
        Vec2::new(-storage_width, storage_height),
        Vec2::new(storage_width, storage_height),
        Vec2::new(storage_width, -storage_height),
        Vec2::new(-storage_width, -storage_height),
    ])
    .concat(
        LineGroup::text("EMPTY")
            .scaled(2.)
            .offset(Vec2::new(-125., -140.)),
    )
    .concat(
        LineGroup::text("FULL")
            .scaled(2.)
            .offset(Vec2::new(-105., 140.)),
    );

    let storage_ui = commands
        .spawn((
            LineMesh(storage_ui_lines),
            Transform::from_xyz(550., -150., 0.),
        ))
        .id();
    let storage_line = commands
        .spawn((
            LineMesh(LineGroup::from_line(Line::new(
                Vec2::new(-storage_width * 1.3, 0.),
                Vec2::new(storage_width * 1.3, 0.),
            ))),
            StorageLine::new(storage_height * 2.),
        ))
        .id();
    commands.entity(camera).add_child(storage_ui);
    commands.entity(storage_ui).add_child(storage_line);
    let death_ui = commands
        .spawn((DeathUI, Visibility::Hidden, Transform::default()))
        .id();

    commands.entity(camera).add_child(death_ui);
    let game_over = commands
        .spawn(LineMesh(LineGroup::text("GAME OVER").scaled(7.).centered()))
        .id();
    commands.entity(death_ui).add_child(game_over);

    let press_r = commands
        .spawn((
            LineMesh(
                LineGroup::text("PRESS R TO TRY AGAIN")
                    .scaled(2.)
                    .centered()
                    .offset(-Vec2::Y * 100.),
            ),
            Blink::new(2., true, Visibility::Hidden),
        ))
        .id();
    commands.entity(death_ui).add_child(press_r);
}

fn init_storage(mut line_q: Query<(&mut Transform, &StorageLine), Added<StorageLine>>) {
    for (mut transform, line) in line_q.iter_mut() {
        transform.translation.y = -0.5 * line.max_offset;
    }
}

fn handle_storage_line(
    mut events: EventReader<InventoryUpdate>,
    mut line_q: Query<&mut StorageLine>,
) {
    for event in events.read() {
        for mut line in line_q.iter_mut() {
            line.target_pos =
                ((event.new_amount as f32 / event.max_amount as f32) - 0.5) * line.max_offset;
        }
    }
}

fn handle_death_ui(
    game_state: Res<CurrentGameState>,
    mut death_q: Query<&mut Visibility, With<DeathUI>>,
) {
    if game_state.0 == GameState::GAMEOVER {
        for mut vis in death_q.iter_mut() {
            *vis = Visibility::Inherited
        }
    }
}

fn handle_lerp(time: Res<Time>, mut line_q: Query<(&mut Transform, &StorageLine)>) {
    for (mut transform, line) in line_q.iter_mut() {
        transform.translation.y = transform
            .translation
            .y
            .lerp(line.target_pos, 10. * time.delta_secs());
    }
}
