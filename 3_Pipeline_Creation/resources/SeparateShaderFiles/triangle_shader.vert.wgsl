const PI: f32 = 3.14159;

struct VertexOutput {
    @location(0) color: vec3<f32>,
    @builtin(position) position: vec4<f32>,
};

@vertex
fn vs_main(
    @builtin(vertex_index) v_index: u32,
) -> VertexOutput {
    var out: VertexOutput;
    
    out.color = get_color(v_index);
    out.position = get_position(v_index);
    
    return out;
}

const num_points = 3.0;

fn get_color(v_index: u32) -> vec3<f32> {
    var r = cos(f32(v_index) / num_points * 2.0 * PI);
    var g = cos(f32(v_index) / num_points * 2.0 * PI + (4 * PI / 3));
    var b = cos(f32(v_index) / num_points * 2.0 * PI + (2 * PI / 3));

    return vec3<f32>(r, g, b);
}

fn get_position(v_index: u32) -> vec4<f32> {
    var rotation = PI / 2.0;

    var x = cos(f32(v_index) / num_points * 2.0 * PI - rotation);
    var y = -1.0 * sin(f32(v_index) / num_points * 2.0 * PI - rotation);
    var z = 0.0;
    var w = 1.0;

    return vec4<f32>(x, y, z, w);
}