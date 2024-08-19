use egui_winit::winit::event_loop::{ ControlFlow, EventLoop };

mod app;
use app::App;

#[tokio::main]
async fn main() {
    env_logger::init();
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Wait);

    let mut app: App = App::default();
    let _ = event_loop.run_app(&mut app);
}
