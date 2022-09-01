use eframe::egui::{DragValue, Grid, Ui};
use egui_extras_xt::displays::{DisplayStyle, DisplayStylePreset};
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
            ui.label("Preset");
            ui.horizontal(|ui| {
                ui.combobox_from_iter("", style_preset, DisplayStylePreset::iter());
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
