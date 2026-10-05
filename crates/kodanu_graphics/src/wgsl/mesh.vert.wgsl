struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
}

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) normal: vec3<f32>,
    @location(1) uv: vec2<f32>,
}

struct Transform {
    model: mat4x4<f32>,
    normal: mat4x4<f32>,
}

struct Camera {
    view_proj: mat4x4<f32>,
}

@group(2) @binding(0)
var<uniform> transform: Transform;

@group(0) @binding(0)
var<uniform> camera: Camera;

@vertex
fn main(input: VertexInput) -> VertexOutput {
    var output: VertexOutput;

    output.position = camera.view_proj * transform.model * vec4<f32>(input.position, 1.0);
    output.normal = (transform.normal * vec4<f32>(input.normal, 0.0)).xyz;
    output.uv = input.uv;

    return output;
}
