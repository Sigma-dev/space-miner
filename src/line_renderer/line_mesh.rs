use bevy::prelude::*;

use super::{line_group::LineGroup, LineRendererMaterial, LineRendererSettings, LineRendererWidth};

#[derive(Component)]
pub struct LineMesh(pub LineGroup);

pub struct LineMeshPlugin;

impl Plugin for LineMeshPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, on_change);
    }
}

pub fn on_change(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut line_materials: ResMut<Assets<LineRendererMaterial>>,
    line_q: Query<(Entity, &LineMesh), Changed<LineMesh>>,
    line_width: Res<LineRendererWidth>,
) {
    for (entity, line) in line_q.iter() {
        let Some(mut entity_commands) = commands.get_entity(entity) else {
            continue;
        };
        let mesh_bundle = lines_to_mesh(&line.0, line_width.0, &mut meshes, &mut line_materials);
        entity_commands.try_insert(mesh_bundle);
    }
}

pub fn lines_to_mesh(
    lines: &LineGroup,
    line_width: f32,
    meshes: &mut Assets<Mesh>,
    line_materials: &mut Assets<LineRendererMaterial>,
) -> (Mesh2d, MeshMaterial2d<LineRendererMaterial>) {
    let padding = 1.2;
    let bounding_size = lines.calculate_bounding_size();
    let actual_size = bounding_size * padding;
    let scaling_factor = bounding_size / 100.;
    let adjusted_width = line_width / scaling_factor;
    let adjusted_lines = lines.scaled((1. / bounding_size) * (1. / padding));
    let flipped_lines = adjusted_lines.flipped_vertically();
    (
        Mesh2d(meshes.add(Rectangle {
            half_size: Vec2::splat(actual_size),
        })),
        MeshMaterial2d(
            line_materials.add(LineRendererMaterial::new(LineRendererSettings::new(
                flipped_lines,
                adjusted_width,
            ))),
        ),
    )
}
