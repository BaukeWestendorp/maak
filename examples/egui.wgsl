struct CameraUniform {
    view_proj: mat4x4<f32>,
}

@group(0) @binding(0)
var<uniform> camera: CameraUniform;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
}

struct InstanceInput {
    @location(3) model_matrix_0: vec4<f32>,
    @location(4) model_matrix_1: vec4<f32>,
    @location(5) model_matrix_2: vec4<f32>,
    @location(6) model_matrix_3: vec4<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) normal: vec3<f32>,
    @location(1) uv: vec2<f32>,
    @location(2) world_position: vec3<f32>,
}

@vertex
fn vs_main(v: VertexInput, i: InstanceInput) -> VertexOutput {
    var out: VertexOutput;

    let model_matrix = mat4x4<f32>(
            i.model_matrix_0,
            i.model_matrix_1,
            i.model_matrix_2,
            i.model_matrix_3,
        );

    let world_position = model_matrix * vec4<f32>(v.position, 1.0);
    out.clip_position = camera.view_proj * world_position;

    out.world_position = world_position.xyz;

    let normal_matrix = mat3x3<f32>(
            model_matrix[0].xyz,
            model_matrix[1].xyz,
            model_matrix[2].xyz,
        );

    out.normal = normalize(normal_matrix * v.normal);
    out.uv = v.uv;

    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let light_dir = normalize(vec3<f32>(0.5, 1.0, 0.5));
    let diffuse = max(dot(in.normal, light_dir), 0.15);
    let base_color = vec3<f32>(1.0, 1.0, 1.0);

    return vec4<f32>(base_color * diffuse, 1.0);
}
