struct CameraUniform {
    view_proj: mat4x4<f32>,
}

@group(0) @binding(0)
var<uniform> camera: CameraUniform;

struct VertexOutput {
    @builtin(position) clip_pos: vec4<f32>,
}

@vertex
fn vs_main(@builtin(vertex_index) ix: u32) -> VertexOutput {
    let pos = array<vec2<f32>, 3>(vec2<f32>(-0.5, -0.5), vec2<f32>(0.5, -0.5), vec2<f32>(0.0, 0.5));

    let world_pos = vec4<f32>(pos[ix], 0.0, 1.0);
    let clip_pos = camera.view_proj * world_pos;

    var output: VertexOutput;
    output.clip_pos = clip_pos;
    return output;
}

@fragment
fn fs_main(v: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(1.0, 0.0, 0.0, 1.0);
}
