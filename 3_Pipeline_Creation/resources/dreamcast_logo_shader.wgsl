const PI: f32 = 3.14159;

const LINE_OFFSETS: array<vec2<f32>, 6> = array(
    vec2<f32>( 1.1, -1.0),
    vec2<f32>(-0.1, -1.0),
    vec2<f32>(-0.1,  1.0),
    vec2<f32>( 1.1, -1.0),
    vec2<f32>(-0.1,  1.0),
    vec2<f32>( 1.1,  1.0),
);

const START_OFFSET: u32 = 20;
const SPIRAL_STEP_DISTANCE: f32 = 0.05 * PI;
const SPIRAL_WIDTH: f32 = 0.045;
const LINE_WIDTH: f32 = 0.05;
const START_PHASE: f32 = 0.0;

fn get_spiral_pos(point_index: u32) -> vec3<f32> {
    var t = f32(point_index + START_OFFSET) * SPIRAL_STEP_DISTANCE;
    var x = SPIRAL_WIDTH * t * sin(t - START_PHASE);
    var y = SPIRAL_WIDTH * t * cos(t - START_PHASE);
    var spiral_pos = vec3<f32>(x * 0.8, y, 0.0);
    var noise = (normalize(spiral_pos) * cos(f32(point_index) * 12345.0)) * 0.01;
    return spiral_pos + noise;
}

@vertex
fn vs_main(
    @builtin(vertex_index) v_index: u32,
    @builtin(instance_index) i_index: u32,
) -> @builtin(position) vec4<f32> {
    var p1 = get_spiral_pos(i_index);
    var p2 = get_spiral_pos(i_index + 1);
    var p3 = get_spiral_pos(i_index + 2);
    var line_vec = p2 - p1;
    var right_vec = normalize(cross(vec3<f32>(0.0, 0.0, -1.0), line_vec));
    var offsets = LINE_OFFSETS[v_index];
    return vec4<f32>(p1 + line_vec * offsets[0] + LINE_WIDTH * right_vec * offsets[1], 1.0);
}


@fragment
fn fs_main() -> @location(0) vec4<f32> {
    //return vec4<f32>(0.5, 0.5, 0.5, 1.0);
    return vec4<f32>(0.921, 0.521, 0.2, 1.0);
}