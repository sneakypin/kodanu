struct Material {
    color: vec4<f32>,
}

struct Light {
    direction: vec3<f32>,
    intensity: f32,
    color: vec4<f32>,
}

struct FragmentInput {
    @location(0) normal: vec3<f32>,
    @location(1) uv: vec2<f32>,
}

@group(1) @binding(0)
var<uniform> material: Material;

@group(3) @binding(0)
var<uniform> light: Light;

@fragment
fn main(input: FragmentInput) -> @location(0) vec4<f32> {
    let normal = normalize(input.normal);
    let direction = normalize(-light.direction);

    let diffuse = max(dot(normal, direction), 0.0);
    let lighting = 0.1 + diffuse * light.intensity;

    return vec4(material.color.rgb * light.color.rgb * lighting, material.color.w);
}
