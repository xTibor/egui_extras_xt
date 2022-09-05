use egui::{ComboBox, Ui, WidgetText};
use std::fmt::Display;
use std::iter::Step;
use std::ops::RangeInclusive;

// ----------------------------------------------------------------------------

pub trait WidgetsFromIterator<Value> {
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

impl<Value> WidgetsFromIterator<Value> for Ui
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

// ----------------------------------------------------------------------------

pub trait WidgetsFromSlice<'a, Value> {
    fn selectable_value_from_slice(&mut self, current_value: &mut Value, values: &'a [Value]);

    fn radio_value_from_slice(&mut self, current_value: &mut Value, values: &'a [Value]);

    fn combobox_from_slice(
        &mut self,
        label: impl Into<WidgetText>,
        current_value: &mut Value,
        values: &'a [Value],
    );
}

impl<'a, Value> WidgetsFromSlice<'a, Value> for Ui
where
    Value: PartialEq + Display + Clone,
{
    fn selectable_value_from_slice(&mut self, current_value: &mut Value, values: &'a [Value]) {
        for value in values {
            self.selectable_value(current_value, value.clone(), format!("{}", value));
        }
    }

    fn radio_value_from_slice(&mut self, current_value: &mut Value, values: &'a [Value]) {
        for value in values {
            self.radio_value(current_value, value.clone(), format!("{}", value));
        }
    }

    fn combobox_from_slice(
        &mut self,
        label: impl Into<WidgetText>,
        current_value: &mut Value,
        values: &'a [Value],
    ) {
        ComboBox::from_label(label)
            .selected_text(format!("{}", current_value))
            .show_ui(self, |ui| {
                for value in values {
                    ui.selectable_value(current_value, value.clone(), format!("{}", value));
                }
            });
    }
}

// ----------------------------------------------------------------------------

pub trait WidgetsFromRange<Value> {
    fn selectable_value_from_range(
        &mut self,
        current_value: &mut Value,
        range: RangeInclusive<Value>,
    );

    fn radio_value_from_range(&mut self, current_value: &mut Value, range: RangeInclusive<Value>);

    fn combobox_from_range(
        &mut self,
        label: impl Into<WidgetText>,
        current_value: &mut Value,
        range: RangeInclusive<Value>,
    );
}

impl<Value> WidgetsFromRange<Value> for Ui
where
    Value: PartialEq + Display + Copy + Step,
{
    fn selectable_value_from_range(
        &mut self,
        current_value: &mut Value,
        range: RangeInclusive<Value>,
    ) {
        for value in range {
            self.selectable_value(current_value, value, format!("{}", value));
        }
    }

    fn radio_value_from_range(&mut self, current_value: &mut Value, range: RangeInclusive<Value>) {
        for value in range {
            self.radio_value(current_value, value, format!("{}", value));
        }
    }

    fn combobox_from_range(
        &mut self,
        label: impl Into<WidgetText>,
        current_value: &mut Value,
        range: RangeInclusive<Value>,
    ) {
        ComboBox::from_label(label)
            .selected_text(format!("{}", current_value))
            .show_ui(self, |ui| {
                for value in range {
                    ui.selectable_value(current_value, value, format!("{}", value));
                }
            });
    }
}
