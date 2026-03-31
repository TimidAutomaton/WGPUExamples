# WGPU Instance Creation
This is a bare bones implementation of a WGPU instance and the structures we need to create structures, send messages to the GPU, and draw to a draw surface.

It opens a blank window, because there is no render() function or WGPU::RenderPipeline.

The WGPU structure hierarchy looks something like this:

<pre>
WGPU_Instance - Main structure that creates everything else (Not needed after initialization)
 └ Surface    - Draw surface created from an existing Winit::Window  
 └ Adapter    - Path to the GPU that lets us set our initial options and create other structures (Not needed after initialization)
    └ Device  - Creates data structures (Buffers, Layouts, etc) that we use to send data to the GPU
    └ Queue   - Message bus that we send data structures to the GPU with and use to update buffers as needed
</pre>

# Resources:
    Winit: https://docs.rs/winit/latest/winit/
    WebGPU docs: https://www.w3.org/TR/webgpu/
    WGSL docs: https://www.w3.org/TR/WGSL/

    WebGPU for JavaScript guide: https://webgpufundamentals.org/ (The basic knowledge is the same for all WebGPU implementations)

    OpenGL Tutorial: https://learnopengl.com/Getting-started/OpenGL (Graphics programming knowledge is also highly transferrable, but OpenGL and WebGPU do have different implementations of some concepts)
