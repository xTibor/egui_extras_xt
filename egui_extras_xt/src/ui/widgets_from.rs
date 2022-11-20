use egui::{ComboBox, Response, Ui, WidgetText};
use std::fmt::Display;
use std::iter::Step;
use std::ops::RangeInclusive;

// ----------------------------------------------------------------------------

pub trait WidgetsFromIterator<Value> {
    fn selectable_value_from_iter(
        &mut self,
        current_value: &mut Value,
        values: impl Iterator<Item = Value>,
    ) -> Response;

    fn radio_value_from_iter(
        &mut self,
        current_value: &mut Value,
        values: impl Iterator<Item = Value>,
    ) -> Response;

    fn combobox_from_iter(
        &mut self,
        label: impl Into<WidgetText>,
        current_value: &mut Value,
        values: impl Iterator<Item = Value>,
    ) -> Response;
}

impl<Value> WidgetsFromIterator<Value> for Ui
where
    Value: PartialEq + Display + Copy,
{
    fn selectable_value_from_iter(
        &mut self,
        current_value: &mut Value,
        values: impl Iterator<Item = Value>,
    ) -> Response {
        values
            .map(|value| self.selectable_value(current_value, value, format!("{}", value)))
            .reduce(|result, response| result.union(response))
            .unwrap_or_else(|| {
                self.colored_label(self.style().visuals.error_fg_color, "\u{1F525} No items")
            })
    }

    fn radio_value_from_iter(
        &mut self,
        current_value: &mut Value,
        values: impl Iterator<Item = Value>,
    ) -> Response {
        values
            .map(|value| self.radio_value(current_value, value, format!("{}", value)))
            .reduce(|result, response| result.union(response))
            .unwrap_or_else(|| {
                self.colored_label(self.style().visuals.error_fg_color, "\u{1F525} No items")
            })
    }

    fn combobox_from_iter(
        &mut self,
        label: impl Into<WidgetText>,
        current_value: &mut Value,
        values: impl Iterator<Item = Value>,
    ) -> Response {
        let combobox_response = ComboBox::from_label(label)
            .selected_text(format!("{}", current_value))
            .show_ui(self, |ui| {
                values
                    .map(|value| ui.selectable_value(current_value, value, format!("{}", value)))
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

// ----------------------------------------------------------------------------

pub trait WidgetsFromSlice<'a, Value> {
    fn selectable_value_from_slice(
        &mut self,
        current_value: &mut Value,
        values: &'a [Value],
    ) -> Response;

    fn radio_value_from_slice(
        &mut self,
        current_value: &mut Value,
        values: &'a [Value],
    ) -> Response;

    fn combobox_from_slice(
        &mut self,
        label: impl Into<WidgetText>,
        current_value: &mut Value,
        values: &'a [Value],
    ) -> Response;
}

impl<'a, Value> WidgetsFromSlice<'a, Value> for Ui
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
            .map(|value| self.selectable_value(current_value, value.clone(), format!("{}", value)))
            .reduce(|result, response| result.union(response))
            .unwrap_or_else(|| {
                self.colored_label(self.style().visuals.error_fg_color, "\u{1F525} No items")
            })
    }

    fn radio_value_from_slice(
        &mut self,
        current_value: &mut Value,
        values: &'a [Value],
    ) -> Response {
        values
            .iter()
            .map(|value| self.radio_value(current_value, value.clone(), format!("{}", value)))
            .reduce(|result, response| result.union(response))
            .unwrap_or_else(|| {
                self.colored_label(self.style().visuals.error_fg_color, "\u{1F525} No items")
            })
    }

    fn combobox_from_slice(
        &mut self,
        label: impl Into<WidgetText>,
        current_value: &mut Value,
        values: &'a [Value],
    ) -> Response {
        let combobox_response = ComboBox::from_label(label)
            .selected_text(format!("{}", current_value))
            .show_ui(self, |ui| {
                values
                    .iter()
                    .map(|value| {
                        ui.selectable_value(current_value, value.clone(), format!("{}", value))
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

// ----------------------------------------------------------------------------

pub trait WidgetsFromRange<Value> {
    fn selectable_value_from_range(
        &mut self,
        current_value: &mut Value,
        range: RangeInclusive<Value>,
    ) -> Response;

    fn radio_value_from_range(
        &mut self,
        current_value: &mut Value,
        range: RangeInclusive<Value>,
    ) -> Response;

    fn combobox_from_range(
        &mut self,
        label: impl Into<WidgetText>,
        current_value: &mut Value,
        range: RangeInclusive<Value>,
    ) -> Response;
}

impl<Value> WidgetsFromRange<Value> for Ui
where
    Value: PartialEq + Display + Copy + Step,
{
    fn selectable_value_from_range(
        &mut self,
        current_value: &mut Value,
        range: RangeInclusive<Value>,
    ) -> Response {
        range
            .map(|value| self.selectable_value(current_value, value, format!("{}", value)))
            .reduce(|result, response| result.union(response))
            .unwrap_or_else(|| {
                self.colored_label(self.style().visuals.error_fg_color, "\u{1F525} No items")
            })
    }

    fn radio_value_from_range(
        &mut self,
        current_value: &mut Value,
        range: RangeInclusive<Value>,
    ) -> Response {
        range
            .map(|value| self.radio_value(current_value, value, format!("{}", value)))
            .reduce(|result, response| result.union(response))
            .unwrap_or_else(|| {
                self.colored_label(self.style().visuals.error_fg_color, "\u{1F525} No items")
            })
    }

    fn combobox_from_range(
        &mut self,
        label: impl Into<WidgetText>,
        current_value: &mut Value,
        range: RangeInclusive<Value>,
    ) -> Response {
        let combobox_response = ComboBox::from_label(label)
            .selected_text(format!("{}", current_value))
            .show_ui(self, |ui| {
                range
                    .map(|value| ui.selectable_value(current_value, value, format!("{}", value)))
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
