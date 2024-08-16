mod app;
use app::App;

use winit::event_loop::{ControlFlow, EventLoop};

fn main() {
    env_logger::init();

    let event_loop: EventLoop<()> = EventLoop::new().unwrap();

    event_loop.set_control_flow(ControlFlow::Wait);
    let mut app: App = App::default();
    let _ = event_loop.run_app(&mut app); // [NOTE] `let _ = ` is not necessary
}
