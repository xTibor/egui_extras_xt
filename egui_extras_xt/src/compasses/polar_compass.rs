use std::f32::consts::TAU;

use ecolor::tint_color_towards;
use egui::{
    lerp, Align2, FontFamily, FontId, Pos2, Rect, Response, Sense, Shape, Stroke, StrokeKind, Ui,
    Vec2, Widget,
};
use emath::normalized_angle;

use strum::{Display, EnumIter};

use crate::common::{
    snap_wrap_constrain_angle, Orientation, RotatedText, SymLog, Winding, WrapMode,
};
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

#[derive(Clone, Copy, Debug, Display, EnumIter, Eq, PartialEq)]
pub enum PolarCompassOverflow {
    #[strum(to_string = "Clip")]
    Clip,

    #[strum(to_string = "Saturate")]
    Saturate,
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
    animated: bool,
    axis_labels: CompassAxisLabels,
    axis_label_height: f32,
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
    markers: &'a [CompassMarker<'a>],
    default_marker_color: DefaultCompassMarkerColor,
    default_marker_shape: CompassMarkerShape,
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
            interactive: true,
            orientation: Orientation::Top,
            winding: Winding::Clockwise,
            overflow: PolarCompassOverflow::Saturate,
            diameter: 256.0,
            wrap: WrapMode::Unsigned,
            min: None,
            max: None,
            snap: None,
            shift_snap: Some(15.0f32.to_radians()),
            animated: false,
            axis_labels: ["N", "E", "S", "W"].into(),
            axis_label_height: 24.0,
            max_distance: 10000.0,
            scale_log_base: 10.0,
            scale_log_mult: 1.0,
            marker_near_size: 16.0,
            marker_far_size: 8.0,
            show_axes: true,
            show_rings: true,
            show_cursor: true,
            show_marker_labels: true,
            show_marker_lines: true,
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
        self.diameter = diameter;
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

    pub fn animated(mut self, animated: bool) -> Self {
        self.animated = animated;
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

    pub fn axis_labels(mut self, axis_labels: CompassAxisLabels) -> Self {
        self.axis_labels = axis_labels;
        self
    }

    pub fn axis_label_height(mut self, axis_label_height: f32) -> Self {
        assert!(axis_label_height > 0.0);
        self.axis_label_height = axis_label_height;
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

    pub fn markers(mut self, markers: &'a [CompassMarker<'a>]) -> Self {
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

impl<'a> Widget for PolarCompass<'a> {
    fn ui(mut self, ui: &mut Ui) -> Response {
        let desired_size = Vec2::splat(self.diameter + self.axis_label_height * 2.0);

        let (rect, mut response) = ui.allocate_exact_size(
            desired_size,
            if self.interactive {
                Sense::click_and_drag()
            } else {
                Sense::hover()
            },
        );

        let rotation_matrix = self.orientation.rot2();

        if response.drag_started() {
            let value_before_drag = get(&mut self.get_set_value);
            ui.memory_mut(|memory| memory.data.insert_temp(response.id, value_before_drag));
        }

        if response.drag_stopped() {
            ui.memory_mut(|memory| memory.data.remove::<f32>(response.id));
        }

        if response.dragged() {
            let screen_pos_to_angle = |pos: Pos2| {
                -(rotation_matrix * (rect.center() - pos)).angle() * self.winding.to_float()
            };

            let value_before_drag =
                ui.memory_mut(|memory| memory.data.get_temp::<f32>(response.id).unwrap());
            let prev_value = get(&mut self.get_set_value);

            let mut new_value = normalized_angle(
                screen_pos_to_angle(response.interact_pointer_pos().unwrap())
                    - screen_pos_to_angle(ui.input(|input| input.pointer.press_origin().unwrap()))
                    + value_before_drag,
            );

            new_value = snap_wrap_constrain_angle(
                prev_value,
                new_value,
                if ui.input(|input| input.modifiers.shift_only()) {
                    self.shift_snap
                } else {
                    self.snap
                },
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

            let value = if self.animated {
                ui.ctx().animate_value_with_time(
                    response.id,
                    get(&mut self.get_set_value),
                    ui.style().animation_time,
                )
            } else {
                get(&mut self.get_set_value)
            };

            {
                ui.painter().circle(
                    rect.center(),
                    radius,
                    ui.style().visuals.extreme_bg_color, // TODO: Semantically correct color
                    visuals.fg_stroke,                   // TODO: Semantically correct color
                );
            }

            if self.show_rings {
                let max_log = (self.max_distance / self.scale_log_mult).symlog(self.scale_log_base);
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
                for (axis_index, axis_label) in self.axis_labels.inner.iter().enumerate() {
                    let axis_angle =
                        axis_index as f32 * (TAU / (self.axis_labels.inner.len() as f32));

                    ui.painter().add(Shape::line_segment(
                        [
                            rect.center(),
                            rect.center() + angle_to_direction(axis_angle) * radius,
                        ],
                        visuals.fg_stroke, // TODO: Semantically correct color
                    ));

                    ui.painter().rotated_text(
                        rect.center()
                            + angle_to_direction(axis_angle)
                                * (radius + self.axis_label_height / 2.0),
                        Align2::CENTER_CENTER,
                        axis_label,
                        FontId::new(self.axis_label_height, FontFamily::Proportional),
                        visuals.text_color(), // TODO: Semantically correct color
                        angle_to_direction(axis_angle).angle() + (TAU / 4.0),
                    );
                }
            }

            for marker in self.markers {
                let marker_distance = marker.distance.expect("marker has no distance");

                if (marker_distance > self.max_distance)
                    && (self.overflow == PolarCompassOverflow::Clip)
                {
                    continue;
                }

                let marker_color = marker
                    .color
                    .unwrap_or_else(|| self.default_marker_color.color(ui, marker));

                let marker_stroke = {
                    let stroke_color =
                        tint_color_towards(marker_color, ui.style().visuals.text_color());
                    Stroke::new(1.0, stroke_color)
                };

                let marker_stroke_kind = StrokeKind::Middle;

                let max_log = (self.max_distance / self.scale_log_mult).symlog(self.scale_log_base);
                let marker_log =
                    (marker_distance / self.scale_log_mult).symlog(self.scale_log_base);
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

                let marker_shape = marker.shape.unwrap_or(self.default_marker_shape);

                marker_shape.paint(
                    ui,
                    Rect::from_center_size(marker_center, Vec2::splat(marker_size)),
                    marker_color,
                    marker_stroke,
                    marker_stroke_kind,
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
