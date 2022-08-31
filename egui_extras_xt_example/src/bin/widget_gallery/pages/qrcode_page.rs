use eframe::egui::{DragValue, Grid, Ui};
use eframe::epaint::Color32;
use egui_extras_xt::barcodes::QrCodeWidget;

use crate::pages::PageImpl;

pub struct QrCodePage {
    value: String,
    module_size: usize,
    quiet_zone: usize,
    foreground_color: Color32,
    background_color: Color32,
}

impl Default for QrCodePage {
    fn default() -> QrCodePage {
        QrCodePage {
            value: "egui_extras_xt".to_owned(),
            module_size: 6,
            quiet_zone: 4,
            foreground_color: Color32::BLACK,
            background_color: Color32::WHITE,
        }
    }
}

impl PageImpl for QrCodePage {
    fn ui(&mut self, ui: &mut Ui) {
        ui.add(
            QrCodeWidget::new(&self.value)
                .module_size(self.module_size)
                .quiet_zone(self.quiet_zone)
                .foreground_color(self.foreground_color)
                .background_color(self.background_color),
        );
        ui.separator();

        Grid::new("qrcode_properties")
            .num_columns(2)
            .spacing([20.0, 10.0])
            .striped(true)
            .show(ui, |ui| {
                ui.label("Value");
                ui.text_edit_singleline(&mut self.value);
                ui.end_row();

                ui.label("Module size");
                ui.add(DragValue::new(&mut self.module_size));
                ui.end_row();

                ui.label("Quiet zone");
                ui.add(DragValue::new(&mut self.quiet_zone));
                ui.end_row();

                ui.label("Foreground color");
                ui.color_edit_button_srgba(&mut self.foreground_color);
                ui.end_row();

                ui.label("Background color");
                ui.color_edit_button_srgba(&mut self.background_color);
                ui.end_row();
            });
    }
}
