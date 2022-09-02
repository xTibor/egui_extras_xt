use egui::{Response, Ui};
use emath::Numeric;

pub trait OptionalValueWidget<T> {
    fn optional_value_widget(
        &mut self,
        value: &mut Option<T>,
        add_contents: impl FnOnce(&mut Self, &mut T) -> Response,
    ) -> Response;
}

impl<T: Numeric + Default> OptionalValueWidget<T> for Ui {
    fn optional_value_widget(
        &mut self,
        value: &mut Option<T>,
        add_contents: impl FnOnce(&mut Self, &mut T) -> Response,
    ) -> Response {
        self.group(|ui| {
            let mut checkbox_state = value.is_some();
            ui.checkbox(&mut checkbox_state, "");

            *value = match (value.is_some(), checkbox_state) {
                (false, true) => Some(T::default()),
                (true, false) => None,
                _ => *value,
            };

            match value {
                Some(ref mut value) => {
                    add_contents(ui, value);
                }
                None => {
                    let mut dummy_value = T::default();
                    ui.add_enabled_ui(false, |ui| add_contents(ui, &mut dummy_value));
                }
            }
        })
        .response
    }
}
