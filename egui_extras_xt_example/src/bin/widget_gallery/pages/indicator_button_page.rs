use eframe::egui::{DragValue, Grid, Ui};
use egui_extras_xt::displays::{DisplayStyle, DisplayStylePreset, IndicatorButton};

use crate::pages::ui::display_style_ui;
use crate::pages::PageImpl;

pub struct IndicatorButtonPage {
    value: bool,
    width: f32,
    height: f32,
    label: String,
    style: DisplayStyle,
    style_preset: DisplayStylePreset,
    animated: bool,
    interactive: bool,
    margin: f32,
}

impl Default for IndicatorButtonPage {
    fn default() -> IndicatorButtonPage {
        IndicatorButtonPage {
            value: false,
            width: 64.0,
            height: 40.0,
            label: "TEST".to_owned(),
            style: DisplayStylePreset::Default.style(),
            style_preset: DisplayStylePreset::Default,
            animated: true,
            interactive: true,
            margin: 0.2,
        }
    }
}

impl PageImpl for IndicatorButtonPage {
    fn ui(&mut self, ui: &mut Ui) {
        ui.add(
            IndicatorButton::new(&mut self.value)
                .width(self.width)
                .height(self.height)
                .label(&self.label)
                .style(self.style)
                .animated(self.animated)
                .interactive(self.interactive)
                .margin(self.margin),
        );
        ui.separator();

        Grid::new("indicator_button_properties")
            .num_columns(2)
            .spacing([20.0, 10.0])
            .striped(true)
            .show(ui, |ui| {
                ui.label("Value");
                ui.checkbox(&mut self.value, "");
                ui.end_row();

                ui.label("Width");
                ui.add(DragValue::new(&mut self.width));
                ui.end_row();

                ui.label("Height");
                ui.add(DragValue::new(&mut self.height));
                ui.end_row();

                ui.label("Label");
                ui.text_edit_singleline(&mut self.label);
                ui.end_row();

                ui.label("Style");
                display_style_ui(ui, &mut self.style, &mut self.style_preset);
                ui.end_row();

                ui.label("Animated");
                ui.checkbox(&mut self.animated, "");
                ui.end_row();

                ui.label("Interactive");
                ui.checkbox(&mut self.interactive, "");
                ui.end_row();

                ui.label("Margin");
                ui.add(DragValue::new(&mut self.margin));
                ui.end_row();
            });
    }
}
