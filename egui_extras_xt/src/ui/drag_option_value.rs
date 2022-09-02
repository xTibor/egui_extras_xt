use egui::{DragValue, Response, Ui};
use emath::Numeric;

pub trait DragOptionValue<T> {
    fn drag_option_value(&mut self, value: &mut Option<T>) -> Response;
}

impl<T: Numeric + Default> DragOptionValue<T> for Ui {
    fn drag_option_value(&mut self, value: &mut Option<T>) -> Response {
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
                    ui.add(DragValue::new(value));
                }
                None => {
                    let mut dummy_value = T::default();
                    ui.add_enabled(false, DragValue::new(&mut dummy_value));
                }
            }
        })
        .response
    }
}
