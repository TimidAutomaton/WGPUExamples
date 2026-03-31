const PI: f32 = 3.14159;
const ARC: f32 = 2.0 * PI / 3.0;           // Constant for how far to move instanced triangle along circle

@vertex
fn vs_main(
    @builtin(vertex_index) v_index: u32,   // Vertex Shader input: Current Vertex Index number
    @builtin(instance_index) i_index: u32, // Vertex Shader input: Current Instance Index number
) -> @builtin(position) vec4<f32> {        // Vertex Shader Output: Final position of Vertex
    var x = (f32(v_index) - 1.0) * 0.5;    // quick mapping of 0, 1, 2 -> -0.5, 0.0, 0.5
    var y = ((f32(v_index) % 2.0) - 0.5);  // maps 0, 1, 2 -> -0.5, 0.5, -0.5
    var pos = vec3<f32>(x + cos(f32(i_index)*ARC), y + sin(f32(i_index)*ARC), 0.0); // Move triangle with each Instance Index
    return vec4<f32>(pos, 1.0); // Output position, (x, y, z, w)
}

@fragment
fn fs_main(
                               // Fragment Shader Inputs: (Empty)
) -> @location(0) vec4<f32> {  // Fragment Shader Output: @location(0) represents framebuffer input
    return vec4<f32>(1.0, 0.0, 0.0, 1.0); // Output Color (r, g, b, a)
}
