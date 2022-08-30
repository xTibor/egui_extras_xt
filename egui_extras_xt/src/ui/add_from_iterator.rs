use egui::Ui;
use std::fmt::Display;

pub trait AddFromIterator {
    fn selectable_value_from_iter<Value: PartialEq + Display + Copy>(
        &mut self,
        current_value: &mut Value,
        values: impl Iterator<Item = Value>,
    );
}

impl AddFromIterator for Ui {
    fn selectable_value_from_iter<Value: PartialEq + Display + Copy>(
        &mut self,
        current_value: &mut Value,
        values: impl Iterator<Item = Value>,
    ) {
        for value in values {
            self.selectable_value(current_value, value, format!("{}", value));
        }
    }
}
