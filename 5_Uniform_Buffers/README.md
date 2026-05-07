# WGPU Uniform Buffers
A basic implementation of uniform buffers. This looks a lot like implementing a vertex buffer.

We create a variable that gets incremented every frame and use it to spin the example object by storing it in a uniform buffer and passing it into the shader.

To create a uniform, we need to do the following:
    - Create the data
    - Create a buffer marked as uniform and add the data to the buffer
    - Create a bind group layout that describes a container that holds the previous buffers and any other desired buffers.
    - Create a bind group and add the buffers in the locations described by the layout.
    - Add the bind group layout to create_pipeline_layout() -> bind_group_layouts: &[] array
    - Set the bind group in the render pass 
    - Create matching variables in shader with matching @group(n) @binding(m) indices and declared with var<uniform> 

# Program Structure

<pre>
src
 └ main.rs                            - Runs an event loop that handles our input and triggers the render() function
 └ app_environment
    └ mod.rs                          - Holds structures used to setup OS and GPU interfaces 
 └ app_graphics_engine
    └ example_objects_uniform_buf.rs  - Holds structures that define and contain the bind group, bind group layout, and uniform buffer.
    └ example_objects.rs              - Holds structure that contains the vertex buffer, vertex buffer layouts, and info to draw the vertices. 
    └ mod.rs                          - Holds structures and functions for rendering to the screen
resources
 └ shaders.wgsl                       - a few WGSL shaders to run on the above framework
</pre>
