
use wgpu::{PipelineCompilationOptions, PollType, include_wgsl};

mod example_objects;
use crate::{ExamplePrograms, app_graphics_engine::example_objects::ExampleObject};

pub struct AppGraphicsEngine {
    pipeline: wgpu::RenderPipeline,
    example_object: ExampleObject,
    screen_size: (u32, u32),
    is_image_saved: bool,
}

impl AppGraphicsEngine {
    pub fn new(device: &wgpu::Device, config: &wgpu::SurfaceConfiguration, example_program: &ExamplePrograms) -> Self {
        
        let shaders;
        let example_object;

        match example_program {
            ExamplePrograms::SimpleTriangle => {
                shaders = device.create_shader_module(include_wgsl!("../../resources/4_storage_shader.wgsl"));
                example_object = ExampleObject::create_triangle(config.width, config.height, device);
            },
            // This one is glitchy, since there is no depth involved and the fragments write to the buffer with no predetermined order.
            ExamplePrograms::InstancedTriangleSpiral => {
                shaders = device.create_shader_module(include_wgsl!("../../resources/5_instance_storage_shader.wgsl"));
                example_object = ExampleObject::create_spiral(config.width, config.height, device, 50);
            },
            ExamplePrograms::IndexedVertexBuffers => {
                shaders = device.create_shader_module(include_wgsl!("../../resources/4_storage_shader.wgsl"));
                example_object = ExampleObject::create_indexed_example(config.width, config.height, device);
            },
        }

        let pipeline_layout = device.create_pipeline_layout(
            &wgpu::PipelineLayoutDescriptor {
                label: Some("triangle_pipeline_layout"), 
                bind_group_layouts: &[&example_object.bind_group_layout], 
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
                    buffers: &example_object.layouts,
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
            is_image_saved: false,
            screen_size: (config.width, config.height),
        }
    }

    pub fn render(&mut self, queue: &wgpu::Queue, device: &wgpu::Device, view: &wgpu::TextureView) {
        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });

        
        let background = [0.1, 0.2, 0.3, 1.0];

        let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Render Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: background[0],
                        g: background[1],
                        b: background[2],
                        a: background[3],
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

        rpass.set_bind_group(0, &self.example_object.bind_group, &[]);

        for i in 0..self.example_object.vertex_buffers.len() {
            rpass.set_vertex_buffer(i as u32, self.example_object.vertex_buffers[i].slice(..));
        }
        if self.example_object.index_buffer.is_some(){
            rpass.set_index_buffer(self.example_object.index_buffer.as_ref().unwrap().slice(..), wgpu::IndexFormat::Uint32);
            rpass.draw_indexed(0..self.example_object.num_to_draw, 0, 0..1);
        }
        else {
            rpass.draw(0..self.example_object.num_to_draw, 0..self.example_object.instances);
        }
    
        drop(rpass);

        encoder.copy_buffer_to_buffer(&self.example_object.storage_buffer, 0,
             &self.example_object.copy_buffer, 0, 
             self.example_object.copy_buffer.size());
        
        queue.submit(Some(encoder.finish()));

        self.map_and_copy_buffer(background, device);

    }

    pub fn update(&mut self, queue: &wgpu::Queue) {
        self.example_object.update(queue);
    }

    pub fn map_and_copy_buffer(&mut self, background_color: [f64; 4], device: &wgpu::Device) {
        let (sender, receiver) = futures_channel::oneshot::channel();

        self.example_object.copy_buffer
            .slice(..)
            .map_async(wgpu::MapMode::Read, |result| {
                let _ = sender.send(result);
            });

        let _ = device.poll(PollType::Wait);

        let result = futures::executor::block_on( async {
            receiver.await.expect("communication failed")
                .expect("buffer reading failed");
        });

        // slice of mapped view must go out of scope before data can be unmapped, so brackets are added
        {
            let byte_slice: &[u8] = &self.example_object.copy_buffer.slice(..).get_mapped_range();
            let f32_slice: &[f32] = bytemuck::cast_slice(&byte_slice);

            // do something with the data (save screenshot to png)
            if !self.is_image_saved { // only save to file once
                self.save_to_file(f32_slice, background_color, false, "screen_capture_raw_data.png");
                self.save_to_file(f32_slice, background_color, true, "screen_capture_sRGB.png");
                self.is_image_saved = true;
            }
        }

        self.example_object.copy_buffer.unmap();
    }

    pub fn save_to_file(&self, color_data: &[f32], background_color: [f64; 4], color_correction: bool, file_name: &str) {
        let mut image_buffer = image::ImageBuffer::new(self.screen_size.0, self.screen_size.1);
        
        // debug stuff
        let size = color_data.len();
        let pixels = size / 4;
        let h = pixels / self.screen_size.0 as usize;
        println!("bytes: {} pixels: {} dimensions: {}x{}", size, pixels, self.screen_size.0, h);

        for (x, y, pixel) in image_buffer.enumerate_pixels_mut() {
            let index = ((x + self.screen_size.0 * y) * 4) as usize;

            let mut r =  color_data[index];
            let mut g =  color_data[index + 1];
            let mut b =  color_data[index + 2];
            let mut _a = color_data[index + 3];

            // The shader only draws fragments that are drawn to and initializes to all 0.0 values
            // If the color is black, we assume it's the background (We'd need a better way to handle this if we want to draw black to the screen in the shader)
            if r == 0.0 && g == 0.0 && b == 0.0 {
                r = background_color[0] as f32;
                g = background_color[1] as f32;
                b = background_color[2] as f32;
            }
            if color_correction {
                let c_r = AppGraphicsEngine::to_srgb(r);
                let c_g = AppGraphicsEngine::to_srgb(g);
                let c_b = AppGraphicsEngine::to_srgb(b);

                *pixel = image::Rgb([c_r, c_g, c_b]);
            }
            else {
                let c_r = AppGraphicsEngine::rgb_f32_to_u8(r);
                let c_g = AppGraphicsEngine::rgb_f32_to_u8(g);
                let c_b = AppGraphicsEngine::rgb_f32_to_u8(b);

                *pixel = image::Rgb([c_r, c_g, c_b]);
            }
            
            
        }
        image_buffer.save(file_name.to_string()).unwrap();
    }


    // Converts from sRGB to linear brightness (from srgb wikipedia)
    pub fn srgb_to_linear(val: f32) -> f32 {
        if val <= 0.04045 {
            return val / 12.92
        }
        else {
            return ((val + 0.055) / 1.055).powf(2.4)
        }
    }

    // Converts from linear to sRGB (from srgb wikipedia)
    pub fn linear_to_srgb(val: f32) -> f32 {
        if val <= 0.0031308 {
            return val * 12.92
        }
        else {
            return (val * 1.055).powf(1.0/2.4) - 0.055
        }
    }

    pub fn rgb_f32_to_u8(val: f32) -> u8 {
        (val * 255.0) as u8
    }

    pub fn to_srgb(val: f32) -> u8 {
        AppGraphicsEngine::rgb_f32_to_u8(AppGraphicsEngine::linear_to_srgb(val))
    }
}