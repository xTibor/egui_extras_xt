use std::f32::consts::TAU;

use egui::color::tint_color_towards;
use egui::{
    lerp, Align2, Color32, FontFamily, FontId, Rect, Response, Sense, Shape, Stroke, Ui, Vec2,
    Widget,
};

use crate::common::{normalized_angle_unsigned_excl, snap_wrap_constrain_angle, Winding};
use crate::compass::{CompassLabels, CompassMarkerShape};
use crate::{Orientation, WrapMode};

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

#[derive(Copy, Clone, PartialEq)]
pub enum PolarCompassOverflow {
    Clip,
    Saturate,
}

// ----------------------------------------------------------------------------

pub struct PolarCompassMarker<'a> {
    angle: f32,
    distance: f32,
    shape: CompassMarkerShape,
    label: Option<&'a str>,
    color: Option<Color32>,
}

impl<'a> PolarCompassMarker<'a> {
    pub fn new(angle: f32, distance: f32) -> Self {
        Self {
            angle: normalized_angle_unsigned_excl(angle),
            distance,
            shape: CompassMarkerShape::Square,
            label: None,
            color: None,
        }
    }

    pub fn shape(mut self, shape: CompassMarkerShape) -> Self {
        self.shape = shape;
        self
    }

    pub fn label(mut self, label: &'a str) -> Self {
        self.label = Some(label);
        self
    }

    pub fn color(mut self, color: Color32) -> Self {
        self.color = Some(color);
        self
    }
}

// ----------------------------------------------------------------------------

#[must_use = "You should put this widget in an ui with `ui.add(widget);`"]
pub struct PolarCompass<'a> {
    get_set_value: GetSetValue<'a>,
    interactive: bool,
    orientation: Orientation,
    winding: Winding,
    overflow: PolarCompassOverflow,
    diameter: f32,
    wrap: WrapMode,
    min: Option<f32>,
    max: Option<f32>,
    snap: Option<f32>,
    shift_snap: Option<f32>,
    labels: CompassLabels<'a>,
    label_height: f32,
    max_distance: f32,
    scale_log_base: f32,
    scale_log_mult: f32,
    marker_near_size: f32,
    marker_far_size: f32,
    show_axes: bool,
    show_rings: bool,
    show_cursor: bool,
    show_marker_labels: bool,
    show_marker_lines: bool,
    markers: &'a [PolarCompassMarker<'a>],
}

impl<'a> PolarCompass<'a> {
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
            interactive: false,
            orientation: Orientation::Top,
            winding: Winding::Clockwise,
            overflow: PolarCompassOverflow::Saturate,
            diameter: 256.0,
            wrap: WrapMode::Unsigned,
            min: None,
            max: None,
            snap: None,
            shift_snap: Some(TAU / 24.0),
            labels: ["N", "E", "S", "W"],
            label_height: 48.0,
            max_distance: 10000.0,
            scale_log_base: 10.0,
            scale_log_mult: 1.0,
            marker_near_size: 16.0,
            marker_far_size: 8.0,
            show_axes: true,
            show_rings: true,
            show_cursor: true,
            show_marker_labels: false,
            show_marker_lines: false,
            markers: &[],
        }
    }

    pub fn interactive(mut self, interactive: bool) -> Self {
        self.interactive = interactive;
        self
    }

    pub fn orientation(mut self, orientation: Orientation) -> Self {
        self.orientation = orientation;
        self
    }

    pub fn winding(mut self, winding: Winding) -> Self {
        self.winding = winding;
        self
    }

    pub fn overflow(mut self, overflow: PolarCompassOverflow) -> Self {
        self.overflow = overflow;
        self
    }

    pub fn diameter(mut self, diameter: f32) -> Self {
        assert!(diameter > 0.0);
        self.diameter = diameter.into();
        self
    }

    pub fn wrap(mut self, wrap: WrapMode) -> Self {
        self.wrap = wrap;
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

    pub fn scale_log_base(mut self, scale_log_base: f32) -> Self {
        assert!(scale_log_base > 1.0);
        self.scale_log_base = scale_log_base;
        self
    }

    pub fn scale_log_mult(mut self, scale_log_mult: f32) -> Self {
        assert!(scale_log_mult > 0.0);
        self.scale_log_mult = scale_log_mult;
        self
    }

    pub fn labels(mut self, labels: CompassLabels<'a>) -> Self {
        self.labels = labels;
        self
    }

    pub fn label_height(mut self, label_height: f32) -> Self {
        assert!(label_height > 0.0);
        self.label_height = label_height;
        self
    }

    pub fn max_distance(mut self, max_distance: f32) -> Self {
        assert!(max_distance >= 0.0);
        self.max_distance = max_distance;
        self
    }

    pub fn ring_count(mut self, ring_count: usize) -> Self {
        assert!(ring_count > 0);
        self.scale_log_base = (self.max_distance.ln() / ring_count as f32).exp();
        self.scale_log_mult = 1.0;
        self
    }

    pub fn marker_near_size(mut self, marker_near_size: f32) -> Self {
        assert!(marker_near_size > 0.0);
        self.marker_near_size = marker_near_size;
        self
    }

    pub fn marker_far_size(mut self, marker_far_size: f32) -> Self {
        assert!(marker_far_size > 0.0);
        self.marker_far_size = marker_far_size;
        self
    }

    pub fn show_axes(mut self, show_axes: bool) -> Self {
        self.show_axes = show_axes;
        self
    }

    pub fn show_rings(mut self, show_rings: bool) -> Self {
        self.show_rings = show_rings;
        self
    }

    pub fn show_cursor(mut self, show_cursor: bool) -> Self {
        self.show_cursor = show_cursor;
        self
    }

    pub fn show_marker_labels(mut self, show_marker_labels: bool) -> Self {
        self.show_marker_labels = show_marker_labels;
        self
    }

    pub fn show_marker_lines(mut self, show_marker_lines: bool) -> Self {
        self.show_marker_lines = show_marker_lines;
        self
    }

    pub fn markers(mut self, markers: &'a [PolarCompassMarker<'a>]) -> Self {
        self.markers = markers;
        self
    }
}

