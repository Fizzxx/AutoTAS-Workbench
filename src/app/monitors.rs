use std::collections::BTreeSet;

use super::egui;

use auto_tas::Monitor;

mod testmonitor;
use testmonitor::TestMonitor;

pub mod toppanel;


pub struct Monitors {
    monitors: Vec<Box<dyn Monitor>>,
    open: BTreeSet<String>,
}

impl Default for Monitors {
    fn default() -> Self {
        Self::from_monitors(vec![
            Box::<TestMonitor>::default(),
        ])
    }
}

impl Monitors {
    pub fn enable_monitor(&mut self, _name: String) {
        self.open.insert("Test Monitor".to_string());
    }
}

impl Monitors {
    pub fn draw_monitors(&mut self, ctx: &egui::Context) {
        for monitor in self.monitors.iter_mut() {
            let mut open = self.open.contains(monitor.name());
            monitor.as_mut().show(ctx, &mut open);
            if !open { self.open.remove(monitor.name()); }
        }
    }

    pub fn from_monitors(monitors: Vec<Box<dyn Monitor>>) -> Self {
        let mut open = BTreeSet::new();

        open.insert(
            TestMonitor::default()
                .name()
                .to_owned(),
        );

        Self { monitors, open }
    }
}