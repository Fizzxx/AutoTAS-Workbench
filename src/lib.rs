pub use egui_winit::egui;
pub use egui_wgpu::wgpu;


pub trait View {
    fn ui(&mut self, ui: &mut egui::Ui);
}

pub trait Monitor {
    fn is_enabled(&self, _ctx: &egui::Context) -> bool {
        true // could this be determined from the context?
    }

    fn name(&self) -> &'static str;

    fn show(&mut self, ctx: &egui::Context, open: &mut bool);
}