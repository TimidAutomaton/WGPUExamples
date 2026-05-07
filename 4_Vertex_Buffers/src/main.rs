
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{WindowId};

mod app_environment;
use crate::app_environment::AppEnvironment;

mod app_graphics_engine;
use crate::app_graphics_engine::*;


enum ExamplePrograms {
    SimpleTriangle,
    InstancedTriangleSpiral,
    IndexedVertexBuffers,
}

fn main() {
    env_logger::init();
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);

    let example_program = ExamplePrograms::SimpleTriangle;

    let mut app = App::new("Vertex Buffer Example".to_string(), (600, 600), example_program);
    let _ = event_loop.run_app(&mut app);
}


struct App {
    window_name: String,
    window_size: (i32, i32),
    environment: Option<AppEnvironment>,
    engine: Option<AppGraphicsEngine>,

    example_program: ExamplePrograms,
}

impl App {
    pub fn new(window_name: String, window_size: (i32, i32), example_program: ExamplePrograms) -> Self {
        Self {
            window_name,
            window_size,
            environment: None,
            engine: None,

            example_program,
        }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        self.environment = Some(AppEnvironment::new(&event_loop, self.window_name.clone(), self.window_size));
        self.engine = Some(AppGraphicsEngine::new(&self.environment.as_ref().unwrap().device, &self.environment.as_ref().unwrap().surface_desc, &self.example_program));
        
        // add queue if using write_buffer() example
        // self.engine = Some(AppGraphicsEngine::new(&self.environment.as_ref().unwrap().device, &self.environment.as_ref().unwrap().surface_desc, &self.environment.as_ref().unwrap().queue));
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _window_id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            },
            WindowEvent::RedrawRequested => {
                self.environment.as_mut().unwrap().window.request_redraw();
                let app_window = self.environment.as_ref().unwrap();

                let frame = match app_window.surface.get_current_texture() {
                    Ok(frame) => frame,
                    Err(e) => {
                        eprintln!("dropped frame: {e:?}");
                        return;
                    }
                }; 

                let view = frame.texture.create_view(&wgpu::TextureViewDescriptor::default());
                self.engine.as_mut().unwrap().render(&app_window.queue, &app_window.device, &view);
                frame.present();
                
            },
            _ => (),
        }

    }
}