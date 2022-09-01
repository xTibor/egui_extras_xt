use std::ops::RangeInclusive;

use eframe::egui::{DragValue, Grid, Ui};
use egui_extras_xt::displays::{DisplayStyle, DisplayStylePreset, LedDisplay};
use egui_extras_xt::ui::drag_rangeinclusive::DragRangeInclusive;

use crate::pages::ui::display_style_ui;
use crate::pages::PageImpl;

pub struct LedDisplayPage {
    value: f32,
    diameter: f32,
    padding: f32,
    range: RangeInclusive<f32>,
    style: DisplayStyle,
    style_preset: DisplayStylePreset,
    animated: bool,
}

impl Default for LedDisplayPage {
    fn default() -> LedDisplayPage {
        LedDisplayPage {
            value: 1.0,
            diameter: 16.0,
            padding: 0.25,
            range: 0.0..=1.0,
            style: DisplayStylePreset::Default.style(),
            style_preset: DisplayStylePreset::Default,
            animated: false,
        }
    }
}

impl PageImpl for LedDisplayPage {
    fn ui(&mut self, ui: &mut Ui) {
        ui.add(
            LedDisplay::new(self.value)
                .diameter(self.diameter)
                .padding(self.padding)
                .range(self.range.clone())
                .style(self.style)
                .animated(self.animated),
        );
        ui.separator();

        Grid::new("led_display_properties")
            .num_columns(2)
            .spacing([20.0, 10.0])
            .striped(true)
            .show(ui, |ui| {
                ui.label("Value");
                ui.add(DragValue::new(&mut self.value));
                ui.end_row();

                ui.label("Diameter");
                ui.add(DragValue::new(&mut self.diameter));
                ui.end_row();

                ui.label("Padding");
                ui.add(DragValue::new(&mut self.padding));
                ui.end_row();

                ui.label("Range");
                ui.drag_rangeinclusive(&mut self.range);
                ui.end_row();

                ui.label("Style");
                display_style_ui(ui, &mut self.style, &mut self.style_preset);
                ui.end_row();

                ui.label("Animated");
                ui.checkbox(&mut self.animated, "");
                ui.end_row();
            });
    }
}
