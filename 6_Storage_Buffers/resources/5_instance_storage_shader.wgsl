struct VertexOutput {
    @location(0) color: vec3<f32>,
    @builtin(position) position: vec4<f32>,
};


@group(0) @binding(0) var<uniform> frame: f32;

fn rotate2D(deg: f32) -> mat2x2<f32> {
    return mat2x2<f32>(cos(deg), sin(deg), -1.0 * sin(deg), cos(deg));
}

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
    out.position = vec4<f32>(out.position.xy * rotate2D(frame * 0.01), out.position.zw);
    return out;
}

@group(0) @binding(1) var<uniform> screen_size: vec2<u32>;
@group(0) @binding(2) var<storage, read_write> example_buf: array<f32>;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let index = (u32(in.position.x) + u32(in.position.y) * screen_size[0]) * 4;
    example_buf[index] = in.color.r;
    example_buf[index + 1] = in.color.g;
    example_buf[index + 2] = in.color.b;
    example_buf[index + 3] = 1.0;
    return vec4<f32>(in.color, 1.0);
}
