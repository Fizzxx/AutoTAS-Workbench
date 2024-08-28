use auto_tas::egui;

pub struct CenterPanel {

}

impl Default for CenterPanel {
    fn default() -> Self { Self{} }
}

impl CenterPanel {
    pub fn show(&mut self, ctx: &egui::Context) {
        let frame = egui::containers::Frame::default()
                                        .fill(egui::Color32::from_rgb(0, 0, 0))
                                        .inner_margin(egui::Margin{
                                            left: 5.0, right: 0.0, top: 5.0, bottom: 5.0});

        
        egui::CentralPanel::default()
            .frame(frame)
            .show(ctx, |ui| {
                ui.label(egui::RichText::new("Screen capture goes here")
                                                .font(egui::FontId::proportional(40.0)));
            });
    }
}