use egui::{ComboBox, Ui, WidgetText};
use std::fmt::Display;

pub trait AddFromIterator<Value> {
    fn selectable_value_from_iter(
        &mut self,
        current_value: &mut Value,
        values: impl Iterator<Item = Value>,
    );

    fn radio_value_from_iter(
        &mut self,
        current_value: &mut Value,
        values: impl Iterator<Item = Value>,
    );

    fn combobox_from_iter(
        &mut self,
        label: impl Into<WidgetText>,
        current_value: &mut Value,
        values: impl Iterator<Item = Value>,
    );
}

impl<Value> AddFromIterator<Value> for Ui
where
    Value: PartialEq + Display + Copy,
{
    fn selectable_value_from_iter(
        &mut self,
        current_value: &mut Value,
        values: impl Iterator<Item = Value>,
    ) {
        for value in values {
            self.selectable_value(current_value, value, format!("{}", value));
        }
    }

    fn radio_value_from_iter(
        &mut self,
        current_value: &mut Value,
        values: impl Iterator<Item = Value>,
    ) {
        for value in values {
            self.radio_value(current_value, value, format!("{}", value));
        }
    }

    fn combobox_from_iter(
        &mut self,
        label: impl Into<WidgetText>,
        current_value: &mut Value,
        values: impl Iterator<Item = Value>,
    ) {
        ComboBox::from_label(label)
            .selected_text(format!("{}", current_value))
            .show_ui(self, |ui| {
                for value in values {
                    ui.selectable_value(current_value, value, format!("{}", value));
                }
            });
    }
}
