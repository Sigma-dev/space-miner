#import bevy_sprite::{mesh2d_vertex_output::VertexOutput}
const lines_number: u32 = 256;

struct LineRendererSettings
{
    lines: array<vec4<f32>, lines_number>,
    width: f32
}

@group(2) @binding(0) var<uniform> properties: LineRendererSettings;

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    if (is_near_a_line(in.uv)) {
        return vec4<f32>(1.);
    }
    return vec4<f32>(0.);
}

fn is_near_a_line(uv: vec2<f32>) -> bool {
    let centered_uv = uv * 2. - vec2<f32>(1., 1.);
    for (var i: u32 = 0u; i < lines_number; i = i + 1u) {
        let line = properties.lines[i];
        let dist = distance_to_segment(centered_uv, vec2<f32>(line.x, line.y), vec2<f32>(line.z, line.w));
        if dist < 0 {
            continue;
        }
        if dist < properties.width {
            return true;
        }
    }
    return false;
}

fn distance_to_segment(p: vec2<f32>, a: vec2<f32>, b: vec2<f32>) -> f32 {
    let ab = b - a;
    let ap = p - a;
    let ab_length_squared = dot(ab, ab);
    if ab_length_squared == 0. {
        return -1.;
    }
    var t = dot(ap, ab) / ab_length_squared;
    t = clamp(t, 0.0, 1.0);
    let closest_point = a + t * ab;
    return length(p - closest_point);
}