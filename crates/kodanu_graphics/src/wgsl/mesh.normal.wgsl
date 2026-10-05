struct FragmentInput {
    @location(0) normal: vec3<f32>,
    @location(1) uv: vec2<f32>,
}

@fragment
fn main(input: FragmentInput) -> @location(0) vec4<f32> {
    let normal = normalize(input.normal);
    let color = normal * 0.5 + 0.5;

    return vec4(color, 1.0);
}