impl<'a> Widget for PolarCompass<'a> {
    fn ui(mut self, ui: &mut Ui) -> Response {
        let desired_size = Vec2::splat(self.diameter + self.label_height * 2.0);

        let (rect, mut response) = ui.allocate_exact_size(
            desired_size,
            if self.interactive {
                Sense::click_and_drag()
            } else {
                Sense::hover()
            },
        );

        let rotation_matrix = self.orientation.rot2();

        if response.clicked() || response.dragged() {
            let prev_value = get(&mut self.get_set_value);
            let mut new_value = -(rotation_matrix
                * (rect.center() - response.interact_pointer_pos().unwrap()))
            .angle()
                * self.winding.to_float();

            new_value = snap_wrap_constrain_angle(
                prev_value,
                new_value,
                ui.input().modifiers.shift_only(),
                self.snap,
                self.shift_snap,
                self.wrap,
                self.min,
                self.max,
            );

            set(&mut self.get_set_value, new_value);
            response.mark_changed();
        }

        if ui.is_rect_visible(rect) {
            let visuals = *ui.style().interact(&response);
            let radius = self.diameter / 2.0;

            let value = get(&mut self.get_set_value);

            {
                ui.painter().circle(
                    rect.center(),
                    radius,
                    ui.style().visuals.extreme_bg_color, // TODO: Semantically correct color
                    visuals.fg_stroke,                   // TODO: Semantically correct color
                );
            }

            if self.show_rings {
                let max_log = (self.max_distance / self.scale_log_mult).log(self.scale_log_base);
                assert!(max_log < 256.0); // Prevent accidental OoM deaths during development

                // No off-by-one bugs here, non-inclusive range end is used to
                // avoid double rendering the outermost ring.
                for i in 1..max_log.ceil() as usize {
                    ui.painter().circle_stroke(
                        rect.center(),
                        radius * (i as f32 / max_log),
                        ui.style().visuals.noninteractive().fg_stroke, // TODO: Semantically correct color
                    );
                }
            }

            let angle_to_direction = |angle: f32| {
                rotation_matrix * Vec2::angled((angle - value) * self.winding.to_float())
            };

            if self.show_cursor {
                ui.painter().add(Shape::dashed_line(
                    &[
                        rect.center(),
                        rect.center() + rotation_matrix * Vec2::RIGHT * radius,
                    ],
                    ui.style().visuals.noninteractive().fg_stroke, // TODO: Semantically correct color
                    2.0,
                    2.0,
                ));
            }

            if self.show_axes {
                for (axis_index, axis_label) in self.labels.iter().enumerate() {
                    let axis_angle = axis_index as f32 * (TAU / (self.labels.len() as f32));

                    ui.painter().add(Shape::line_segment(
                        [
                            rect.center(),
                            rect.center() + angle_to_direction(axis_angle) * radius,
                        ],
                        visuals.fg_stroke, // TODO: Semantically correct color
                    ));

                    ui.painter().text(
                        rect.center()
                            + angle_to_direction(axis_angle) * (radius + self.label_height / 2.0),
                        Align2::CENTER_CENTER,
                        axis_label,
                        FontId::new(self.label_height, FontFamily::Proportional),
                        visuals.text_color(), // TODO: Semantically correct color
                    );
                }
            }

            for marker in self.markers {
                if (marker.distance > self.max_distance)
                    && (self.overflow == PolarCompassOverflow::Clip)
                {
                    continue;
                }

                let marker_color = marker.color.unwrap_or(ui.style().visuals.text_color());

                let marker_stroke = {
                    let stroke_color =
                        tint_color_towards(marker_color, ui.style().visuals.text_color());
                    Stroke::new(1.0, stroke_color)
                };

                let max_log = (self.max_distance / self.scale_log_mult).log(self.scale_log_base);
                let marker_log = (marker.distance / self.scale_log_mult).log(self.scale_log_base);
                let marker_t = (marker_log / max_log).clamp(0.0, 1.0);

                let marker_center =
                    rect.center() + angle_to_direction(marker.angle) * (radius * marker_t);
                let marker_size = lerp(self.marker_near_size..=self.marker_far_size, marker_t);

                if self.show_marker_lines {
                    ui.painter().add(Shape::dashed_line(
                        &[rect.center(), marker_center],
                        marker_stroke,
                        2.0,
                        4.0,
                    ));
                }

                marker.shape.paint(
                    ui,
                    Rect::from_center_size(marker_center, Vec2::splat(marker_size)),
                    marker_color,
                    marker_stroke,
                );

                if self.show_marker_labels {
                    let label_center = marker_center + Vec2::DOWN * marker_size;

                    if let Some(marker_label) = marker.label {
                        ui.painter().text(
                            label_center,
                            Align2::CENTER_CENTER,
                            marker_label,
                            FontId::new(marker_size, FontFamily::Proportional),
                            marker_color,
                        );
                    }
                }
            }
        }

        response
    }
}
