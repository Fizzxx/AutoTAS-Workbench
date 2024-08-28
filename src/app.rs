use std::sync::Arc;

use auto_tas::{ egui, wgpu };
use egui_winit::winit:: {
    application::ApplicationHandler,
    dpi::PhysicalSize,
    event::WindowEvent,
    event_loop::ActiveEventLoop,
    keyboard::{ KeyCode, PhysicalKey::Code },
    window::{ Window, WindowId },
};
use egui_wgpu::ScreenDescriptor;

pub mod eguirenderer;
use eguirenderer::EguiRenderer;

pub mod renderstate;
use renderstate::RenderState;

pub mod screencap;
use screencap::ScreenCap;

pub mod toppanel;
use toppanel::TopPanel;

pub mod monitorpanel;
use monitorpanel::MonitorPanel;

pub mod bottompanel;
use bottompanel::BottomPanel;

pub mod centerpanel;
use centerpanel::CenterPanel;


pub struct App {
    title: String,
    window_size: PhysicalSize<u32>,
    scale_factor: f32,
    egui_renderer: Option<EguiRenderer>,
    render_state: Option<RenderState>,
    screencap: ScreenCap,
    toppanel: TopPanel,
    //monitors: Monitors,
    monitorpanel: MonitorPanel,
    bottompanel: BottomPanel,
    centerpanel: CenterPanel,
    

}

impl Default for App {
    fn default() -> Self {
        Self {
            title: String::from("Auto-TAS"),
            window_size: PhysicalSize::new(1920, 1080),
            scale_factor: 1.0,
            render_state: None,
            egui_renderer: None,
            screencap: ScreenCap::default(),
            toppanel: TopPanel::default(),
            //monitors: Monitors::default(),
            monitorpanel: MonitorPanel::default(),
            bottompanel: BottomPanel::default(),
            centerpanel: CenterPanel::default(),
            
        }
    }
}

impl App {
    fn initialize_eguirenderer(&mut self) {
        let rs = self.render_state.as_ref().unwrap();
        self.egui_renderer = Some(EguiRenderer::new(
            &rs.device,
            rs.surface_configs.format,
            None,
            1,
            rs.window.clone().as_ref()
        ))
    }

    fn initialize_renderstate(&mut self, window: Arc<Window>) {
        use tokio::task;
        use tokio::runtime::Handle;
        self.render_state = Some(task::block_in_place(|| {
            Handle::current().block_on(RenderState::new(window.clone()))
        }));
    }

    // -!- need to update error propogation
    fn redraw(&mut self) -> Result<(), wgpu::SurfaceError> {
        let rs = self.render_state.as_ref().unwrap();

        let surface_texture = rs.surface
                .get_current_texture()
                .expect("Failed to acquire next swap chain texture");

        let surface_view = surface_texture
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder =
            rs.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: None,
            });

        let screen_descriptor = ScreenDescriptor {
            size_in_pixels: [rs.surface_configs.width, rs.surface_configs.height],
            pixels_per_point: rs.window.scale_factor() as f32 * self.scale_factor,
        };

        self.egui_renderer.as_mut().unwrap().draw(
            &rs.device,
            &rs.queue,
            &mut encoder,
            &rs.window,
            &surface_view,
            screen_descriptor,
            |ctx| {
                self.toppanel.show(ctx);
                self.monitorpanel.show(ctx);
                //self.screencap.show(ctx);
                self.bottompanel.show(ctx);
                self.centerpanel.show(ctx);
                //self.monitors.draw_monitors(ctx);
            },
        );
        rs.queue.submit(Some(encoder.finish()));
        surface_texture.present();
        rs.window.request_redraw();
        Ok(())
    }

    fn resize(&mut self, new_size: PhysicalSize<u32>) {
        let rs = self.render_state.as_mut().unwrap();
        let device = &rs.device;
        rs.surface_configs.width = new_size.width;
        rs.surface_configs.height = new_size.height;
        rs.surface.configure(device, &rs.surface_configs);
    }
}

impl ApplicationHandler for App {
    // resumed is called at the beginning of the application
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window_attributes = Window::default_attributes()
            .with_inner_size(self.window_size)
            .with_title(self.title.clone());
        let window = Arc::new(event_loop.create_window(window_attributes).unwrap());

        self.initialize_renderstate(window.clone());

        self.initialize_eguirenderer();

        window.request_redraw();
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        let window = self.render_state.as_ref().unwrap().window.clone();
        self.egui_renderer.as_mut().unwrap().handle_input(window.as_ref(), &event);
        match event {
            WindowEvent::CloseRequested => {
                println!("The close button was pressed; stopping");
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                // self.update();
                match self.redraw() {
                    Ok(_) => {}
                    // Recreate the swap_chain if lost
                    Err(wgpu::SurfaceError::Lost) => self.resize(self.render_state.as_ref().unwrap().size),
                    Err(wgpu::SurfaceError::OutOfMemory) => event_loop.exit(),
                    Err(e) => eprintln!("Some unhandled error {:?}", e),
                }    
            }
            WindowEvent::Resized(physical_size) => {
                self.resize(physical_size);
            }
            WindowEvent::KeyboardInput{event, ..} => {
                if event.state.is_pressed() {
                    match event.physical_key {
                        Code(KeyCode::KeyM) => {
                            self.monitorpanel.is_expanded = !self.monitorpanel.is_expanded;
                        }
                        Code(KeyCode::KeyB) => {
                            self.bottompanel.is_expanded = !self.bottompanel.is_expanded;
                        }
                        _ => {}
                    }
                }
                
                window.request_redraw()
            }
            _ => (),
        }
    }
}
