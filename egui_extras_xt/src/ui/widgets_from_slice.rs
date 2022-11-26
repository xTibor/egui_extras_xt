use std::fmt::Display;

use egui::{ComboBox, Response, Ui, WidgetText};

// ----------------------------------------------------------------------------

pub trait SelectableValueFromSlice<'a, Value> {
    fn selectable_value_from_slice(
        &mut self,
        current_value: &mut Value,
        values: &'a [Value],
    ) -> Response;
}

impl<'a, Value> SelectableValueFromSlice<'a, Value> for Ui
where
    Value: PartialEq + Display + Clone,
{
    fn selectable_value_from_slice(
        &mut self,
        current_value: &mut Value,
        values: &'a [Value],
    ) -> Response {
        values
            .iter()
            .map(|value| self.selectable_value(current_value, value.clone(), format!("{value}")))
            .reduce(|result, response| result.union(response))
            .unwrap_or_else(|| {
                self.colored_label(self.style().visuals.error_fg_color, "\u{1F525} No items")
            })
    }
}

// ----------------------------------------------------------------------------

pub trait RadioValueFromSlice<'a, Value> {
    fn radio_value_from_slice(
        &mut self,
        current_value: &mut Value,
        values: &'a [Value],
    ) -> Response;
}

impl<'a, Value> RadioValueFromSlice<'a, Value> for Ui
where
    Value: PartialEq + Display + Clone,
{
    fn radio_value_from_slice(
        &mut self,
        current_value: &mut Value,
        values: &'a [Value],
    ) -> Response {
        values
            .iter()
            .map(|value| self.radio_value(current_value, value.clone(), format!("{value}")))
            .reduce(|result, response| result.union(response))
            .unwrap_or_else(|| {
                self.colored_label(self.style().visuals.error_fg_color, "\u{1F525} No items")
            })
    }
}

// ----------------------------------------------------------------------------

pub trait ComboBoxFromSlice<'a, Value> {
    fn combobox_from_slice(
        &mut self,
        label: impl Into<WidgetText>,
        current_value: &mut Value,
        values: &'a [Value],
    ) -> Response;
}

impl<'a, Value> ComboBoxFromSlice<'a, Value> for Ui
where
    Value: PartialEq + Display + Clone,
{
    fn combobox_from_slice(
        &mut self,
        label: impl Into<WidgetText>,
        current_value: &mut Value,
        values: &'a [Value],
    ) -> Response {
        let combobox_response = ComboBox::from_label(label)
            .selected_text(format!("{current_value}"))
            .show_ui(self, |ui| {
                values
                    .iter()
                    .map(|value| {
                        ui.selectable_value(current_value, value.clone(), format!("{value}"))
                    })
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
