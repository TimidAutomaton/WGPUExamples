use wgpu::{include_wgsl, PipelineCompilationOptions};
pub struct AppGraphicsEngine {
    pipeline: wgpu::RenderPipeline,
}

impl AppGraphicsEngine {
    pub fn new(device: &wgpu::Device, config: &wgpu::SurfaceConfiguration) -> Self {
        let triangle_shader = device.create_shader_module(include_wgsl!("../../resources/triangle_shader.wgsl"));

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
                    module: &triangle_shader, 
                    entry_point: Some("vs_main"), 
                    compilation_options: PipelineCompilationOptions::default(), 
                    buffers: &[],
                }, 
                fragment: Some(wgpu::FragmentState {
                    module: &triangle_shader,
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
                    cull_mode: None, // Some(wgpu::Face::Back),
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
        }
    }

    pub fn render(&mut self, device: &wgpu::Device, queue: &wgpu::Queue, view: &wgpu::TextureView) {
        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });

        // Keep the rpass encapsulated to this scope with curly brackets, so we can use encoder.finish() without borrow checker issues
        { 
            let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.3,
                            g: 0.7,
                            b: 0.9,
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

            // I made the shaders to look a certain way given the following settings.
            //  Put whatever values you want to for the instances, these are just what I set for each shader
            //      The shaders expect a certain amount of vertices, so modifying this will cause issues.

            // triangle_shader: 3 vertices and 3 instances
            // triangle_shader2: 3 vertices and 4 instances
            // dreamcast_logo_shader: 6 vertices and 110 instances
            // flower_spiral: 6 vertices and 110 instances (I messed up the spiral shader and thought it looked cool)

            // The spiral shaders work by creating rectangular strips from two triangles and using them to draw lines around a spiral shape

            rpass.draw(0..3, 0..4);
        }

        // drop(rpass); // We can manually drop the rpass instead of using the curly brackets if we want

        queue.submit(Some(encoder.finish()));
    }

}