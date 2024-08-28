use super::egui;


pub struct MonitorPanel {
    width_range: egui::Rangef,
    pub is_expanded: bool,
}

impl Default for MonitorPanel {
    fn default() -> Self {
        Self {
            width_range: egui::Rangef::new(480.0, 960.0),
            is_expanded: true,
        }
    }
}

impl MonitorPanel {
    pub fn show(&mut self, ctx: &egui::Context) {
        let frame = egui::containers::Frame::default()
                                .fill(egui::Color32::from_rgb(20, 20, 20))
                                .inner_margin(egui::Margin{
                                    left: 5.0, right: 0.0, top: 5.0, bottom: 5.0});

        egui::SidePanel::right("monitor_panel")
            .frame(frame)
            .show_separator_line(true)
            .default_width(self.width_range.min)
            .width_range(self.width_range)
            .show_animated(ctx, self.is_expanded, |ui| {
                if ui.button("Collapse >").clicked() {
                    self.is_expanded = false;
                }
                ui.separator();
                ui.horizontal_wrapped(|ui| {
                    ui.label("Lorem ipsum odor amet, consectetuer adipiscing elit. Penatibus at penatibus sit leo efficitur; sodales non massa. Penatibus ligula fringilla quis id dapibus etiam vulputate. Pulvinar sit phasellus dictum maximus potenti risus. Aenean pharetra lacus lacus ornare phasellus faucibus vivamus nascetur. Suscipit gravida mollis fermentum platea tempor sollicitudin sit dictum. Litora donec nascetur dis elementum dictumst sapien sollicitudin. Habitasse semper arcu hac non suscipit massa fermentum sed. Hendrerit semper dictum vulputate platea finibus primis curae sagittis semper. Diam quam elementum rhoncus nisi felis dis cursus. Neque donec ligula ornare interdum fames sapien torquent. Vestibulum nullam erat scelerisque sem tincidunt dolor nostra platea. Hendrerit maximus porttitor in finibus phasellus maecenas diam maximus. Dictumst nullam commodo nisi metus adipiscing vulputate? Dui ut netus justo feugiat consectetur? Varius proin odio accumsan dictumst eleifend hac. Tortor potenti facilisis senectus scelerisque quis natoque. Erat potenti bibendum netus massa sapien eget porta sem. Euismod lobortis facilisi purus convallis etiam cursus curae placerat? Elit suscipit vitae nisl potenti consectetur metus dis. Accumsan potenti eleifend aliquam ligula enim senectus felis ligula. Metus in et elit bibendum; felis at. Rutrum elementum sociosqu libero elit dui ultricies habitasse senectus. Fusce quisque egestas turpis sapien curabitur. Semper aliquam donec scelerisque ante posuere duis. Molestie lobortis purus ac dictumst convallis pulvinar consectetur. Mattis placerat in eu hendrerit magna. Magna dolor aptent velit euismod tincidunt lacus morbi nascetur dictum. Arcu commodo pulvinar vulputate posuere felis feugiat dignissim euismod. Sociosqu montes adipiscing vel magna ex. Dui interdum a; imperdiet cursus himenaeos mauris. Aptent mollis est potenti vehicula magnis aliquet. Mi duis diam gravida lacus faucibus turpis massa habitant. At curae convallis lacinia consequat donec. Dapibus mattis tempus felis sed netus.")
                })
            });
    }
}