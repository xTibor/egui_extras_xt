use egui_extras_xt::barcodes::DataMatrixWidget;

use crate::pages::PageImpl;

#[derive(Default)]
pub struct DataMatrixPage {}

impl PageImpl for DataMatrixPage {
    fn ui(&mut self, ui: &mut eframe::egui::Ui) {
        ui.add(DataMatrixWidget::new("TEST"));
    }
}
