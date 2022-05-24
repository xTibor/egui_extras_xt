use std::f32::consts::TAU;

use eframe::egui::{self, Response, Ui, Widget};
use eframe::emath::{normalized_angle, pos2, vec2, Align2, Vec2, Rect};
use eframe::epaint::color::tint_color_towards;
use eframe::epaint::{Color32, FontFamily, FontId, Shape, Stroke};

use crate::common::{normalized_angle_unsigned_incl, KnobMode};

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

pub type CompassLabels<'a> = [&'a str; 4];

// ----------------------------------------------------------------------------

#[derive(Clone, Copy)]
pub enum CompassKnobTargetShape {
    DownArrow,
    UpArrow,
    Square,
}

pub struct CompassKnobTarget<'a> {
    angle: f32,
    shape: CompassKnobTargetShape,
    label: Option<&'a str>,
    color: Color32,
}

impl<'a> CompassKnobTarget<'a> {
    pub fn new(
        angle: f32,
        shape: CompassKnobTargetShape,
        label: Option<&'a str>,
        color: Color32,
    ) -> Self {
        Self {
            angle,
            shape,
            label,
            color,
        }
    }
}

// ----------------------------------------------------------------------------

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
    animated: bool,
    targets: &'a [CompassKnobTarget<'a>],
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
            labels: ["N", "E", "S", "W"],
            snap_angle: None,
            shift_snap_angle: Some(TAU / 36.0),
            min: None,
            max: None,
            animated: false,
            targets: &[],
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

    pub fn animated(mut self, animated: bool) -> Self {
        self.animated = animated;
        self
    }

    pub fn targets(mut self, targets: &'a [CompassKnobTarget]) -> Self {
        self.targets = targets;
        self
    }
}

impl<'a> Widget for CompassKnob<'a> {
    fn ui(mut self, ui: &mut Ui) -> Response {
        let desired_size = egui::vec2(self.width, self.height);
        let (rect, mut response) =
            ui.allocate_exact_size(desired_size, egui::Sense::click_and_drag());

        let constrain_value = |mut value| {
            if self.mode == KnobMode::Signed {
                // Animations require inclusive normalization bounds (-PI..=PI)
                value = normalized_angle(value);
            }

            if self.mode == KnobMode::Unsigned {
                // Animations require inclusive normalization bounds (0..=TAU)
                value = normalized_angle_unsigned_incl(value);
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
            if self.animated {
                ui.ctx().clear_animations();
                ui.ctx()
                    .animate_value_with_time(response.id, get(&mut self.get_set_value), 0.1);
            }

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

        if ui.is_rect_visible(rect) {
            let visuals = *ui.style().interact(&response);

            let value = if self.animated && !response.dragged() {
                ui.ctx()
                    .animate_value_with_time(response.id, get(&mut self.get_set_value), 0.1)
            } else {
                get(&mut self.get_set_value)
            };

            let map_angle_to_screen =
                |angle: f32| rect.center().x - (value - angle) * (rect.width() / self.spread);

            ui.painter().rect(
                rect,
                visuals.rounding,
                ui.style().visuals.extreme_bg_color,
                ui.style().visuals.noninteractive().fg_stroke,
            );

            ui.set_clip_rect(rect);

            let paint_target = |angle,
                                label: Option<&str>,
                                arrow_shape,
                                text_color,
                                arrow_color,
                                arrow_stroke| {
                let target_x = map_angle_to_screen(angle);

                let label_center = pos2(target_x, rect.top() + self.height * 0.125);
                let arrow_center = pos2(target_x, rect.top() + self.height * 0.375);

                let arrow_size = self.height / 6.0;

                match arrow_shape {
                    CompassKnobTargetShape::DownArrow => {
                        ui.painter().add(Shape::convex_polygon(
                            vec![
                                arrow_center + Vec2::angled(TAU * (3.0 / 12.0)) * arrow_size,
                                arrow_center + Vec2::angled(TAU * (7.0 / 12.0)) * arrow_size,
                                arrow_center + Vec2::angled(TAU * (11.0 / 12.0)) * arrow_size,
                            ],
                            arrow_color,
                            arrow_stroke,
                        ));
                    }
                    CompassKnobTargetShape::UpArrow => {
                        ui.painter().add(Shape::convex_polygon(
                            vec![
                                arrow_center + Vec2::angled(TAU * (1.0 / 12.0)) * arrow_size,
                                arrow_center + Vec2::angled(TAU * (5.0 / 12.0)) * arrow_size,
                                arrow_center + Vec2::angled(TAU * (9.0 / 12.0)) * arrow_size,
                            ],
                            arrow_color,
                            arrow_stroke,
                        ));
                    }
                    CompassKnobTargetShape::Square => {
                        ui.painter().rect(Rect::from_center_size(arrow_center, Vec2::splat(arrow_size * 2.0f32.sqrt())), 0.0, arrow_color, arrow_stroke);
                    },
                }

                if let Some(label) = label {
                    ui.painter().text(
                        label_center,
                        Align2::CENTER_CENTER,
                        label,
                        FontId::new(self.height / 4.0, FontFamily::Proportional),
                        text_color,
                    );
                }
            };

            for &CompassKnobTarget {
                angle,
                shape,
                label,
                color,
            } in self.targets.iter()
            {
                let tinted_color = tint_color_towards(color, ui.style().visuals.text_color());

                paint_target(
                    angle,
                    label,
                    shape,
                    tinted_color,
                    color,
                    Stroke::new(1.0, tinted_color),
                );
            }

            paint_target(
                value,
                Some(&format!("{:.0}°", value.to_degrees())),
                CompassKnobTargetShape::DownArrow,
                visuals.text_color(),
                visuals.bg_fill,
                visuals.fg_stroke,
            );

            let round_bounds_to = 10.0;

            let start_degrees = (((value - (self.spread / 2.0)).to_degrees() / round_bounds_to)
                .floor()
                * round_bounds_to) as isize;

            let end_degrees = (((value + (self.spread / 2.0)).to_degrees() / round_bounds_to)
                .ceil()
                * round_bounds_to) as isize;

            for degree in (start_degrees..=end_degrees).step_by(5) {
                let tick_x = map_angle_to_screen((degree as f32).to_radians());

                let (tick_height, tick_label) = if degree % 90 == 0 {
                    let tick_label = self.labels[(degree / 90).rem_euclid(4) as usize];
                    (1.0, Some(tick_label))
                } else if degree % 30 == 0 {
                    (0.75, None)
                } else if degree % 10 == 0 {
                    (0.5, None)
                } else if degree % 5 == 0 {
                    (0.3, None)
                } else {
                    unreachable!()
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

                if let Some(tick_label) = tick_label {
                    ui.painter().text(
                        pos2(tick_x, rect.bottom()),
                        Align2::CENTER_BOTTOM,
                        tick_label,
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
