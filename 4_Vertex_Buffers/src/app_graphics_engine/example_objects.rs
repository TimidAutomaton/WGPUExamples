
use std::f32::consts::PI;

use bytemuck::{Pod, Zeroable};
use wgpu::{BufferDescriptor, VertexBufferLayout, util::{BufferInitDescriptor, DeviceExt}, wgc::device::queue};


#[repr(C)]
#[derive(Copy, Clone, Zeroable, Pod)]
struct Vertex {
    pos: [f32; 3],
    color: [f32; 3],
}

pub struct ExampleObject {
    pub vertex_buffers: Vec<wgpu::Buffer>, 
    pub layouts: Vec<VertexBufferLayout<'static>>,
    pub index_buffer: Option<wgpu::Buffer>,
    pub num_to_draw: u32, // number of vertices or indices to pass to draw()
    pub instances: u32,
}

impl ExampleObject {
    pub fn create_triangle(device: &wgpu::Device) -> Self {
    // pub fn create_triangle(device: &wgpu::Device, queue: &wgpu::Queue) -> Self { // queue is needed as an argument if you write to the buffer
        let mut vertex_data = Vec::new();
        vertex_data.push( Vertex {pos: [ 0.0,   0.5, 0.0], color: [1.0, 0.0, 0.0]}); // Top
        vertex_data.push( Vertex {pos: [-0.5,  -0.5, 0.0], color: [0.0, 1.0, 0.0]}); // Bottom Left
        vertex_data.push( Vertex {pos: [ 0.5,  -0.5, 0.0], color: [0.0, 0.0, 1.0]}); // Bottom Right


        // ------------------------------------ Manual Buffer Initialization ------------------------------------
        // Manual buffer initialization
        /* 
        let vertex_data_bytes = bytemuck::cast_slice(&vertex_data);

        let vertex_buffer = device.create_buffer(&BufferDescriptor {
            label: Some("Vertex Buffer Manual Initialization"),
            size: (vertex_data_bytes.len()) as u64,
            usage: wgpu::BufferUsages::VERTEX,
            mapped_at_creation: true,
        });


        // Mapping the buffer means to map the memory addresses to the CPU space (make the buffers memory range available to CPU operations)
        //  The buffer can be mapped to the CPU, allowing us to read and write in this program, or unmapped, freeing it for GPU programs 
        //  The goal is to prevent both the CPU and GPU from altering the data at the same time, which would cause a race condition (this means the 
        //      result is determined by whichever system was executed first)

        let mut v_buf_view = vertex_buffer.slice(..).get_mapped_range_mut();
        v_buf_view.copy_from_slice(vertex_data_bytes);
        drop(v_buf_view);
        vertex_buffer.unmap();
        */

        // ------------------------------------ Simple Buffer Initialization -------------------------------------
        // Simple buffer creation and initialization (does all of the above for you)
         
        let vertex_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Vertex Buffer Simple Initialization"),
            contents: bytemuck::cast_slice(&vertex_data),
            usage: wgpu::BufferUsages::VERTEX,
        });
        


        // ---------------------------------------- Update Buffer Example ----------------------------------------
        // Creating an empty, uninitialized buffer and writing the data to it later (write_buffer can happen at any time in the program) 
        /* 
        let vertex_data_raw = bytemuck::cast_slice(&vertex_data);

        let vertex_buffer = device.create_buffer(&BufferDescriptor {
            label: Some("Vertex Buffer Empty Initialization"),
            size: (vertex_data_raw.len()) as u64,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        queue.write_buffer(&vertex_buffer, 0, vertex_data_raw);
        */





        let layout = wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress, 
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x3,
                }
            ]
        };

        let mut vertex_buffers = Vec::new();
        vertex_buffers.push(vertex_buffer);

        let mut layouts = Vec::new();
        layouts.push(layout);

        Self { 
            vertex_buffers,
            layouts,
            index_buffer: None,
            num_to_draw: 3,
            instances: 1,
        }
    }


    // ----------------------------------- Create a spiral of triangles -----------------------------------
    // Instance buffer example

    pub fn create_spiral(device: &wgpu::Device, instances: u32) -> Self {
        let mut vertex_data = Vec::new();
        vertex_data.push( Vertex {pos: [ 0.0,   0.5, 0.0], color: [1.0, 0.0, 0.0]}); // Top
        vertex_data.push( Vertex {pos: [-0.5,  -0.5, 0.0], color: [0.0, 1.0, 0.0]}); // Bottom Left
        vertex_data.push( Vertex {pos: [ 0.5,  -0.5, 0.0], color: [0.0, 0.0, 1.0]}); // Bottom Right


        let vertex_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Spiral Vertex Buffer"),
            contents: bytemuck::cast_slice(&vertex_data),
            usage: wgpu::BufferUsages::VERTEX,
        });


        let vertex_layout = wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress, 
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x3,
                }
            ]
        };

        
        // ------------------------ Instance Buffer ------------------------

        let mut instance_data = Vec::new();
        let radius_step = 0.05;
        let points_per_rotation = 15.0;
        let spiral_angle_step = 2.0 * PI / points_per_rotation;
        for i in 0..instances {
            let r = radius_step * i as f32;
            let x = r * (spiral_angle_step * i as f32).cos();
            let y = r * (spiral_angle_step * i as f32).sin();
            instance_data.push(x);
            instance_data.push(y);
            instance_data.push(r);
        }

        let instance_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Spiral Instance Buffer"),
            contents: bytemuck::cast_slice(&instance_data),
            usage: wgpu::BufferUsages::VERTEX,
        });


        let instance_layout = wgpu::VertexBufferLayout {
            array_stride: (std::mem::size_of::<f32>() * 3) as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 2,
                    format: wgpu::VertexFormat::Float32x2,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<f32>() as u64 * 2,
                    shader_location: 3,
                    format: wgpu::VertexFormat::Float32,
                },
            ]
        };

        let mut vertex_buffers = Vec::new();
        vertex_buffers.push(vertex_buffer);
        vertex_buffers.push(instance_buffer);

        let mut layouts = Vec::new();
        layouts.push(vertex_layout);
        layouts.push(instance_layout);

        Self { 
            vertex_buffers,
            layouts,
            index_buffer: None,
            num_to_draw: 3, // each object drawn is a triangle made of three vertices
            instances,
        }
    }

    pub fn create_indexed_example(device: &wgpu::Device) -> Self {
        let mut vertex_data = Vec::new();

        let points = [
            [-1.0,  1.0, 0.0], // 0
            [ 0.0,  1.0, 0.0], // 1
            [-0.5,  0.5, 0.0], // 2
            [-1.0,  0.0, 0.0], // 3
            [ 0.0,  0.5, 0.0], // 4
            [-0.5,  0.0, 0.0], // 5
            [ 0.0,  0.0, 0.0], // 6
            [ 0.5,  0.0, 0.0], // 7
            [ 0.0, -0.5, 0.0], // 8
            [ 1.0,  0.0, 0.0], // 9
            [ 0.5, -0.5, 0.0], // 10
            [ 0.0, -1.0, 0.0], // 11
            [ 1.0, -1.0, 0.0], // 12
        ];

        let color = [
            [0.1, 0.0, 0.0], // 0
            [0.1, 0.0, 0.0], // 1
            [1.0, 0.0, 0.0], // 2
            [0.1, 0.0, 0.0], // 3
            [1.0, 0.0, 0.0], // 4
            [1.0, 0.0, 0.0], // 5
            [1.0, 0.0, 0.0], // 6
            [1.0, 0.0, 0.0], // 7
            [1.0, 0.0, 0.0], // 8
            [0.1, 0.0, 0.0], // 9
            [1.0, 0.0, 0.0], // 10
            [0.1, 0.0, 0.0], // 11
            [0.1, 0.0, 0.0], // 12
        ];

        let indices: [[u32; 3]; 18] = [
            [0, 3, 2], // 0
            [0, 2, 1], // 1
            [2, 3, 5], // 2
            [2, 5, 4], // 3
            [1, 2, 4], // 4
            [5, 3, 8], // 5
            [4, 5, 6], // 6
            [1, 4, 7], // 7
            [8, 3, 11], // 8
            [6, 5, 8], // 9
            [4, 6, 7], // 10
            [1, 7, 9], // 11
            [6, 8, 7], // 12
            [8, 11, 10], // 13
            [7, 8, 10], // 14
            [7, 10, 9], // 15
            [10, 11, 12], // 16
            [9, 10, 12], // 17
        ];

        for i in 0..points.len() {
            vertex_data.push( Vertex {pos: points[i], color: color[i]});
        }

        let mut index_vec = Vec::new();
        for i in 0..indices.len() {
            for k in 0..3 {
                index_vec.push(indices[i][k]);
            }
        }

        let vertex_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Indexed Example Vertex Buffer"),
            contents: bytemuck::cast_slice(&vertex_data),
            usage: wgpu::BufferUsages::VERTEX,
        });


        let vertex_layout = wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress, 
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x3,
                }
            ]
        };

        let index_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(&indices),
            usage: wgpu::BufferUsages::INDEX,
        });

        let mut vertex_buffers = Vec::new();
        vertex_buffers.push(vertex_buffer);

        let mut layouts = Vec::new();
        layouts.push(vertex_layout);

        let index_buffer = Some(index_buffer);

        Self {
            vertex_buffers,
            layouts,
            index_buffer,
            num_to_draw: index_vec.len() as u32,
            instances: 1,
        }
    }

}

