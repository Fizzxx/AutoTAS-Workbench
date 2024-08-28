use super::egui;

pub struct ScreenCap {}

impl Default for ScreenCap {
    fn default() -> Self { Self{} }
}

impl ScreenCap {
    pub fn show(&mut self, ctx: &egui::Context) {
        let frame = egui::Frame::none()
                            .fill(egui::Color32::from_rgb(0, 0, 0))
                            .shadow(egui::Shadow::NONE);

        egui::Window::new("screencap")
            .default_size(egui::Vec2::new(960.0, 540.0))
            .frame(frame)
            .title_bar(false)
            .anchor(egui::Align2::LEFT_TOP, egui::Vec2::new(30.0, 55.0))
            .collapsible(false)
            .interactable(false)
            .resizable(true)
            .show(ctx, |ui| {
                // ui.add(egui::Image::new(
                //     egui::include_image!("../../EXPLORE_ixfx.png")
                // ));
                ui.image("../../EXPLORE_ixfx.png");
            });
    }
}