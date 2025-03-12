use std::f32::consts::TAU;

use ecolor::tint_color_towards;
use egui::{self, Response, Sense, StrokeKind, Ui, UiBuilder, Widget};
use emath::{normalized_angle, pos2, vec2, Align2, Rect, Vec2};
use epaint::{Color32, FontFamily, FontId, Stroke};

use crate::common::{normalized_angle_unsigned_incl, Winding, WrapMode};
use crate::compasses::{
    CompassAxisLabels, CompassMarker, CompassMarkerShape, DefaultCompassMarkerColor,
};

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
pub struct LinearCompass<'a> {
    get_set_value: GetSetValue<'a>,
    interactive: bool,
    wrap: WrapMode,
    winding: Winding,
    width: f32,
    height: f32,
    spread: f32,
    axis_labels: CompassAxisLabels,
    snap: Option<f32>,
    shift_snap: Option<f32>,
    min: Option<f32>,
    max: Option<f32>,
    animated: bool,
    show_cursor: bool,
    show_ticks: bool,
    show_axes: bool,
    markers: &'a [CompassMarker<'a>],
    default_marker_color: DefaultCompassMarkerColor,
    default_marker_shape: CompassMarkerShape,
}

impl<'a> LinearCompass<'a> {
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
            interactive: true,
            wrap: WrapMode::Unsigned,
            winding: Winding::Clockwise,
            width: 512.0,
            height: 48.0,
            spread: 180.0f32.to_radians(),
            axis_labels: ["N", "E", "S", "W"].into(),
            snap: None,
            shift_snap: Some(10.0f32.to_radians()),
            min: None,
            max: None,
            animated: false,
            show_cursor: true,
            show_ticks: true,
            show_axes: true,
            markers: &[],
            default_marker_color: DefaultCompassMarkerColor::HsvByAngle {
                hue_phase: 0.0,
                saturation: 1.0,
                value: 1.0,
            },
            default_marker_shape: CompassMarkerShape::Square,
        }
    }

    pub fn interactive(mut self, interactive: bool) -> Self {
        self.interactive = interactive;
        self
    }

    pub fn wrap(mut self, wrap: WrapMode) -> Self {
        self.wrap = wrap;
        self
    }

    pub fn winding(mut self, winding: Winding) -> Self {
        self.winding = winding;
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

    pub fn axis_labels(mut self, axis_labels: CompassAxisLabels) -> Self {
        self.axis_labels = axis_labels;
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

    pub fn show_cursor(mut self, show_cursor: bool) -> Self {
        self.show_cursor = show_cursor;
        self
    }

    pub fn show_ticks(mut self, show_ticks: bool) -> Self {
        self.show_ticks = show_ticks;
        self
    }

    pub fn show_axes(mut self, show_axes: bool) -> Self {
        self.show_axes = show_axes;
        self
    }

    pub fn markers(mut self, markers: &'a [CompassMarker]) -> Self {
        self.markers = markers;
        self
    }

    pub fn default_marker_color(mut self, default_marker_color: DefaultCompassMarkerColor) -> Self {
        self.default_marker_color = default_marker_color;
        self
    }

    pub fn default_marker_shape(mut self, default_marker_shape: CompassMarkerShape) -> Self {
        self.default_marker_shape = default_marker_shape;
        self
    }
}

impl<'a> Widget for LinearCompass<'a> {
    fn ui(mut self, ui: &mut Ui) -> Response {
        let desired_size = egui::vec2(self.width, self.height);

        let (rect, mut response) = ui.allocate_exact_size(
            desired_size,
            if self.interactive {
                Sense::click_and_drag()
            } else {
                Sense::hover()
            },
        );

        let mut child_ui = ui.new_child(UiBuilder::new().max_rect(rect).layout(*ui.layout()));
        child_ui.set_clip_rect(child_ui.clip_rect().intersect(rect));

        let constrain_value = |mut value| {
            if self.wrap == WrapMode::Signed {
                // Animations require inclusive normalization bounds (-PI..=PI)
                value = normalized_angle(value);
            }

            if self.wrap == WrapMode::Unsigned {
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
            let new_value = get(&mut self.get_set_value)
                - response.drag_delta().x / rect.width() * (self.spread * self.winding.to_float());
            set(&mut self.get_set_value, constrain_value(new_value));
            response.mark_changed();
        }

        if response.drag_stopped() {
            if self.animated {
                child_ui.ctx().clear_animations();
                child_ui.ctx().animate_value_with_time(
                    response.id,
                    get(&mut self.get_set_value),
                    0.1,
                );
            }

            if let Some(snap_angle) = if child_ui.input(|input| input.modifiers.shift_only()) {
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

        if child_ui.is_rect_visible(rect) {
            let visuals = *child_ui.style().interact(&response);

            let value = if self.animated && !response.dragged() {
                child_ui.ctx().animate_value_with_time(
                    response.id,
                    get(&mut self.get_set_value),
                    child_ui.style().animation_time,
                )
            } else {
                get(&mut self.get_set_value)
            };

            let map_angle_to_screen = |angle: f32| {
                rect.center().x
                    - (value - angle) * (rect.width() / (self.spread * self.winding.to_float()))
            };

            // Draw the widget background without clipping to avoid truncated outline strokes
            ui.painter().rect(
                rect,
                visuals.corner_radius,
                ui.style().visuals.extreme_bg_color,
                ui.style().visuals.noninteractive().fg_stroke,
                StrokeKind::Middle,
            );

            {
                let paint_marker = |child_ui: &mut Ui,
                                    angle: f32,
                                    label: Option<&str>,
                                    text_color: Color32,
                                    shape: CompassMarkerShape,
                                    fill: Color32,
                                    stroke: Stroke,
                                    stroke_kind: StrokeKind| {
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

                        shape.paint(child_ui, marker_rect, fill, stroke, stroke_kind);
                    }

                    // Draw marker text label
                    {
                        let label_center =
                            pos2(map_angle_to_screen(angle), rect.top() + self.height * 0.125);

                        if let Some(label) = label {
                            child_ui.painter().text(
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
                        let marker_color = marker
                            .color
                            .unwrap_or_else(|| self.default_marker_color.color(&child_ui, marker));

                        let marker_stroke = {
                            let stroke_color = tint_color_towards(
                                marker_color,
                                child_ui.style().visuals.text_color(),
                            );
                            Stroke::new(1.0, stroke_color)
                        };

                        let marker_stroke_kind = StrokeKind::Middle;

                        let marker_shape = marker.shape.unwrap_or(self.default_marker_shape);

                        paint_marker(
                            &mut child_ui,
                            (tau as f32 * TAU) + marker.angle,
                            marker.label,
                            marker_color,
                            marker_shape,
                            marker_color,
                            marker_stroke,
                            marker_stroke_kind,
                        );
                    }
                }

                if self.show_cursor {
                    paint_marker(
                        &mut child_ui,
                        value,
                        Some(&format!("{:.0}°", value.to_degrees())),
                        visuals.text_color(),
                        CompassMarkerShape::DownArrow,
                        visuals.bg_fill,
                        visuals.fg_stroke,
                        StrokeKind::Middle,
                    );
                }
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

                    let (tick_scale, tick_label, is_axis_tick) = if degree % 90 == 0 {
                        let axis_label_index = (degree / 90).rem_euclid(4) as usize;
                        (1.0, Some(&self.axis_labels.inner[axis_label_index]), true)
                    } else if degree % 30 == 0 {
                        (0.75, None, false)
                    } else if degree % 10 == 0 {
                        (0.5, None, false)
                    } else if degree % 5 == 0 {
                        (0.3, None, false)
                    } else {
                        unreachable!()
                    };

                    if self.show_ticks || (self.show_axes && is_axis_tick) {
                        child_ui.painter().line_segment(
                            [tick_position, tick_position + tick_size * tick_scale],
                            child_ui.style().visuals.noninteractive().fg_stroke,
                        );
                    }

                    if self.show_axes {
                        if let Some(tick_label) = tick_label {
                            child_ui.painter().text(
                                tick_label_center,
                                Align2::CENTER_CENTER,
                                tick_label,
                                FontId::new(self.height / 4.0, FontFamily::Proportional),
                                child_ui.style().visuals.text_color(),
                            );
                        }
                    }
                }
            }

            {
                let paint_stop = |angle: f32| {
                    let stop_x = map_angle_to_screen(angle);

                    child_ui.painter().line_segment(
                        [pos2(stop_x, rect.top()), pos2(stop_x, rect.bottom())],
                        child_ui.style().visuals.noninteractive().fg_stroke,
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
