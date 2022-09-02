use eframe::egui::Ui;

use crate::pages::PageImpl;

pub struct WelcomePage {}

impl Default for WelcomePage {
    fn default() -> WelcomePage {
        WelcomePage {}
    }
}

impl PageImpl for WelcomePage {
    fn ui(&mut self, ui: &mut Ui) {
        ui.label("TODO");
    }
}
