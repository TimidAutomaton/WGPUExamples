
use std::f32::consts::PI;

use bytemuck::{Pod, Zeroable};
use wgpu::{VertexBufferLayout, util::{BufferInitDescriptor, DeviceExt}};
use wgpu::{BindGroupDescriptor, BindGroupEntry, BindGroupLayoutDescriptor, BindGroupLayoutEntry, ShaderStages};


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
    
    pub frame: f32,
    pub uniform_buffer: wgpu::Buffer,
    pub storage_buffer: wgpu::Buffer,
    pub copy_buffer: wgpu::Buffer,
    pub bind_group: wgpu::BindGroup,
    pub bind_group_layout: wgpu::BindGroupLayout,
}

impl ExampleObject{
    pub fn create_triangle(screen_width: u32, screen_height: u32,  device: &wgpu::Device) -> Self {
        let mut vertex_data = Vec::new();
        vertex_data.push( Vertex {pos: [ 0.0,   0.5, 0.0], color: [1.0, 0.0, 0.0]}); // Top
        vertex_data.push( Vertex {pos: [-0.5,  -0.5, 0.0], color: [0.0, 1.0, 0.0]}); // Bottom Left
        vertex_data.push( Vertex {pos: [ 0.5,  -0.5, 0.0], color: [0.0, 0.0, 1.0]}); // Bottom Right
        
        let vertex_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Vertex Buffer Simple Initialization"),
            contents: bytemuck::cast_slice(&vertex_data),
            usage: wgpu::BufferUsages::VERTEX,
        });

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

        let (uniform_buffer, 
            storage_buffer, 
            copy_buffer,
            bind_group_layout, 
            bind_group
        ) = ExampleObject::create_example_bind_group(screen_width, screen_height, device);

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

            frame: 0.0,
            uniform_buffer,
            storage_buffer,
            copy_buffer,
            bind_group_layout,
            bind_group,
        }
    }


    pub fn create_example_bind_group(screen_width: u32, screen_height: u32, device: &wgpu::Device) -> (
        wgpu::Buffer, // frame buffer from the uniform example
        wgpu::Buffer, // storage buffer to write image data to from the screen
        wgpu::Buffer, // a buffer to hold a copy of the storage buffer that can be used to write the image to a file
        wgpu::BindGroupLayout,
        wgpu::BindGroup) {
        let frame = 0.0;

        let frame_buffer  = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Uniform Buffer"),
            contents: bytemuck::cast_slice(&[frame]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let screen_size_buffer  = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Uniform Buffer"),
            contents: bytemuck::cast_slice(&[screen_width, screen_height]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let num_pixels = screen_width * screen_height * 4;

        let mut storage_buffer_data = Vec::new();
        for _ in 0..num_pixels {
            storage_buffer_data.push(0.0 as f32); 
        }

        let storage_buffer  = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Storage Buffer Example"),
            contents: bytemuck::cast_slice(&storage_buffer_data),
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::COPY_SRC
        });

        let copy_buffer  = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Storage Buffer Example Copy"),
            contents: bytemuck::cast_slice(&storage_buffer_data),
            usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
        });


        let bind_group_layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: Some("Uniform Bind Group Layout"),
            entries: &[
                BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                BindGroupLayoutEntry {
                    binding: 1,
                    visibility: ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                BindGroupLayoutEntry {
                    binding: 2,
                    visibility: ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { 
                            read_only: false 
                        },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }
            ],
        });

        let bind_group = device.create_bind_group(&BindGroupDescriptor {
            label: Some("Uniform Bind Group"),
            layout: &bind_group_layout,
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: frame_buffer.as_entire_binding(),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: screen_size_buffer.as_entire_binding(),
                },
                BindGroupEntry {
                    binding: 2,
                    resource: storage_buffer.as_entire_binding(),
                }
            ],
        });

        (frame_buffer, storage_buffer, copy_buffer, bind_group_layout, bind_group)
    }

    pub fn create_spiral(screen_width: u32, screen_height: u32, device: &wgpu::Device, instances: u32) -> Self {
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

        let (uniform_buffer, 
            storage_buffer, 
            copy_buffer,
            bind_group_layout, 
            bind_group
        ) = ExampleObject::create_example_bind_group(screen_width, screen_height, device);

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

            frame: 0.0,
            uniform_buffer,
            storage_buffer,
            copy_buffer,
            bind_group_layout,
            bind_group,
        }
    }

    pub fn create_indexed_example(screen_width: u32, screen_height: u32, device: &wgpu::Device) -> Self {
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

        let (uniform_buffer, 
            storage_buffer, 
            copy_buffer,
            bind_group_layout, 
            bind_group
        ) = ExampleObject::create_example_bind_group(screen_width, screen_height, device);

        Self {
            vertex_buffers,
            layouts,
            index_buffer,
            num_to_draw: 3,
            instances: 1,

            frame: 0.0,
            uniform_buffer,
            storage_buffer,
            copy_buffer,
            bind_group_layout,
            bind_group,
        }
    }

    pub fn update(&mut self, queue: &wgpu::Queue) {
        self.frame += 1.0;
        queue.write_buffer(&self.uniform_buffer, 0, bytemuck::cast_slice(&[self.frame]));
    }

}

