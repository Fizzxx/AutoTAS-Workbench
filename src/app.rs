use std::sync::Arc;

use egui_winit::egui;
//use egui_winit::egui::{RichText, Color32};
use egui_winit::winit:: {
    application::ApplicationHandler,
    dpi::PhysicalSize,
    event::WindowEvent,
    event_loop::ActiveEventLoop,
    window::{ Window, WindowId },
};
use egui_wgpu::{ ScreenDescriptor, wgpu };

mod renderstate;
use renderstate::RenderState;

mod eguirenderer;
use eguirenderer::EguiRenderer;


pub struct App {
    title: String,
    window_size: PhysicalSize<u32>,
    scale_factor: f32,
    render_state: Option<RenderState>,
    egui_renderer: Option<EguiRenderer>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            title: String::from("Auto-TAS"),
            window_size: PhysicalSize::new(1920, 1080),
            scale_factor: 1.0,
            render_state: None,
            //egui_context: egui::Context::default(),
            // egui_winit: None,
            egui_renderer: None,
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
            rs.window.as_ref()
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
        //self.render_state.as_mut().unwrap().render()
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
                egui::Window::new("winit + egui + wgpu says hello!")
                    .default_size(egui::vec2(800.0,800.0))
                    .resizable(true)
                    .scroll(egui::Vec2b{x:true, y:true})
                    .default_open(false)
                    .show(ctx, |ui| {
                        ui.label("Label!");
                        if ui.button("button!").clicked() {
                            println!("boom!")
                        }
                        ui.separator();
                        ui.horizontal(|ui| {
                            ui.label(format!(
                                "Pixels per point: {}",
                                ctx.pixels_per_point()
                            ));
                            if ui.button("-").clicked() {
                                self.scale_factor = (self.scale_factor - 0.1).max(0.3);
                            }
                            if ui.button("+").clicked() {
                                self.scale_factor = (self.scale_factor + 0.1).min(3.0);
                            }
                        });
                    });
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
            _ => (),
        }
    }
}
