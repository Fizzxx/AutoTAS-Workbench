use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;
use winit::window::{ Window, WindowId };

mod windowstate;
use windowstate::WindowState;

#[derive(Default)]
pub struct App<'a> {
    window: Option<Window>,
    state: Option<WindowState<'a>>,
}

impl<'a> App<'a> {
    fn window(&self) -> &Window {
        self.window.as_ref().expect("Window is not initialized")
    }
}

impl<'a> ApplicationHandler for App<'a> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        self.window = Some(
            event_loop
                .create_window(Window::default_attributes().with_title("Auto-TAS"))
                .unwrap(),
        );
        // if self.state.is_none() {
        //     let window = self.window.as_ref().unwrap();
        //     let new_state = tokio::runtime::Handle::current().block_on(WindowState::new(window));
        //     self.state = Some(new_state);
        //     println!("state built")
        // }
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                println!("The close button was pressed; stopping");
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                self.window.as_ref().unwrap().request_redraw();
            }
            _ => (),
        }
    }
}
