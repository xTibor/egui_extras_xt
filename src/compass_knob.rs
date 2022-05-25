use std::f32::consts::TAU;

use eframe::egui::{self, Response, Ui, Widget};
use eframe::emath::{normalized_angle, pos2, vec2, Align2, Rect, Vec2};
use eframe::epaint::color::tint_color_towards;
use eframe::epaint::{Color32, FontFamily, FontId, Shape, Stroke};
use itertools::Itertools;

use crate::common::{normalized_angle_unsigned_excl, normalized_angle_unsigned_incl, KnobMode};

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
pub enum CompassKnobMarkerShape {
    Square,
    Circle,
    RightArrow,
    UpArrow,
    LeftArrow,
    DownArrow,
    Diamond,
    Star(usize, f32),
    Emoji(char),
}

pub struct CompassKnobMarker<'a> {
    angle: f32,
    shape: CompassKnobMarkerShape,
    label: Option<&'a str>,
    color: Color32,
}

impl<'a> CompassKnobMarker<'a> {
    pub fn new(angle: f32) -> Self {
        Self {
            angle: normalized_angle_unsigned_excl(angle),
            shape: CompassKnobMarkerShape::Square,
            label: None,
            color: Color32::GRAY,
        }
    }

    pub fn shape(mut self, shape: CompassKnobMarkerShape) -> Self {
        self.shape = shape;
        self
    }

    pub fn label(mut self, label: &'a str) -> Self {
        self.label = Some(label);
        self
    }

    pub fn color(mut self, color: Color32) -> Self {
        self.color = color;
        self
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
    snap: Option<f32>,
    shift_snap: Option<f32>,
    min: Option<f32>,
    max: Option<f32>,
    animated: bool,
    markers: &'a [CompassKnobMarker<'a>],
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
            snap: None,
            shift_snap: Some(TAU / 36.0),
            min: None,
            max: None,
            animated: false,
            markers: &[],
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

    pub fn snap(mut self, snap: Option<f32>) -> Self {
        self.snap = snap;
        self
    }

    pub fn shift_snap(mut self, shift_snap: Option<f32>) -> Self {
        self.shift_snap = shift_snap;
        self
    }

    pub fn animated(mut self, animated: bool) -> Self {
        self.animated = animated;
        self
    }

