use std::fmt::Display;

use egui::{ComboBox, Response, Ui, WidgetText};

// ----------------------------------------------------------------------------

pub trait SelectableValueFromIter<Value> {
    fn selectable_value_from_iter(
        &mut self,
        current_value: &mut Value,
        values: impl Iterator<Item = Value>,
    ) -> Response;
}

impl<Value> SelectableValueFromIter<Value> for Ui
where
    Value: PartialEq + Display + Copy,
{
    fn selectable_value_from_iter(
        &mut self,
        current_value: &mut Value,
        values: impl Iterator<Item = Value>,
    ) -> Response {
        values
            .map(|value| self.selectable_value(current_value, value, format!("{value}")))
            .reduce(|result, response| result.union(response))
            .unwrap_or_else(|| {
                self.colored_label(self.style().visuals.error_fg_color, "\u{1F525} No items")
            })
    }
}

// ----------------------------------------------------------------------------

pub trait RadioValueFromIter<Value> {
    fn radio_value_from_iter(
        &mut self,
        current_value: &mut Value,
        values: impl Iterator<Item = Value>,
    ) -> Response;
}

impl<Value> RadioValueFromIter<Value> for Ui
where
    Value: PartialEq + Display + Copy,
{
    fn radio_value_from_iter(
        &mut self,
        current_value: &mut Value,
        values: impl Iterator<Item = Value>,
    ) -> Response {
        values
            .map(|value| self.radio_value(current_value, value, format!("{value}")))
            .reduce(|result, response| result.union(response))
            .unwrap_or_else(|| {
                self.colored_label(self.style().visuals.error_fg_color, "\u{1F525} No items")
            })
    }
}

// ----------------------------------------------------------------------------

pub trait ComboBoxFromIter<Value> {
    fn combobox_from_iter(
        &mut self,
        label: impl Into<WidgetText>,
        current_value: &mut Value,
        values: impl Iterator<Item = Value>,
    ) -> Response;
}

impl<Value> ComboBoxFromIter<Value> for Ui
where
    Value: PartialEq + Display + Copy,
{
    fn combobox_from_iter(
        &mut self,
        label: impl Into<WidgetText>,
        current_value: &mut Value,
        values: impl Iterator<Item = Value>,
    ) -> Response {
        let combobox_response = ComboBox::from_label(label)
            .selected_text(format!("{current_value}"))
            .show_ui(self, |ui| {
                values
                    .map(|value| ui.selectable_value(current_value, value, format!("{value}")))
                    .reduce(|result, response| result.union(response))
                    .unwrap_or_else(|| {
                        ui.colored_label(ui.style().visuals.error_fg_color, "\u{1F525} No items")
                    })
            });

        combobox_response
            .inner
            .unwrap_or(combobox_response.response)
    }
}
