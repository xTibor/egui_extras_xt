use eframe::egui::Ui;
use egui_extras_xt::displays::IndicatorButton;

use crate::pages::PageImpl;

pub struct IndicatorButtonPage {
    value: bool,
}

impl Default for IndicatorButtonPage {
    fn default() -> IndicatorButtonPage {
        IndicatorButtonPage { value: false }
    }
}

impl PageImpl for IndicatorButtonPage {
    fn ui(&mut self, ui: &mut Ui) {
        ui.add(IndicatorButton::new(&mut self.value));
        ui.separator();
    }
}
