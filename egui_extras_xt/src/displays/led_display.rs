use std::ops::RangeInclusive;

use egui::{self, remap_clamp, Response, Sense, Ui, Widget};
use emath::Vec2;
use epaint::Stroke;

use crate::displays::{DisplayStyle, DisplayStylePreset};

#[must_use = "You should put this widget in an ui with `ui.add(widget);`"]
pub struct LedDisplay {
    value: f32,
    diameter: f32,
    padding: f32,
    range: RangeInclusive<f32>,
    style: DisplayStyle,
    animated: bool,
}

impl LedDisplay {
    pub fn new(value: f32) -> Self {
        Self {
            value,
            diameter: 16.0,
            padding: 0.25,
            range: 0.0..=1.0,
            style: DisplayStylePreset::Default.style(),
            animated: true,
        }
    }

    pub fn from_bool(value: bool) -> Self {
        Self::new(if value { 1.0 } else { 0.0 })
    }

    pub fn diameter(mut self, diameter: impl Into<f32>) -> Self {
        self.diameter = diameter.into();
        self
    }

    pub fn padding(mut self, padding: impl Into<f32>) -> Self {
        self.padding = padding.into();
        self
    }

    pub fn range(mut self, range: RangeInclusive<f32>) -> Self {
        self.range = range;
        self
    }

    pub fn style(mut self, style: DisplayStyle) -> Self {
        self.style = style;
        self
    }

    pub fn style_preset(mut self, preset: DisplayStylePreset) -> Self {
        self.style = preset.style();
        self
    }

    pub fn animated(mut self, animated: bool) -> Self {
        self.animated = animated;
        self
    }
}

impl Widget for LedDisplay {
    fn ui(self, ui: &mut Ui) -> Response {
        let desired_size = Vec2::splat(self.diameter + self.padding * self.diameter);

        let (rect, response) = ui.allocate_exact_size(desired_size, Sense::hover());

        if ui.is_rect_visible(rect) {
            let value = remap_clamp(
                if self.animated {
                    ui.ctx()
                        .animate_value_with_time(response.id, self.value, 0.1)
                } else {
                    self.value
                },
                self.range,
                0.0..=1.0,
            );

            ui.painter().rect(
                rect,
                ui.style().visuals.noninteractive().rounding,
                self.style.background_color,
                Stroke::none(),
            );

            ui.painter().circle(
                rect.center(),
                self.diameter / 2.0,
                self.style.foreground_color_blend(value),
                self.style.foreground_stroke_blend(value),
            );
        }

        response
    }
}
