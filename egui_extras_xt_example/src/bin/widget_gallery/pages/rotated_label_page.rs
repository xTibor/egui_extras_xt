use eframe::egui::{Grid, TextStyle, Ui};
use egui_extras_xt::ui::rotated_label::RotatedLabel;

use crate::pages::PageImpl;

pub struct RotatedLabelPage {
    text: String,
    angle: f32,
}

impl Default for RotatedLabelPage {
    fn default() -> RotatedLabelPage {
        RotatedLabelPage {
            text: "egui_extras_xt".to_owned(),
            angle: 0.0,
        }
    }
}

impl PageImpl for RotatedLabelPage {
    fn ui(&mut self, ui: &mut Ui) {
        ui.scope(|ui| {
            ui.style_mut().override_text_style = Some(TextStyle::Heading);
            ui.add(RotatedLabel::new(&self.text).angle(self.angle));
        });
        ui.separator();

        Grid::new("rotated_label_properties")
            .num_columns(2)
            .spacing([20.0, 10.0])
            .striped(true)
            .show(ui, |ui| {
                ui.label("Text");
                ui.text_edit_singleline(&mut self.text);
                ui.end_row();

                ui.label("Angle");
                ui.drag_angle(&mut self.angle);
                ui.end_row();
            });
    }
}
