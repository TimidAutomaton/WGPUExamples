# WGPU Vertex Buffers
A basic implementation of vertex buffers. Has examples for vertex buffers, instances, and index buffers.

The ExamplePrograms variable "example_program" in main.rs allows you to choose which example gets run. It chooses which ExampleObject function gets created and the appropriate shader.

To create a vertex, we need to do the following:
   - Create the data
   - Create a buffer marked as vertex and add the data to the buffer
   - Create a layout that describes the data
   - Add the vertex layout to create_render_pipeline() -> VertexState -> buffers: &[] array
   - Set the vertex buffer in the render pass
   - Add the variables at the matching @loction(n) index in the vertex shader input

# Program Structure

<pre>
src
 └ main.rs                - Runs an event loop that handles our input and triggers the render() function
 └ app_environment
    └ mod.rs              - Holds structures used to setup OS and GPU interfaces 
 └ app_graphics_engine
    └ example_objects.rs  - Holds structure that contains the vertex buffer, vertex buffer layouts, and info to draw the vertices. 
    └ mod.rs              - Holds structures and functions for rendering to the screen
resources
 └ shaders.wgsl           - a few WGSL shaders to run on the above framework
</pre>
