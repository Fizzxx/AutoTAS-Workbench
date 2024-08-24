use super::egui;


pub struct TopPanel {
    // name: String,
    is_expanded: bool,
}

impl Default for TopPanel {
    fn default() -> Self {
        Self { 
            // name: "Top Panel".to_string(),
            is_expanded: true,
        }
    }
}

impl auto_tas::View for TopPanel {
    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.horizontal_centered(|ui| {
            egui::menu::bar(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.separator();
                    ui.menu_button("File", Self::file_menu);
                });
            });
        });
    }
}

impl TopPanel {
    pub fn show(&mut self, ctx: &egui::Context) {
        let frame = egui::containers::Frame::default()
                                .fill(egui::Color32::from_rgb(20, 20, 20));
        
        use auto_tas::View;
        egui::TopBottomPanel::top("my_panel")
            .frame(frame)
            .exact_height(25.0)
            .show_animated(
                ctx, 
                self.is_expanded, 
                |ui| self.ui(ui)
            );
    }

    fn file_menu(ui: &mut egui::Ui) {
        ui.menu_button("New", |ui| {
            if ui.button("Monitor").clicked() {};
            if ui.button("Session").clicked() {};
        });
        ui.separator();
        if ui.button("Open Session...").clicked() {}
        ui.menu_button("Open Recent", |ui| {
            if ui.button("Prev Session 1").clicked() {};
            if ui.button("Prev Session 2").clicked() {};
        });
        ui.separator();
        if ui.button("Save Session").clicked() {}
        if ui.button("Save Session As...").clicked() {}
    }
}

// impl Monitor for TopPanel {
//     fn name(&self) -> &'static str {
//         "Top Panel"
//     }

//     fn show(&mut self, ctx: &egui::Context) {
//         let frame = egui::containers::Frame::default();
        
//         egui::TopBottomPanel::top("my_panel")
//             .frame(frame)
//             .exact_height(25.0)
//             .show(ctx, |ui| {
//                 ui.horizontal(|ui| {
//                     ui.label("Hello World!");
//                     egui::ComboBox::from_label("File")
//                         .show_ui(ui, |ui| {
//                             // ui.selectable_value(current_value, selected_value, text)
//                         });
//                 })
//             });
//     }
// }

