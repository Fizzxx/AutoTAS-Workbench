use egui_winit::egui;
use egui_winit::egui::Context;
use egui_winit::winit::{
    event::WindowEvent,
    window::Window,
};

use egui_wgpu::{wgpu, Renderer, ScreenDescriptor};
use egui_wgpu::wgpu::{CommandEncoder, Device, Queue, StoreOp, TextureFormat, TextureView};

pub struct EguiRenderer {
    state: egui_winit::State,
    renderer: egui_wgpu::Renderer,
}

impl EguiRenderer {
    // pub fn context(&self) -> &Context {
    //     self.state.egui_ctx()
    // }

    pub fn new(
        device: &Device,
        output_color_format: TextureFormat,
        output_depth_format: Option<TextureFormat>,
        msaa_samples: u32,
        window: &Window,
    ) -> Self {
        let egui_context = Context::default();
        let mut fonts = egui::FontDefinitions::default();

        fonts.font_data.insert("my_font".to_owned(),
            egui::epaint::text::FontData::from_static(include_bytes!(
                "../../fonts/FiraCodeNerdFont-Regular.ttf")));

        fonts.families.get_mut(&egui::epaint::FontFamily::Proportional)
                      .unwrap()
                      .insert(0, "my_font".to_owned());

        fonts.families.get_mut(&egui::epaint::FontFamily::Monospace)
                      .unwrap()
                      .push("my_font".to_owned());

        egui_context.set_fonts(fonts);

        let egui_state = egui_winit::State::new(
            egui_context,
            egui::viewport::ViewportId::ROOT,
            &window,
            Some(window.scale_factor() as f32),
            None,
            None,
        );
        let egui_renderer = Renderer::new(
            device,
            output_color_format,
            output_depth_format,
            msaa_samples,
            false,
        );

        Self {
            state: egui_state,
            renderer: egui_renderer,
        }
    }

    pub fn handle_input(&mut self, window: &Window, event: &WindowEvent) {
        let _ = self.state.on_window_event(window, event);
    }

    pub fn draw(
        &mut self,
        device: &Device,
        queue: &Queue,
        encoder: &mut CommandEncoder,
        window: &Window,
        window_surface_view: &TextureView,
        screen_descriptor: ScreenDescriptor,
        run_ui: impl FnOnce(&Context),
    ) {
        self.state
            .egui_ctx()
            .set_pixels_per_point(screen_descriptor.pixels_per_point);

        let raw_input = self.state.take_egui_input(window);
        let full_output = self.state.egui_ctx().run(
            raw_input, 
            |_ctx| {
                run_ui(self.state.egui_ctx());
                //run_ui(ctx);  // what's the difference b/w these 2?
            });

        self.state.handle_platform_output(window, full_output.platform_output);

        let tris = self
            .state
            .egui_ctx()
            .tessellate(full_output.shapes, self.state.egui_ctx().pixels_per_point());
        for (id, image_delta) in &full_output.textures_delta.set {
            self.renderer
                .update_texture(device, queue, *id, image_delta);
        }
        self.renderer
            .update_buffers(device, queue, encoder, &tris, &screen_descriptor);
        let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: window_surface_view,
                resolve_target: None,
                ops: wgpu::Operations {
                    //load: egui_wgpu::wgpu::LoadOp::Load,
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: 0.04,
                        g: 0.042,
                        b: 0.04,
                        a: 1.0,
                    }),
                    store: StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            label: Some("egui main render pass"),
            occlusion_query_set: None,
        });
        self.renderer.render(&mut rpass, &tris, &screen_descriptor);
        drop(rpass);
        for x in &full_output.textures_delta.free {
            self.renderer.free_texture(x)
        }
    }
}