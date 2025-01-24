use bevy::{ecs::system::*, prelude::*};

use super::{line_group::LineGroup, LineRendererMaterial, LineRendererSettings, LineRendererWidth};

#[derive(Component)]
struct LineHistory {
    lines: LineGroup,
}

impl LineHistory {
    pub fn new(lines: LineGroup) -> LineHistory {
        LineHistory { lines }
    }
}

#[derive(SystemParam)]
pub struct LineRenderer<'w, 's> {
    #[doc(hidden)]
    pub commands: Commands<'w, 's>,
    #[doc(hidden)]
    pub meshes: ResMut<'w, Assets<Mesh>>,
    #[doc(hidden)]
    pub line_materials: ResMut<'w, Assets<LineRendererMaterial>>,
    #[doc(hidden)]
    pub line_width: Res<'w, LineRendererWidth>,
    #[doc(hidden)]
    lines: Query<'w, 's, &'static LineHistory>,
}

impl<'w, 's> LineRenderer<'w, 's> {
    pub fn spawn<T: Bundle>(&mut self, lines: LineGroup, bundle: T) -> Entity {
        let (mesh, settings) = self.lines_to_mesh(&lines);
        self.commands
            .spawn((bundle, mesh, settings, LineHistory::new(lines)))
            .id()
    }

    pub fn update(&mut self, entity: Entity, lines: LineGroup) {
        let mesh_bundle = self.lines_to_mesh(&lines);
        self.commands
            .entity(entity)
            .insert((mesh_bundle, LineHistory::new(lines)));
    }

    pub fn get_lines(&self, entity: Entity) -> LineGroup {
        self.lines.get(entity).unwrap().lines.clone()
    }

    fn lines_to_mesh(
        &mut self,
        lines: &LineGroup,
    ) -> (Mesh2d, MeshMaterial2d<LineRendererMaterial>) {
        let padding = 1.2;
        let bounding_size = lines.calculate_bounding_size();
        let actual_size = bounding_size * padding;
        let scaling_factor = bounding_size / 100.;
        let adjusted_width = self.line_width.0 / scaling_factor;
        let adjusted_lines = lines.scaled((1. / bounding_size) * (1. / padding));
        let flipped_lines = adjusted_lines.flipped_vertically();
        (
            Mesh2d(self.meshes.add(Rectangle {
                half_size: Vec2::splat(actual_size),
            })),
            MeshMaterial2d(self.line_materials.add(LineRendererMaterial::new(
                LineRendererSettings::new(flipped_lines, adjusted_width),
            ))),
        )
    }
}
