# WGPU Storage Buffers
A basic implementation of storage buffers. This is the same as uniform buffers.

We keep the uniform buffer from the previous example (it doesn't actually have anything to do with the storage in this example) and add another uniform that contains the size of the screen.

We use the screen size to get the storage buffer index of each pixel in the fragment shader and store the colors in the buffer. We then use a copy buffer to get the information from GPU space to CPU space and store it in a file with an image buffer after converting the colors to 8-bit values.

To create a storage buffer, we need to do the following:
   - Create the data
   - Create a buffer marked as storage and copy destination (also probably copy source) and add the data to the buffer
   - Create a bind group layout that describes a container that holds the previous buffers and any other desired buffers.
   - Create a bind group and add the buffers in the locations described by the layout.
   - Add the bind group layout to create_pipeline_layout() -> bind_group_layouts: &[] array
   - Set the bind group in the render pass 
   - Create matching variables in shader with matching @group(n) @binding(m) indices and declared with var<storage, read_write> 

To read from a storage buffer wre do the following:
   - Create a copy buffer with map read and copy destination flags
   - Copy storage buffer to copy buffer after the draw command, but before submitting the commands to the queue 
   - Get a mapped view of the data from the copy buffer
   - Copy the data from the view to an array or other data structure to use in other parts of the program
   - unmap the copy buffer

# Program Structure

<pre>
src
 └ main.rs                            - Runs an event loop that handles our input and triggers the render() function
 └ app_environment
    └ mod.rs                          - Holds structures used to setup OS and GPU interfaces 
 └ app_graphics_engine
    └ example_objects.rs              - Holds structure that contains vertex data, bind group data, and uniform/storage buffer.
    └ mod.rs                          - Holds structures and functions for rendering to the screen
resources
 └ shaders.wgsl                       - a few WGSL shaders to run on the above framework
</pre>


# Resources
    Stuff about sRGB and color
    Chromitacity: https://jlongster.com/why-chromaticity-shape
    sRGB: https://en.wikipedia.org/wiki/SRGB
    Transfer Functions (Imaging): https://en.wikipedia.org/wiki/Transfer_functions_in_imaging
    Gamma Correction: https://en.wikipedia.org/wiki/Gamma_correction