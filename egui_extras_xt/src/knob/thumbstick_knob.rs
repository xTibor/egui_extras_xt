use std::f32::consts::TAU;
use std::ops::RangeInclusive;

use egui::{self, lerp, Response, Sense, Ui, Widget};
use emath::{remap_clamp, vec2, Rect, Rot2, Vec2};

use crate::common::{paint_ellipse, Orientation, WidgetShape, Winding};

// ----------------------------------------------------------------------------

/// Combined into one function (rather than two) to make it easier
/// for the borrow checker.
type GetSetValue<'a> = Box<dyn 'a + FnMut(Option<(f32, f32)>) -> (f32, f32)>;

fn get(get_set_value: &mut GetSetValue<'_>) -> (f32, f32) {
    (get_set_value)(None)
}

fn set(get_set_value: &mut GetSetValue<'_>, value: (f32, f32)) {
    (get_set_value)(Some(value));
}

// ----------------------------------------------------------------------------

#[must_use = "You should put this widget in an ui with `ui.add(widget);`"]
pub struct ThumbstickKnob<'a> {
    get_set_value: GetSetValue<'a>,
    interactive: bool,
    diameter: f32,
    animated: bool,
}

impl<'a> ThumbstickKnob<'a> {
    pub fn new(value: &'a mut (f32, f32)) -> Self {
        Self::from_get_set(move |v: Option<(f32, f32)>| {
            if let Some(v) = v {
                *value = v;
            }
            *value
        })
    }

    pub fn from_get_set(get_set_value: impl 'a + FnMut(Option<(f32, f32)>) -> (f32, f32)) -> Self {
        Self {
            get_set_value: Box::new(get_set_value),
            interactive: true,
            diameter: 128.0,
            animated: true,
        }
    }

    pub fn interactive(mut self, interactive: bool) -> Self {
        self.interactive = interactive;
        self
    }

    pub fn diameter(mut self, diameter: impl Into<f32>) -> Self {
        self.diameter = diameter.into();
        self
    }

    pub fn animated(mut self, animated: bool) -> Self {
        self.animated = animated;
        self
    }
}

impl<'a> Widget for ThumbstickKnob<'a> {
    fn ui(mut self, ui: &mut Ui) -> Response {
        let desired_size = Vec2::splat(self.diameter);

        let (rect, mut response) = ui.allocate_exact_size(
            desired_size,
            if self.interactive {
                Sense::click_and_drag()
            } else {
                Sense::hover()
            },
        );

        if ui.is_rect_visible(rect) {
            let visuals = *ui.style().interact(&response);

            let (r, theta) = {
                let (x, y) = get(&mut self.get_set_value);
                let v = vec2(x, y);
                (v.length().clamp(0.0, 1.0), v.angle())
            };

            let tilt_factor = 0.75;

            ui.painter().circle(
                rect.center(),
                self.diameter / 2.0,
                ui.style().visuals.faint_bg_color,
                ui.style().visuals.window_stroke(),
            );

            let mut paint_thumbstick = |size| {
                let ellipse_center = rect.center()
                    + Vec2::angled(theta)
                        * r
                        * ((self.diameter - (self.diameter * tilt_factor * size)) / 2.0);

                let ellipse_size = Vec2::splat(self.diameter)
                    * size
                    * Vec2::new(1.0 - (1.0 - tilt_factor) * r, 1.0);

                paint_ellipse(
                    ui,
                    ellipse_center,
                    ellipse_size,
                    visuals.bg_fill,
                    visuals.fg_stroke,
                    Rot2::from_angle(theta),
                );
            };

            paint_thumbstick(0.750);
            paint_thumbstick(0.625);
        }

        response
    }
}
