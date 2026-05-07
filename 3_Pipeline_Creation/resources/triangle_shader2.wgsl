
// A more complex version of our first example. 
// Our shader is still pretty limited, but we can make it look a little more interesting this way.

const PI: f32 = 3.14159;

const TRIANGLE_SCALE: f32 = 0.2; // Set size of a triangle
const ROTATION_SHIFT: f32 = 2.0 * PI / 7.0; // Amount a triangle rotates per instance

// draw a triangle using the indices by drawing points around a circle
fn get_vertex_pos(v_index: u32, i_index: u32) -> vec3<f32> {
    var rot_offset = ROTATION_SHIFT * f32(i_index); // Rotate the triangle by ROTATION_SHIFT for each instance
    var rotation = (2.0 * PI / 3.0) * f32(v_index); // Create a triangle by rotating 120 degrees around a circle each v_index (repeats after 3 vertices)
    var x = cos(rotation + rot_offset) * TRIANGLE_SCALE;
    var y = sin(rotation + rot_offset) * TRIANGLE_SCALE;
    return vec3<f32>(x, y, 0.0);
}

const CIRCLE_SCALE: f32 = 0.6; // Set the size of the circle triangles are drawn on
const CIRCLE_SHIFT: f32 = 2.0 * PI / 4.0; // Amount a triangle gets shifted along the circle per instance

// Draw each instanced triangle around a circle
fn get_triangle_pos(i_index: u32) -> vec3<f32> {
    var rotation = CIRCLE_SHIFT * f32(i_index);
    var x = cos(rotation) * CIRCLE_SCALE;
    var y = sin(rotation) * CIRCLE_SCALE;
    return vec3<f32>(x, y, 0.0);
}


const VERTEX_COLOR_SHIFT: f32 = 2.0 * PI / 3;
const INSTANCE_COLOR_SHIFT: f32 = 2.0 * PI / 7;

// Set colors for the vertices, I just threw in some trig functions
fn get_vertex_color(v_index: u32, i_index: u32) -> vec3<f32> {
    var r = cos(VERTEX_COLOR_SHIFT * f32(v_index) + INSTANCE_COLOR_SHIFT * f32(i_index));
    var g = sin(VERTEX_COLOR_SHIFT * f32(v_index) + INSTANCE_COLOR_SHIFT * f32(i_index));
    var b = (cos(VERTEX_COLOR_SHIFT * f32(v_index)) + sin(INSTANCE_COLOR_SHIFT * f32(i_index))) / 2.0;
    return vec3<f32>(r, g, b);
}

// Combine our inputs into a structure to help clean things up
struct VertexInput {
    @builtin(vertex_index) v_index: u32,
    @builtin(instance_index) i_index: u32,
}

// Combine our outputs into a structure so we can pass multiple values to the fragment shader.
// @location(n) binds this variable to the channel n of an adjacent shader stage. 
struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) color: vec3<f32>,  // bind to the 0 channel of the fragment shader input. 
};

@vertex
fn vs_main(in: VertexInput) -> VertexOutput {
    var pos = get_vertex_pos(in.v_index, in.i_index) + get_triangle_pos(in.i_index);
    var color = get_vertex_color(in.v_index, in.i_index);
    var out: VertexOutput;
    out.position = vec4<f32>(pos, 1.0); // Add the w value before passing to the next step
    out.color = color; // Color at a vertex position
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(in.color, 1.0); // Color of a fragment interpolated between vertex positions
}
