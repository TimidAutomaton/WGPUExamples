# WGPU Pipeline Creation
This is the basic implementation needed to run shaders that draw to a window.

Creates a WGPU::RenderPipeline, compiles some shaders, and creates a render function to draw to a surface texture.

If you're here from the accompanying video, AppData was renamed to AppGraphicsEngine, sorry for the confusion!

# Program Structure

<pre>
src
 └ main.rs             - Runs an event loop that handles our input and triggers the render() function
 └ app_environment
    └ mod.rs           - Holds structures used to setup OS and GPU interfaces 
 └ app_graphics_engine
    └ mod.rs           - Holds structures and functions for rendering to the screen
resources
 └ shaders.wgsl        - a few WGSL shaders to run on the above framework
</pre>



# Resources
    Winit: https://docs.rs/winit/latest/winit/
    WebGPU docs: https://www.w3.org/TR/webgpu/
    WGSL docs: https://www.w3.org/TR/WGSL/

    WebGPU for JavaScript guide: https://webgpufundamentals.org/ (The basic knowledge is the same for all WebGPU implementations)

    OpenGL Tutorial: https://learnopengl.com/Getting-started/OpenGL (Graphics programming knowledge is also highly transferrable, but OpenGL and WebGPU do have different implementations of some concepts)
