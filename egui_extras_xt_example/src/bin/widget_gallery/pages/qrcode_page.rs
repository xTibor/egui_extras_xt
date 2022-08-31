use egui_extras_xt::barcodes::QrCodeWidget;

use crate::pages::PageImpl;

#[derive(Default)]
pub struct QrCodePage {
    value: String,
}

impl PageImpl for QrCodePage {
    fn ui(&mut self, ui: &mut eframe::egui::Ui) {
        ui.text_edit_singleline(&mut self.value);
        ui.add(QrCodeWidget::new(&self.value));
    }
}
