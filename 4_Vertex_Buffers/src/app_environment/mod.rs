use std::sync::Arc;

use pollster::block_on;
use winit::{dpi::LogicalSize, event_loop::ActiveEventLoop, window::Window};


pub struct AppEnvironment {
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub window: Arc<Window>,
    pub surface_desc: wgpu::SurfaceConfiguration,
    pub surface: wgpu::Surface<'static>,
}


impl AppEnvironment {
    pub fn new(event_loop: &ActiveEventLoop, window_name: String, window_size: (i32, i32)) -> Self {
        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
            backends: wgpu::Backends::PRIMARY,
            ..Default::default()
        });

        let window_size = LogicalSize::new(window_size.0, window_size.1);
        let window = {
            let attributes = Window::default_attributes()
                .with_inner_size(window_size)
                .with_title(format!("{} {}x{}", window_name, window_size.width, window_size.height));  
            Arc::new(event_loop.create_window(attributes).unwrap())
        };

        let size = window.inner_size();
        let surface = instance.create_surface(window.clone()).unwrap();

        let adapter = block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        }))
        .unwrap();

        let (device, queue) =
            block_on(adapter.request_device(&wgpu::DeviceDescriptor::default())).unwrap();

        let surface_desc = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: wgpu::TextureFormat::Bgra8UnormSrgb,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
            desired_maximum_frame_latency: 2,
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
            view_formats: vec![wgpu::TextureFormat::Bgra8Unorm],
        };

        surface.configure(&device, &surface_desc);

        Self {
            device,
            queue,
            window,
            surface_desc,
            surface,
        }
    }
}
