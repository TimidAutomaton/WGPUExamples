use winit::application::ApplicationHandler;
use winit::dpi::LogicalSize; // used for window size if we use set window attributes
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Window, WindowId};

fn main() {
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);


    // ControlFlow::Wait pauses the event loop if no events are available to process.
    // This is ideal for non-game applications that only update in response to user
    // input, and uses significantly less power/CPU time than ControlFlow::Poll.

    // event_loop.set_control_flow(ControlFlow::Wait);

    let mut app = App::default();
    let _ = event_loop.run_app(&mut app);
}


#[derive(Default)]
struct App {
    window: Option<Window>,
}

impl ApplicationHandler for App {

    // Create a window of a specific size with a specific title
    /* 
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window_size = LogicalSize::new(300, 250);
        self.window = Some(event_loop.create_window(Window::default_attributes()
            .with_inner_size(window_size)
            .with_title(format!("New Window {}x{}", window_size.width, window_size.height))).unwrap())    
    }
    */

    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        self.window = Some(event_loop.create_window(Window::default_attributes()).unwrap())
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _window_id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            },
            WindowEvent::RedrawRequested => {
                self.window.as_ref().unwrap().request_redraw();
            },
            _ => (),
        }

    }
}