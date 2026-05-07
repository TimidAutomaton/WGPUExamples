
use wgpu::{include_wgsl, PipelineCompilationOptions};

mod example_objects;
use crate::{ExamplePrograms, app_graphics_engine::example_objects::ExampleObject};

pub struct AppGraphicsEngine {
    pipeline: wgpu::RenderPipeline,
    example_object: ExampleObject,
}

impl AppGraphicsEngine {
    pub fn new(device: &wgpu::Device, config: &wgpu::SurfaceConfiguration, example_program: &ExamplePrograms) -> Self {
    // pub fn new(device: &wgpu::Device, config: &wgpu::SurfaceConfiguration, queue: &wgpu::Queue) -> Self { // add queue if you use write_buffer in create_triangle

        let shaders;
        let example_object;

        match example_program {
            ExamplePrograms::SimpleTriangle => {
                shaders = device.create_shader_module(include_wgsl!("../../resources/1_v_buf_shader.wgsl"));
                example_object = ExampleObject::create_triangle(device);
            },
            ExamplePrograms::InstancedTriangleSpiral => {
                shaders = device.create_shader_module(include_wgsl!("../../resources/2_instance_shader.wgsl"));
                example_object = ExampleObject::create_spiral(device, 50);
            },
            ExamplePrograms::IndexedVertexBuffers => {
                shaders = device.create_shader_module(include_wgsl!("../../resources/1_v_buf_shader.wgsl"));
                example_object = ExampleObject::create_indexed_example(device);
            },
        }


        let pipeline_layout = device.create_pipeline_layout(
            &wgpu::PipelineLayoutDescriptor {
                label: Some("triangle_pipeline_layout"), 
                bind_group_layouts: &[], 
                push_constant_ranges: &[], 
        });
        let pipeline = device.create_render_pipeline(
            &wgpu::RenderPipelineDescriptor { 
                label: Some("triangle_render_pipeline"), 
                layout: Some(&pipeline_layout), 
                vertex: wgpu::VertexState { 
                    module: &shaders, 
                    entry_point: Some("vs_main"), 
                    compilation_options: PipelineCompilationOptions::default(), 
                    buffers: &example_object.layouts, // Add vertex buffer layouts here
                }, 
                fragment: Some(wgpu::FragmentState {
                    module: &shaders,
                    entry_point: Some("fs_main"),
                    compilation_options: PipelineCompilationOptions::default(), 
                    targets: &[Some(wgpu::ColorTargetState {
                        format: config.format,
                        blend: Some(wgpu::BlendState::REPLACE),
                        write_mask: wgpu::ColorWrites::ALL,
                    })],
                }), 
                primitive: wgpu::PrimitiveState {
                    topology: wgpu::PrimitiveTopology::TriangleList,
                    strip_index_format: None,
                    front_face: wgpu::FrontFace::Ccw,
                    cull_mode: Some(wgpu::Face::Back),
                    polygon_mode: wgpu::PolygonMode::Fill,
                    unclipped_depth: false,
                    conservative: false,
                },
                depth_stencil: None, 
                multisample: wgpu::MultisampleState {
                    count: 1,
                    mask: !0,
                    alpha_to_coverage_enabled: false,
                },
                
                multiview: None, 
                cache: None, 
            });

        Self {
            pipeline,
            example_object,
        }
    }

    pub fn render(&mut self, queue: &wgpu::Queue, device: &wgpu::Device, view: &wgpu::TextureView) {
        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });

        
        let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Render Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: 0.1,
                        g: 0.2,
                        b: 0.3,
                        a: 1.0,
                    }),
                    store: wgpu::StoreOp::Store,
                },
                depth_slice: None,
            })],
            depth_stencil_attachment: None,
            occlusion_query_set: None,
            timestamp_writes: None,
        });

        rpass.set_pipeline(&self.pipeline);

        // Loop through each vertex buffer and set them in the render pipeline
        for i in 0..self.example_object.vertex_buffers.len() {
            rpass.set_vertex_buffer(i as u32, self.example_object.vertex_buffers[i].slice(..));
        }

        // If we have an index buffer, draw using indexing, if we don't, draw using vertices
        if self.example_object.index_buffer.is_some(){
            rpass.set_index_buffer(self.example_object.index_buffer.as_ref().unwrap().slice(..), wgpu::IndexFormat::Uint32);
            rpass.draw_indexed(0..self.example_object.num_to_draw, 0, 0..1);
        }
        else {
            rpass.draw(0..self.example_object.num_to_draw, 0..self.example_object.instances);
        }
    
        drop(rpass);

        queue.submit(Some(encoder.finish()));
    }
}