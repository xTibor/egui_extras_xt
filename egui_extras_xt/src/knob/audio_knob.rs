use std::f32::consts::TAU;
use std::ops::RangeInclusive;

use egui::{self, Response, Sense, Ui, Widget};
use emath::{remap_clamp, Vec2};

use crate::common::{Orientation, WidgetShape, Winding};

// ----------------------------------------------------------------------------

/// Combined into one function (rather than two) to make it easier
/// for the borrow checker.
type GetSetValue<'a> = Box<dyn 'a + FnMut(Option<f32>) -> f32>;

fn get(get_set_value: &mut GetSetValue<'_>) -> f32 {
    (get_set_value)(None)
}

fn set(get_set_value: &mut GetSetValue<'_>, value: f32) {
    (get_set_value)(Some(value));
}

// ----------------------------------------------------------------------------

#[must_use = "You should put this widget in an ui with `ui.add(widget);`"]
pub struct AudioKnob<'a> {
    get_set_value: GetSetValue<'a>,
    interactive: bool,
    diameter: f32,
    winding: Winding,
    orientation: Orientation,
    range: RangeInclusive<f32>,
    spread: f32,
    thickness: f32,
    shape: WidgetShape<'a>,
    animated: bool,
    snap: Option<f32>,
    shift_snap: Option<f32>,
}

impl<'a> AudioKnob<'a> {
    pub fn new(value: &'a mut f32, range: RangeInclusive<f32>) -> Self {
        Self::from_get_set(range, move |v: Option<f32>| {
            if let Some(v) = v {
                *value = v;
            }
            *value
        })
    }

    pub fn from_get_set(
        range: RangeInclusive<f32>,
        get_set_value: impl 'a + FnMut(Option<f32>) -> f32,
    ) -> Self {
        Self {
            get_set_value: Box::new(get_set_value),
            interactive: true,
            diameter: 32.0,
            orientation: Orientation::Top,
            winding: Winding::Clockwise,
            range,
            spread: 1.0,
            thickness: 0.66,
            shape: WidgetShape::Squircle(4.0),
            animated: true,
            snap: None,
            shift_snap: None,
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

    pub fn winding(mut self, winding: Winding) -> Self {
        self.winding = winding;
        self
    }

    pub fn orientation(mut self, orientation: Orientation) -> Self {
        self.orientation = orientation;
        self
    }

    pub fn spread(mut self, spread: impl Into<f32>) -> Self {
        self.spread = spread.into();
        self
    }

    pub fn thickness(mut self, thickness: impl Into<f32>) -> Self {
        self.thickness = thickness.into();
        self
    }

    pub fn shape(mut self, shape: WidgetShape<'a>) -> Self {
        self.shape = shape;
        self
    }

    pub fn animated(mut self, animated: bool) -> Self {
        self.animated = animated;
        self
    }

    pub fn snap(mut self, snap: Option<f32>) -> Self {
        self.snap = snap;
        self
    }

    pub fn shift_snap(mut self, shift_snap: Option<f32>) -> Self {
        self.shift_snap = shift_snap;
        self
    }
}

impl<'a> Widget for AudioKnob<'a> {
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

        let constrain_value = |value: f32| value.clamp(*self.range.start(), *self.range.end());

        if response.dragged() {
            let drag_delta = self.orientation.rot2().inverse() * response.drag_delta();

            let mut new_value = get(&mut self.get_set_value);

            let delta = drag_delta.x + drag_delta.y * self.winding.to_float();
            new_value += delta * (self.range.end() - self.range.start()) / self.diameter;

            set(&mut self.get_set_value, constrain_value(new_value));
            response.mark_changed();
        }

        if response.drag_released() {
            if self.animated {
                ui.ctx().clear_animations();
                ui.ctx().animate_value_with_time(
                    response.id,
                    get(&mut self.get_set_value),
                    ui.style().animation_time,
                );
            }

            if let Some(snap_angle) = if ui.input().modifiers.shift_only() {
                self.shift_snap
            } else {
                self.snap
            } {
                assert!(
                    snap_angle > 0.0,
                    "non-positive snap angles are not supported"
                );
                let new_value = (get(&mut self.get_set_value) / snap_angle).round() * snap_angle;
                set(&mut self.get_set_value, constrain_value(new_value));
                response.mark_changed();
            }
        }

        if ui.is_rect_visible(rect) {
            let visuals = *ui.style().interact(&response);

            let value = if self.animated && !response.dragged() {
                ui.ctx()
                    .animate_value_with_time(response.id, get(&mut self.get_set_value), 0.1)
            } else {
                get(&mut self.get_set_value)
            };

            let center_angle = (self.orientation.rot2() * Vec2::RIGHT).angle();
            let spread_angle = (TAU / 2.0) * self.spread.clamp(0.0, 1.0);

            let (min_angle, max_angle) = (
                center_angle - spread_angle * self.winding.to_float(),
                center_angle + spread_angle * self.winding.to_float(),
            );

            let outer_radius = self.diameter / 2.0;
            let inner_radius = outer_radius * (1.0 - self.thickness.clamp(0.0, 1.0));

            self.shape.paint_arc(
                ui,
                rect.center(),
                inner_radius,
                outer_radius,
                min_angle,
                max_angle,
                ui.style().visuals.faint_bg_color,
                ui.style().visuals.window_stroke(),
                self.orientation.rot2(),
            );

            self.shape.paint_arc(
                ui,
                rect.center(),
                (inner_radius - visuals.expansion).max(0.0),
                outer_radius + visuals.expansion,
                remap_clamp(0.0, self.range.clone(), min_angle..=max_angle),
                remap_clamp(value, self.range, min_angle..=max_angle),
                visuals.bg_fill,
                visuals.fg_stroke,
                self.orientation.rot2(),
            );
        }

        response
    }
}
