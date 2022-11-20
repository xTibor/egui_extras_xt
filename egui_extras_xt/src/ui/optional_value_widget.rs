use egui::{Response, Ui};

pub trait OptionalValueWidget<T> {
    fn optional_value_widget(
        &mut self,
        value: &mut Option<T>,
        add_contents: impl FnOnce(&mut Self, &mut T) -> Response,
    ) -> Response;
}

impl<T: Default> OptionalValueWidget<T> for Ui {
    fn optional_value_widget(
        &mut self,
        value: &mut Option<T>,
        add_contents: impl FnOnce(&mut Self, &mut T) -> Response,
    ) -> Response {
        self.group(|ui| {
            ui.horizontal(|ui| {
                let mut checkbox_state = value.is_some();
                let mut response = ui.checkbox(&mut checkbox_state, "");

                match (value.is_some(), checkbox_state) {
                    (false, true) => *value = Some(T::default()),
                    (true, false) => *value = None,
                    _ => {}
                };

                match value {
                    Some(ref mut value) => {
                        response = response.union(add_contents(ui, value));
                    }
                    None => {
                        let mut dummy_value = T::default();
                        ui.add_enabled_ui(false, |ui| add_contents(ui, &mut dummy_value));
                    }
                }

                response
            })
        })
        .inner
        .inner
    }
}
