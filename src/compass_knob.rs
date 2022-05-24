use std::f32::consts::TAU;

use eframe::egui::{self, Response, Ui, Widget};
use eframe::emath::{normalized_angle, pos2, vec2, Align2};
use eframe::epaint::{FontFamily, FontId, Shape};

use crate::common::{normalized_angle_unsigned, KnobMode};

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

pub struct CompassLabels<'a>(pub [&'a str; 4]);

#[must_use = "You should put this widget in an ui with `ui.add(widget);`"]
pub struct CompassKnob<'a> {
    get_set_value: GetSetValue<'a>,
    mode: KnobMode,
    width: f32,
    height: f32,
    spread: f32,
    labels: CompassLabels<'a>,
    snap_angle: Option<f32>,
    shift_snap_angle: Option<f32>,
    min: Option<f32>,
    max: Option<f32>,
}

impl<'a> CompassKnob<'a> {
    pub fn new(value: &'a mut f32) -> Self {
        Self::from_get_set(move |v: Option<f32>| {
            if let Some(v) = v {
                *value = v;
            }
            *value
        })
    }

    pub fn from_get_set(get_set_value: impl 'a + FnMut(Option<f32>) -> f32) -> Self {
        Self {
            get_set_value: Box::new(get_set_value),
            mode: KnobMode::Unsigned,
            width: 256.0,
            height: 48.0,
            spread: TAU / 2.0,
            labels: CompassLabels(["N", "E", "S", "W"]),
            snap_angle: None,
            shift_snap_angle: Some(TAU / 36.0),
            min: None,
            max: None,
        }
    }

    pub fn mode(mut self, mode: KnobMode) -> Self {
        self.mode = mode;
        self
    }

    pub fn width(mut self, width: impl Into<f32>) -> Self {
        self.width = width.into();
        self
    }

    pub fn height(mut self, height: impl Into<f32>) -> Self {
        self.height = height.into();
        self
    }

    pub fn spread(mut self, spread: impl Into<f32>) -> Self {
        self.spread = spread.into();
        self
    }

    pub fn labels(mut self, labels: CompassLabels<'a>) -> Self {
        self.labels = labels;
        self
    }

    pub fn min(mut self, min: Option<f32>) -> Self {
        self.min = min;
        self
    }

    pub fn max(mut self, max: Option<f32>) -> Self {
        self.max = max;
        self
    }

    pub fn snap_angle(mut self, snap_angle: Option<f32>) -> Self {
        self.snap_angle = snap_angle;
        self
    }

    pub fn shift_snap_angle(mut self, shift_snap_angle: Option<f32>) -> Self {
        self.shift_snap_angle = shift_snap_angle;
        self
    }
}

impl CompassKnob<'_> {}

impl<'a> Widget for CompassKnob<'a> {
    fn ui(mut self, ui: &mut Ui) -> Response {
        let desired_size = egui::vec2(self.width, self.height);
        let (rect, mut response) =
            ui.allocate_exact_size(desired_size, egui::Sense::click_and_drag());

        let constrain_value = |mut value| {
            if self.mode == KnobMode::Signed {
                value = normalized_angle(value);
            }

            if self.mode == KnobMode::Unsigned {
                value = normalized_angle_unsigned(value);
            }

            if let Some(min) = self.min {
                value = value.max(min);
            }

            if let Some(max) = self.max {
                value = value.min(max);
            }

            value
        };

        if response.dragged() {
            let new_value =
                get(&mut self.get_set_value) - response.drag_delta().x / rect.width() * self.spread;
            set(&mut self.get_set_value, constrain_value(new_value));
            response.mark_changed();
        }

        if response.drag_released() {
            if let Some(angle) = if ui.input().modifiers.shift_only() {
                self.shift_snap_angle
            } else {
                self.snap_angle
            } {
                assert!(angle > 0.0, "non-positive snap angles are not supported");
                let new_value = (get(&mut self.get_set_value) / angle).round() * angle;
                set(&mut self.get_set_value, constrain_value(new_value));
                response.mark_changed();
            }
        }

        let value = get(&mut self.get_set_value);
        let map_angle_to_screen = |angle: f32| {
            rect.center().x - (value - angle) * (rect.width() / self.spread)
        };

        if ui.is_rect_visible(rect) {
            let visuals = *ui.style().interact(&response);

            ui.painter().rect(
                rect,
                visuals.rounding,
                ui.style().visuals.extreme_bg_color,
                ui.style().visuals.noninteractive().fg_stroke,
            );

            ui.set_clip_rect(rect);

            ui.painter().add(Shape::convex_polygon(
                vec![
                    rect.center(),
                    rect.center() - vec2(self.height / 6.0, self.height / 4.0),
                    rect.center() - vec2(-self.height / 6.0, self.height / 4.0),
                ],
                visuals.bg_fill,
                visuals.fg_stroke,
            ));

            ui.painter().text(
                rect.center_top(),
                Align2::CENTER_TOP,
                format!("{:.0}°", get(&mut self.get_set_value).to_degrees()),
                FontId::new(self.height / 4.0, FontFamily::Proportional),
                visuals.text_color(),
            );

            let left_degrees =
                (((get(&mut self.get_set_value) - (self.spread / 2.0)).to_degrees() / 10.0).floor()
                    * 10.0) as isize;

            let right_degrees =
                (((get(&mut self.get_set_value) + (self.spread / 2.0)).to_degrees() / 10.0).ceil()
                    * 10.0) as isize;

            for degree in (left_degrees..=right_degrees).step_by(10) {
                let tick_x = map_angle_to_screen((degree as f32).to_radians());

                let tick_height = if degree % 90 == 0 {
                    1.0
                } else if degree % 30 == 0 {
                    0.75
                } else {
                    0.5
                };

                ui.painter().line_segment(
                    [
                        pos2(tick_x, rect.top() + self.height * 0.5),
                        pos2(
                            tick_x,
                            rect.top() + self.height * 0.5 + self.height * 0.25 * tick_height,
                        ),
                    ],
                    ui.style().visuals.noninteractive().fg_stroke,
                );

                if degree % 90 == 0 {
                    ui.painter().text(
                        pos2(tick_x, rect.bottom()),
                        Align2::CENTER_BOTTOM,
                        self.labels.0[((((degree / 90) % 4) + 4) % 4) as usize],
                        FontId::new(self.height / 4.0, FontFamily::Proportional),
                        ui.style().visuals.text_color(),
                    );
                }
            }

            let paint_stop = |angle: f32| {
                let stop_x = map_angle_to_screen(angle);

                ui.painter().line_segment(
                    [pos2(stop_x, rect.top()), pos2(stop_x, rect.bottom())],
                    ui.style().visuals.noninteractive().fg_stroke,
                );
            };

            if let Some(min) = self.min {
                paint_stop(min);
            }

            if let Some(max) = self.max {
                paint_stop(max);
            }
        }

        response
    }
}
