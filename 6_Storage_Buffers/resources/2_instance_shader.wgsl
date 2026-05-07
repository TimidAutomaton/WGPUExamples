struct VertexOutput {
    @location(0) color: vec3<f32>,
    @builtin(position) position: vec4<f32>,
};

@vertex
fn vs_main(
    @location(0) position: vec3<f32>,
    @location(1) color: vec3<f32>,
    @location(2) instance_pos: vec2<f32>,
    @location(3) radius: f32,
) -> VertexOutput {
    var out: VertexOutput;
    out.color = color;
    var size = vec3<f32>(radius * 0.3);
    var out_pos = (position + vec3<f32>(instance_pos, 0.0)) * size;
    out.position = vec4<f32>(out_pos, 1.0);
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(in.color, 1.0);
}
