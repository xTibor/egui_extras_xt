use eframe::egui::{Grid, Ui};
use egui_extras_xt::ui::standard_buttons::{ButtonKind, StandardButtons};
use egui_extras_xt::ui::widgets_from_iter::ComboBoxFromIter;
use strum::IntoEnumIterator;

use crate::pages::PageImpl;

pub struct StandardButtonsPage {
    button_kind: ButtonKind,
}

impl Default for StandardButtonsPage {
    fn default() -> StandardButtonsPage {
        StandardButtonsPage {
            button_kind: ButtonKind::Ok,
        }
    }
}

impl PageImpl for StandardButtonsPage {
    fn ui(&mut self, ui: &mut Ui) {
        ui.standard_button(self.button_kind);
        ui.separator();

        Grid::new("standard_buttons_properties")
            .num_columns(2)
            .spacing([20.0, 10.0])
            .striped(true)
            .show(ui, |ui| {
                ui.label("Button kind");
                ui.combobox_from_iter("", &mut self.button_kind, ButtonKind::iter());
                ui.end_row();
            });
    }
}
