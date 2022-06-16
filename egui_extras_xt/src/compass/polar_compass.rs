use std::f32::consts::TAU;

use egui::{Color32, Response, Shape, Ui, Vec2, Widget};

use crate::common::{normalized_angle_unsigned_excl, Winding, WrapMode};
use crate::compass::{CompassLabels, CompassMarkerShape};
use crate::Orientation;

// ----------------------------------------------------------------------------

pub struct PolarCompassMarker {
    angle: f32,
    distance: f32,
    shape: CompassMarkerShape,
    color: Option<Color32>,
}

impl<'a> PolarCompassMarker {
    pub fn new(angle: f32, distance: f32) -> Self {
        Self {
            angle: normalized_angle_unsigned_excl(angle),
            distance,
            shape: CompassMarkerShape::Square,
            color: None,
        }
    }

    pub fn shape(mut self, shape: CompassMarkerShape) -> Self {
        self.shape = shape;
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
    orientation: Orientation,
    winding: Winding,
    diameter: f32,
    labels: CompassLabels<'a>,
    label_height: f32,
    max_distance: f32,
    ring_count: usize,
    markers: &'a [PolarCompassMarker],
}

impl<'a> PolarCompass<'a> {
    pub fn new() -> Self {
        Self {
            orientation: Orientation::Top,
            winding: Winding::Clockwise,
            diameter: 256.0,
            labels: ["N", "E", "S", "W"],
            label_height: 48.0,
            max_distance: 10000.0,
            ring_count: 4,
            markers: &[],
        }
    }

    pub fn orientation(mut self, orientation: Orientation) -> Self {
        self.orientation = orientation;
        self
    }

    pub fn winding(mut self, winding: Winding) -> Self {
        self.winding = winding;
        self
    }

    pub fn diameter(mut self, diameter: impl Into<f32>) -> Self {
        self.diameter = diameter.into();
        self
    }

    pub fn labels(mut self, labels: CompassLabels<'a>) -> Self {
        self.labels = labels;
        self
    }

    pub fn label_height(mut self, label_height: impl Into<f32>) -> Self {
        self.label_height = label_height.into();
        self
    }

    pub fn max_distance(mut self, max_distance: impl Into<f32>) -> Self {
        self.max_distance = max_distance.into();
        self
    }

    pub fn ring_count(mut self, ring_count: usize) -> Self {
        self.ring_count = ring_count;
        self
    }

    pub fn markers(mut self, markers: &'a [PolarCompassMarker]) -> Self {
        self.markers = markers;
        self
    }
}

impl<'a> Widget for PolarCompass<'a> {
    fn ui(mut self, ui: &mut Ui) -> Response {
        let desired_size = Vec2::splat(self.diameter + self.label_height * 2.0);
        let (rect, mut response) =
            ui.allocate_exact_size(desired_size, egui::Sense::click_and_drag());

        let rotation_matrix = self.orientation.rot2();

        if ui.is_rect_visible(rect) {
            let visuals = *ui.style().interact(&response);
            let radius = self.diameter / 2.0;

            {
                ui.painter().circle(
                    rect.center(),
                    radius,
                    ui.style().visuals.extreme_bg_color, // TODO: Semantically correct color
                    ui.style().visuals.noninteractive().fg_stroke, // TODO: Semantically correct color
                );

                for i in 1..self.ring_count {
                    ui.painter().circle_stroke(
                        rect.center(),
                        radius * (i as f32 / self.ring_count as f32),
                        ui.style().visuals.noninteractive().fg_stroke, // TODO: Semantically correct color
                    );
                }
            }

            let angle_to_direction =
                |angle: f32| rotation_matrix * Vec2::angled(angle * self.winding.to_float());

            {
                let paint_axis = |axis_angle| {
                    ui.painter().add(Shape::line_segment(
                        [
                            rect.center(),
                            rect.center() + angle_to_direction(axis_angle) * radius,
                        ],
                        ui.style().visuals.noninteractive().fg_stroke, // TODO: Semantically correct color
                    ));
                };

                for axis in 0..self.labels.len() {
                    paint_axis(axis as f32 * (TAU / (self.labels.len() as f32)));
                }
            }
        }

        response
    }
}
