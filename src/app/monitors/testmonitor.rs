use super::egui;

use super::Monitor;


pub struct TestMonitor {}

impl Default for TestMonitor {
    fn default() -> Self {
        Self {}
    }
}

impl Monitor for TestMonitor {
    fn name(&self) -> &'static str {
        "Test Monitor"
    }

    fn show(&mut self, ctx: &egui::Context, open: &mut bool) {
        egui::Window::new(self.name())
            .open(open)
            .default_size(egui::vec2(400.0,400.0))
            .resizable(true)
            .scroll(egui::Vec2b{x:true, y:true})
            .default_open(true)
            .collapsible(false)
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
                });
            });
    }
}