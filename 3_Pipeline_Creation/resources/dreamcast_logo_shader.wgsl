
/*
The main program gets a forward vector from two points on the spiral and then gets a right hand point vector
    using the cross between a vector going into the screen (0, 0, -1) and the forward vector (p2 - p1). 
    z_vec X forward_vec = right_vec

    p1 -----> p2
    |
    |
    v

    The LINE_OFFSETS array below multiplies these vectors to create a rectangle from the lines. The .1 stretches the 
    line a little forward and backward to try to close the gaps a bit.

    Below is an attempt to draw the resulting line. I don't know if it will actually show up correctly.


    f_v = forward_vector
    r_v = right_vector

    (-0.1 * f_v, -1.0 * r_v * LINE_WIDTH)

    |                (1.1 * f_v, -1.0 * r_v * LINE_WIDTH)
    |
    |                 |


    v1               v0/v3
    * -----------------* 
    |                  |
    |   p1 -----> p2   |
    |   |              |
    |   v              |
    *------------------*
  v2/v4                v5


    |                  |
    |                 
    |                  (1.1 * f_v, 1.0 * r_v * LINE_WIDTH)
    
    (-1.0 * f_v, 1.0 * r_v * LINE_WIDTH)
*/

const PI: f32 = 3.14159;

const LINE_OFFSETS: array<vec2<f32>, 6> = array(
    vec2<f32>( 1.1, -1.0), // v0 
    vec2<f32>(-0.1, -1.0), // v1
    vec2<f32>(-0.1,  1.0), // v2
    vec2<f32>( 1.1, -1.0), // v3
    vec2<f32>(-0.1,  1.0), // v4
    vec2<f32>( 1.1,  1.0), // v5
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
    var noise = (normalize(spiral_pos) * cos(f32(point_index) * 12345.0)) * 0.01; // high frequency cos function to add a little bit of wiggle
    return spiral_pos + noise;
}

@vertex
fn vs_main(
    @builtin(vertex_index) v_index: u32,
    @builtin(instance_index) i_index: u32,
) -> @builtin(position) vec4<f32> {
    var p1 = get_spiral_pos(i_index);
    var p2 = get_spiral_pos(i_index + 1);
    var line_vec = p2 - p1;
    var right_vec = normalize(cross(vec3<f32>(0.0, 0.0, -1.0), line_vec));
    var offsets = LINE_OFFSETS[v_index];
    return vec4<f32>(p1 + line_vec * offsets[0] + LINE_WIDTH * right_vec * offsets[1], 1.0);
}


@fragment
fn fs_main() -> @location(0) vec4<f32> {
    return vec4<f32>(0.921, 0.521, 0.2, 1.0);
}