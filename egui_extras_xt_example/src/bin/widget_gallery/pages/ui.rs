use eframe::egui::{DragValue, Grid, Ui};
use egui_extras_xt::displays::segmented_display::DisplayMetricsPreset;
use egui_extras_xt::displays::{DisplayMetrics, DisplayStyle, DisplayStylePreset};
use egui_extras_xt::ui::widgets_from::WidgetsFromIterator;
use strum::IntoEnumIterator;

pub fn display_style_ui(
    ui: &mut Ui,
    style: &mut DisplayStyle,
    style_preset: &mut DisplayStylePreset,
) {
    Grid::new("style_properties")
        .num_columns(2)
        .spacing([20.0, 10.0])
        .striped(true)
        .show(ui, |ui| {
            ui.label("Style preset");
            ui.horizontal(|ui| {
                ui.push_id("style_preset_combo", |ui| {
                    ui.combobox_from_iter("", style_preset, DisplayStylePreset::iter())
                });
                // `.changed()` responses of combobox are broken.
                if ui.button("\u{2714} Apply").clicked() {
                    *style = style_preset.style();
                }
            });
            ui.end_row();

            ui.label("Background color");
            ui.color_edit_button_srgba(&mut style.background_color);
            ui.end_row();

            ui.label("Inactive foreground color");
            ui.color_edit_button_srgba(&mut style.inactive_foreground_color);
            ui.end_row();

            ui.label("Inactive foreground stroke");
            ui.horizontal(|ui| {
                ui.color_edit_button_srgba(&mut style.inactive_foreground_stroke.color);
                ui.add(DragValue::new(&mut style.inactive_foreground_stroke.width));
            });
            ui.end_row();

            ui.label("Active foreground color");
            ui.color_edit_button_srgba(&mut style.active_foreground_color);
            ui.end_row();

            ui.label("Active foreground stroke");
            ui.horizontal(|ui| {
                ui.color_edit_button_srgba(&mut style.active_foreground_stroke.color);
                ui.add(DragValue::new(&mut style.active_foreground_stroke.width));
            });
            ui.end_row();
        });
}

pub fn display_metrics_ui(
    ui: &mut Ui,
    metrics: &mut DisplayMetrics,
    metrics_preset: &mut DisplayMetricsPreset,
) {
    Grid::new("metrics_properties")
        .num_columns(2)
        .spacing([20.0, 10.0])
        .striped(true)
        .show(ui, |ui| {
            ui.label("Metrics preset");
            ui.horizontal(|ui| {
                ui.push_id("metrics_preset_combo", |ui| {
                    ui.combobox_from_iter("", metrics_preset, DisplayMetricsPreset::iter())
                });
                // `.changed()` responses of combobox are broken.
                if ui.button("\u{2714} Apply").clicked() {
                    *metrics = metrics_preset.metrics();
                }
            });
            ui.end_row();

            ui.label("Segment spacing");
            ui.add(DragValue::new(&mut metrics.segment_spacing));
            ui.end_row();

            ui.label("Segment thickness");
            ui.add(DragValue::new(&mut metrics.segment_thickness));
            ui.end_row();

            ui.label("Digit median");
            ui.add(DragValue::new(&mut metrics.digit_median));
            ui.end_row();

            ui.label("Digit ratio");
            ui.add(DragValue::new(&mut metrics.digit_ratio));
            ui.end_row();

            ui.label("Digit shearing");
            ui.add(DragValue::new(&mut metrics.digit_shearing));
            ui.end_row();

            ui.label("Digit spacing");
            ui.add(DragValue::new(&mut metrics.digit_spacing));
            ui.end_row();

            ui.label("Horizontal margin");
            ui.add(DragValue::new(&mut metrics.margin_horizontal));
            ui.end_row();

            ui.label("Vertical margin");
            ui.add(DragValue::new(&mut metrics.margin_vertical));
            ui.end_row();

            ui.label("Colon separation");
            ui.add(DragValue::new(&mut metrics.colon_separation));
            ui.end_row();
        });
}
