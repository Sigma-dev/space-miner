use bevy::{prelude::*, render::render_resource::*, sprite::*};
use line::Line;
use line_group::LineGroup;
use line_mesh::LineMeshPlugin;

pub mod line;
pub mod line_group;
pub mod line_mesh;

const LINE_NUMBER: usize = 256;

pub struct LineRendererPlugin;

impl Plugin for LineRendererPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(LineRendererWidth(0.01))
            .add_plugins(Material2dPlugin::<LineRendererMaterial>::default())
            .add_plugins(LineMeshPlugin);
    }
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct LineRendererMaterial {
    #[uniform(0)]
    pub settings: LineRendererSettings,
}

impl Material2d for LineRendererMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/lines.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode2d {
        AlphaMode2d::Blend
    }
}

impl LineRendererMaterial {
    pub fn new(settings: LineRendererSettings) -> LineRendererMaterial {
        LineRendererMaterial { settings }
    }
}

#[derive(ShaderType, Debug, Clone)]
pub struct LineRendererSettings {
    pub lines: [Vec4; LINE_NUMBER],
    pub width: f32,
}

impl LineRendererSettings {
    pub fn new(line_group: LineGroup, width: f32) -> LineRendererSettings {
        let lines = line_group.get_lines();
        let line_amount = lines.len();
        if line_amount > LINE_NUMBER {
            panic!("Too many lines!")
        }
        let mut lines_array: [Line; LINE_NUMBER] = [Line::new(Vec2::ZERO, Vec2::ZERO); LINE_NUMBER];
        for i in 0..line_amount {
            lines_array[i] = lines[i];
        }
        LineRendererSettings {
            lines: lines_array
                .iter()
                .map(|l| l.as_vec4())
                .collect::<Vec<Vec4>>()
                .try_into()
                .unwrap(),
            width,
        }
    }

    pub fn get_lines(&self) -> LineGroup {
        LineGroup::new(self.lines.map(|l| Line::new(l.xy(), l.zw())).to_vec())
    }
}

#[derive(Resource)]
pub struct LineRendererWidth(f32);
