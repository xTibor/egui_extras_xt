use eframe::egui::{Grid, Ui};
use eframe::epaint::Color32;
use egui_extras_xt::ui::optional_value_widget::OptionalValueWidget;
use egui_extras_xt::ui::rotated_label::RotatedLabel;

use crate::pages::PageImpl;

pub struct RotatedLabelPage {
    value: String,
    angle: f32,
    color: Option<Color32>,
}

impl Default for RotatedLabelPage {
    fn default() -> RotatedLabelPage {
        RotatedLabelPage {
            value: "egui_extras_xt".to_owned(),
            angle: 0.0,
            color: None,
        }
    }
}

impl PageImpl for RotatedLabelPage {
    fn ui(&mut self, ui: &mut Ui) {
        ui.add(
            RotatedLabel::new(&self.value)
                .angle(self.angle)
                .color(self.color),
        );
        ui.separator();

        Grid::new("rotated_label_properties")
            .num_columns(2)
            .spacing([20.0, 10.0])
            .striped(true)
            .show(ui, |ui| {
                ui.label("Value");
                ui.text_edit_singleline(&mut self.value);
                ui.end_row();

                ui.label("Angle");
                ui.drag_angle(&mut self.angle);
                ui.end_row();

                ui.label("Color");
                ui.optional_value_widget(&mut self.color, |ui, value| {
                    ui.color_edit_button_srgba(value)
                });
                ui.end_row();
            });
    }
}
