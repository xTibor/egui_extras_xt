use std::ops::RangeInclusive;

use egui::{DragValue, Response, Ui};
use emath::Numeric;

pub trait DragRangeInclusive<T> {
    fn drag_rangeinclusive(&mut self, value: &mut RangeInclusive<T>) -> Response;
}

impl<T: Numeric> DragRangeInclusive<T> for Ui {
    fn drag_rangeinclusive(&mut self, value: &mut RangeInclusive<T>) -> Response {
        self.group(|ui| {
            let (mut start, mut end) = (*value.start(), *value.end());

            ui.add(DragValue::new(&mut start));
            ui.add(DragValue::new(&mut end));

            *value = start..=end;
        })
        .response
    }
}
