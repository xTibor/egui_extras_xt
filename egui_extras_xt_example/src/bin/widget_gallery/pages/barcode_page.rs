use eframe::egui::{DragValue, Grid, Ui};
use eframe::epaint::Color32;
use egui_extras_xt::barcodes::{BarcodeKind, BarcodeWidget};
use egui_extras_xt::ui::widgets_from::WidgetsFromIterator;
use strum::IntoEnumIterator;

use crate::pages::PageImpl;

pub struct BarcodePage {
    value: String,
    barcode_kind: BarcodeKind,
    bar_width: usize,
    bar_height: f32,
    horizontal_padding: f32,
    vertical_padding: f32,
    label: String,
    label_height: f32,
    label_top_margin: f32,
    foreground_color: Color32,
    background_color: Color32,
}

impl Default for BarcodePage {
    fn default() -> BarcodePage {
        BarcodePage {
            value: "01189998819991197253".to_owned(),
            barcode_kind: BarcodeKind::Code39,
            bar_width: 2,
            bar_height: 64.0,
            horizontal_padding: 50.0,
            vertical_padding: 10.0,
            label: "egui_extras_xt".to_owned(),
            label_height: 20.0,
            label_top_margin: 4.0,
            foreground_color: Color32::BLACK,
            background_color: Color32::WHITE,
        }
    }
}

impl PageImpl for BarcodePage {
    fn ui(&mut self, ui: &mut Ui) {
        ui.add(
            BarcodeWidget::new(&self.value)
                .barcode_kind(self.barcode_kind)
                .bar_width(self.bar_width)
                .bar_height(self.bar_height)
                .horizontal_padding(self.horizontal_padding)
                .vertical_padding(self.vertical_padding)
                .label(&self.label)
                .label_height(self.label_height)
                .label_top_margin(self.label_top_margin)
                .foreground_color(self.foreground_color)
                .background_color(self.background_color),
        );
        ui.separator();

        Grid::new("barcode_properties")
            .num_columns(2)
            .spacing([20.0, 10.0])
            .striped(true)
            .show(ui, |ui| {
                ui.label("Value");
                ui.text_edit_singleline(&mut self.value);
                ui.end_row();

                ui.label("Barcode kind");
                ui.combobox_from_iter("", &mut self.barcode_kind, BarcodeKind::iter());
                ui.end_row();

                ui.label("Bar width");
                ui.add(DragValue::new(&mut self.bar_width));
                ui.end_row();

                ui.label("Bar height");
                ui.add(DragValue::new(&mut self.bar_height));
                ui.end_row();

                ui.label("Horizontal padding");
                ui.add(DragValue::new(&mut self.horizontal_padding));
                ui.end_row();

                ui.label("Vertical padding");
                ui.add(DragValue::new(&mut self.vertical_padding));
                ui.end_row();

                ui.label("Label");
                ui.text_edit_singleline(&mut self.label);
                ui.end_row();

                ui.label("Label height");
                ui.add(DragValue::new(&mut self.label_height));
                ui.end_row();

                ui.label("Label top margin");
                ui.add(DragValue::new(&mut self.label_top_margin));
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