    pub fn markers(mut self, markers: &'a [CompassKnobMarker]) -> Self {
        self.markers = markers;
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

            let map_angle_to_screen =
                |angle: f32| rect.center().x - (value - angle) * (rect.width() / self.spread);

            ui.painter().rect(
                rect,
                visuals.rounding,
                ui.style().visuals.extreme_bg_color,
                ui.style().visuals.noninteractive().fg_stroke,
            );

            ui.set_clip_rect(rect);

            {
                let paint_marker = |angle,
                                    label: Option<&str>,
                                    text_color,
                                    marker_shape,
                                    marker_color,
                                    marker_stroke| {
                    // Early exit when the marker is outside of the bounds of the widget,
                    // plus some safety margin to avoid markers abruptly popping in from the sides.
                    {
                        let safety_margin = rect.width() / 4.0;
                        if !((rect.left() - safety_margin)..=(rect.right() + safety_margin))
                            .contains(&map_angle_to_screen(angle))
                        {
                            return;
                        }
                    }

                    // Draw marker shape
                    {
                        let marker_rect = {
                            let center =
                                pos2(map_angle_to_screen(angle), rect.top() + self.height * 0.375);
                            Rect::from_center_size(center, Vec2::splat(self.height * 0.25))
                        };

                        match marker_shape {
                            CompassKnobMarkerShape::Square => {
                                ui.painter()
                                    .rect(marker_rect, 0.0, marker_color, marker_stroke);
                            }
                            CompassKnobMarkerShape::Circle => {
                                ui.painter().rect(
                                    marker_rect,
                                    marker_rect.width() / 2.0,
                                    marker_color,
                                    marker_stroke,
                                );
                            }
                            CompassKnobMarkerShape::RightArrow => {
                                let marker_rect = Rect::from_center_size(
                                    marker_rect.center(),
                                    marker_rect.size() * vec2(3.0f32.sqrt() / 2.0, 1.0),
                                );

                                ui.painter().add(Shape::convex_polygon(
                                    vec![
                                        marker_rect.right_center(),
                                        marker_rect.left_bottom(),
                                        marker_rect.left_top(),
                                    ],
                                    marker_color,
                                    marker_stroke,
                                ));
                            }
                            CompassKnobMarkerShape::UpArrow => {
                                let marker_rect = Rect::from_center_size(
                                    marker_rect.center(),
                                    marker_rect.size() * vec2(1.0, 3.0f32.sqrt() / 2.0),
                                );

                                ui.painter().add(Shape::convex_polygon(
                                    vec![
                                        marker_rect.center_top(),
                                        marker_rect.right_bottom(),
                                        marker_rect.left_bottom(),
                                    ],
                                    marker_color,
                                    marker_stroke,
                                ));
                            }
                            CompassKnobMarkerShape::LeftArrow => {
                                let marker_rect = Rect::from_center_size(
                                    marker_rect.center(),
                                    marker_rect.size() * vec2(3.0f32.sqrt() / 2.0, 1.0),
                                );

                                ui.painter().add(Shape::convex_polygon(
                                    vec![
                                        marker_rect.left_center(),
                                        marker_rect.right_top(),
                                        marker_rect.right_bottom(),
                                    ],
                                    marker_color,
                                    marker_stroke,
                                ));
                            }
                            CompassKnobMarkerShape::DownArrow => {
                                let marker_rect = Rect::from_center_size(
                                    marker_rect.center(),
                                    marker_rect.size() * vec2(1.0, 3.0f32.sqrt() / 2.0),
                                );

                                ui.painter().add(Shape::convex_polygon(
                                    vec![
                                        marker_rect.left_top(),
                                        marker_rect.right_top(),
                                        marker_rect.center_bottom(),
                                    ],
                                    marker_color,
                                    marker_stroke,
                                ));
                            }
                            CompassKnobMarkerShape::Diamond => {
                                ui.painter().add(Shape::convex_polygon(
                                    vec![
                                        marker_rect.center_top(),
                                        marker_rect.right_center(),
                                        marker_rect.center_bottom(),
                                        marker_rect.left_center(),
                                    ],
                                    marker_color,
                                    marker_stroke,
                                ));
                            }
                            CompassKnobMarkerShape::Star(rays, ratio) => {
                                assert!(rays >= 2, "star-shaped markers must have at least 2 rays");
                                assert!(
                                    (0.0..=1.0).contains(&ratio),
                                    "ray ratio of star-shaped markers must be normalized"
                                );

                                let outer_radius = marker_rect.width() * 0.5;
                                let inner_radius = outer_radius * ratio;
                                let star_rotation = -TAU * 0.25;

                                let outer_points = (0..rays).map(|point_index| {
                                    marker_rect.center()
                                        + Vec2::angled(
                                            star_rotation
                                                + TAU * ((point_index as f32 + 0.0) / rays as f32),
                                        ) * outer_radius
                                });

                                let inner_points = (0..rays).map(|point_index| {
                                    marker_rect.center()
                                        + Vec2::angled(
                                            star_rotation
                                                + TAU * ((point_index as f32 + 0.5) / rays as f32),
                                        ) * inner_radius
                                });

                                // TODO: Broken polygon renderer
                                // https://github.com/emilk/egui/issues/513
                                ui.painter().add(Shape::convex_polygon(
                                    outer_points.interleave(inner_points).collect_vec(),
                                    marker_color,
                                    marker_stroke,
                                ));
                            }
                            CompassKnobMarkerShape::Emoji(emoji) => {
                                ui.painter().text(
                                    marker_rect.center(),
                                    Align2::CENTER_CENTER,
                                    emoji,
                                    FontId::new(marker_rect.height(), FontFamily::Proportional),
                                    marker_color,
                                );
                            }
                        }
                    }

                    // Draw marker text label
                    {
                        let label_center =
                            pos2(map_angle_to_screen(angle), rect.top() + self.height * 0.125);

                        if let Some(label) = label {
                            ui.painter().text(
                                label_center,
                                Align2::CENTER_CENTER,
                                label,
                                FontId::new(self.height / 4.0, FontFamily::Proportional),
                                text_color,
                            );
                        }
                    }
                };

                let start_tau = ((value - (self.spread.abs() / 2.0)) / TAU).floor() as isize;
                let end_tau = ((value + (self.spread.abs() / 2.0)) / TAU).ceil() as isize;

                for tau in start_tau..=end_tau {
                    for marker in self.markers.iter() {
                        let tinted_color =
                            tint_color_towards(marker.color, ui.style().visuals.text_color());

                        paint_marker(
                            (tau as f32 * TAU) + marker.angle,
                            marker.label,
                            tinted_color,
                            marker.shape,
                            marker.color,
                            Stroke::new(1.0, tinted_color),
                        );
                    }
                }

                paint_marker(
                    value,
                    Some(&format!("{:.0}°", value.to_degrees())),
                    visuals.text_color(),
                    CompassKnobMarkerShape::DownArrow,
                    visuals.bg_fill,
                    visuals.fg_stroke,
                );
            }

            {
                let round_bounds_to = 10.0;

                let start_degrees =
                    (((value - (self.spread.abs() / 2.0)).to_degrees() / round_bounds_to).floor()
                        * round_bounds_to) as isize;

                let end_degrees =
                    (((value + (self.spread.abs() / 2.0)).to_degrees() / round_bounds_to).ceil()
                        * round_bounds_to) as isize;

                for degree in (start_degrees..=end_degrees).step_by(5) {
                    let tick_x = map_angle_to_screen((degree as f32).to_radians());

                    let tick_position = pos2(tick_x, rect.top() + (self.height * 0.5));
                    let tick_size = vec2(0.0, self.height * 0.25);

                    let tick_label_center = pos2(tick_x, rect.top() + (self.height * 0.875));

                    let (tick_scale, tick_label) = if degree % 90 == 0 {
                        let label_index = (degree / 90).rem_euclid(4) as usize;
                        (1.0, Some(self.labels[label_index]))
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
                        [tick_position, tick_position + tick_size * tick_scale],
                        ui.style().visuals.noninteractive().fg_stroke,
                    );

                    if let Some(tick_label) = tick_label {
                        ui.painter().text(
                            tick_label_center,
                            Align2::CENTER_CENTER,
                            tick_label,
                            FontId::new(self.height / 4.0, FontFamily::Proportional),
                            ui.style().visuals.text_color(),
                        );
                    }
                }
            }

            {
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
        }

        response
    }
}
