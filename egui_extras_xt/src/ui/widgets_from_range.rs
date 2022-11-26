use std::fmt::Display;
use std::iter::Step;
use std::ops::RangeInclusive;

use egui::{ComboBox, Response, Ui, WidgetText};

// ----------------------------------------------------------------------------

pub trait SelectableValueFromRange<Value> {
    fn selectable_value_from_range(
        &mut self,
        current_value: &mut Value,
        range: RangeInclusive<Value>,
    ) -> Response;
}

impl<Value> SelectableValueFromRange<Value> for Ui
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
}

// ----------------------------------------------------------------------------

pub trait RadioValueFromRange<Value> {
    fn radio_value_from_range(
        &mut self,
        current_value: &mut Value,
        range: RangeInclusive<Value>,
    ) -> Response;
}

impl<Value> RadioValueFromRange<Value> for Ui
where
    Value: PartialEq + Display + Copy + Step,
{
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
}

// ----------------------------------------------------------------------------

pub trait ComboBoxFromRange<Value> {
    fn combobox_from_range(
        &mut self,
        label: impl Into<WidgetText>,
        current_value: &mut Value,
        range: RangeInclusive<Value>,
    ) -> Response;
}

impl<Value> ComboBoxFromRange<Value> for Ui
where
    Value: PartialEq + Display + Copy + Step,
{
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
