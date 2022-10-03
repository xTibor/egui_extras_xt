use eframe::egui::{Grid, TextStyle, Ui};
use egui_extras_xt::ui::hyperlink_with_icon::HyperlinkWithIcon;

use crate::pages::PageImpl;

pub struct HyperlinkWithIconPage {
    label: String,
    url: String,
}

impl Default for HyperlinkWithIconPage {
    fn default() -> HyperlinkWithIconPage {
        HyperlinkWithIconPage {
            label: "egui_extras_xt".to_owned(),
            url: "https://github.com/xTibor/egui_extras_xt".to_owned(),
        }
    }
}

impl PageImpl for HyperlinkWithIconPage {
    fn ui(&mut self, ui: &mut Ui) {
        ui.scope(|ui| {
            ui.style_mut().override_text_style = Some(TextStyle::Heading);
            ui.hyperlink_with_icon_to(&self.label, &self.url);
        });
        ui.separator();

        Grid::new("hyperlink_with_icon_properties")
            .num_columns(2)
            .spacing([20.0, 10.0])
            .striped(true)
            .show(ui, |ui| {
                ui.label("Label");
                ui.text_edit_singleline(&mut self.label);
                ui.end_row();

                ui.label("URL");
                ui.text_edit_singleline(&mut self.url);
                ui.end_row();
            });
    }
}
