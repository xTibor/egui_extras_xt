use std::f32::consts::TAU;

use eframe::egui::{self, Response, Ui, Widget};
use eframe::emath::Vec2;
use eframe::epaint::{Shape, Stroke};

use crate::common::{normalized_angle_unsigned_excl, KnobDirection, KnobMode, KnobOrientation};

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

#[non_exhaustive]
#[derive(PartialEq, Debug, Clone, Copy)]
pub enum AngleKnobPreset {
    AdobePhotoshop,
    AdobePremierePro,
    Gimp,
    GoogleChromeDevTools,
    Krita,
    LibreOffice,
    QtWidgets,
    // Software without knob widgets:
    // - Blender (no knobs but transform gizmo suggests Top/Clockwise/SpinAround)
    // - Inkscape
    // - Kdenlive
    // - MyPaint (no knobs but canvas rotation suggests Right/Clockwise/Signed)
}

impl AngleKnobPreset {
    fn properties(&self) -> (KnobOrientation, KnobDirection, KnobMode) {
        match *self {
            AngleKnobPreset::AdobePhotoshop => (
                KnobOrientation::Right,
                KnobDirection::Counterclockwise,
                KnobMode::Signed,
            ),
            AngleKnobPreset::AdobePremierePro => (
                KnobOrientation::Top,
                KnobDirection::Clockwise,
                KnobMode::SpinAround,
            ),
            AngleKnobPreset::Gimp => (
                KnobOrientation::Right,
                KnobDirection::Counterclockwise,
                KnobMode::Unsigned,
            ),
            AngleKnobPreset::GoogleChromeDevTools => (
                KnobOrientation::Top,
                KnobDirection::Clockwise,
                KnobMode::Unsigned,
            ),
            AngleKnobPreset::Krita => (
                KnobOrientation::Right,
                KnobDirection::Counterclockwise,
                KnobMode::Signed,
            ),
            AngleKnobPreset::LibreOffice => (
                KnobOrientation::Right,
                KnobDirection::Counterclockwise,
                KnobMode::Unsigned,
            ),
            AngleKnobPreset::QtWidgets => (
                KnobOrientation::Bottom,
                KnobDirection::Clockwise,
                KnobMode::Unsigned,
            ),
        }
    }
}

#[must_use = "You should put this widget in an ui with `ui.add(widget);`"]
pub struct AngleKnob<'a> {
    get_set_value: GetSetValue<'a>,
    diameter: f32,
    orientation: KnobOrientation,
    direction: KnobDirection,
    mode: KnobMode,
    min: Option<f32>,
    max: Option<f32>,
    snap_angle: Option<f32>,
    shift_snap_angle: Option<f32>,
}

impl<'a> AngleKnob<'a> {
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
            diameter: 32.0,
            orientation: KnobOrientation::Top,
            direction: KnobDirection::Clockwise,
            mode: KnobMode::Unsigned,
            min: None,
            max: None,
            snap_angle: None,
            shift_snap_angle: Some(TAU / 24.0),
        }
    }

    pub fn preset(mut self, preset: AngleKnobPreset) -> Self {
        (self.orientation, self.direction, self.mode) = preset.properties();
        self
    }

    pub fn diameter(mut self, diameter: impl Into<f32>) -> Self {
        self.diameter = diameter.into();
        self
    }

    pub fn direction(mut self, direction: KnobDirection) -> Self {
        self.direction = direction;
        self
    }

    pub fn orientation(mut self, orientation: KnobOrientation) -> Self {
        self.orientation = orientation;
        self
    }

    pub fn mode(mut self, mode: KnobMode) -> Self {
        self.mode = mode;
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

impl<'a> Widget for AngleKnob<'a> {
    fn ui(mut self, ui: &mut Ui) -> Response {
        let desired_size = Vec2::splat(self.diameter);
        let (rect, mut response) =
            ui.allocate_exact_size(desired_size, egui::Sense::click_and_drag());

        let rotation_matrix = self.orientation.rot2();

        if response.clicked() || response.dragged() {
            let prev_value = get(&mut self.get_set_value);
            let mut new_value = (rotation_matrix.inverse()
                * (response.interact_pointer_pos().unwrap() - rect.center()))
            .angle()
                * self.direction.to_float();

            if let Some(angle) = if ui.input().modifiers.shift_only() {
                self.shift_snap_angle
            } else {
                self.snap_angle
            } {
                assert!(angle > 0.0, "non-positive snap angles are not supported");
                new_value = (new_value / angle).round() * angle;
            }

            if self.mode == KnobMode::Unsigned {
                new_value = normalized_angle_unsigned_excl(new_value);
            }

            if self.mode == KnobMode::SpinAround {
                let prev_turns = (prev_value / TAU).round();
                new_value += prev_turns * TAU;

                if new_value - prev_value > (TAU / 2.0) {
                    new_value -= TAU;
                } else if new_value - prev_value < -(TAU / 2.0) {
                    new_value += TAU;
                }
            }

            if let Some(min) = self.min {
                new_value = new_value.max(min);
            }

            if let Some(max) = self.max {
                new_value = new_value.min(max);
            }

            set(&mut self.get_set_value, new_value);
            response.mark_changed();
        }

        if ui.is_rect_visible(rect) {
            let visuals = ui.style().interact(&response);
            let radius = self.diameter / 2.0;

            ui.painter()
                .circle(rect.center(), radius, visuals.bg_fill, visuals.fg_stroke);

            let paint_axis = |axis_direction| {
                let axis_vec2 = rotation_matrix * axis_direction * radius;

                ui.painter().add(Shape::dashed_line(
                    &[rect.center() + axis_vec2, rect.center() - axis_vec2],
                    ui.visuals().window_stroke(), // TODO: Semantically correct color
                    1.0,
                    1.0,
                ));
            };

            paint_axis(Vec2::DOWN);
            paint_axis(Vec2::RIGHT);

            let mut paint_stop = |stop_position: f32| {
                let stop_vec2 = rotation_matrix
                    * Vec2::angled(stop_position * self.direction.to_float())
                    * radius;

                let stop_alpha = 1.0
                    - ((stop_position - get(&mut self.get_set_value)).abs() / (TAU * 0.75))
                        .clamp(0.0, 1.0)
                        .powf(5.0);

                // TODO: Semantically correct color
                let stop_stroke = Stroke::new(
                    visuals.fg_stroke.width,
                    visuals.fg_stroke.color.linear_multiply(stop_alpha),
                );

                ui.painter()
                    .line_segment([rect.center(), rect.center() + stop_vec2], stop_stroke);
            };

            if let Some(min) = self.min {
                paint_stop(min);
            }

            if let Some(max) = self.max {
                paint_stop(max);
            }

            {
                let value_vec2 = rotation_matrix
                    * Vec2::angled(get(&mut self.get_set_value) * self.direction.to_float())
                    * radius;

                ui.painter().line_segment(
                    [rect.center(), rect.center() + value_vec2],
                    visuals.fg_stroke, // TODO: Semantically correct color
                );

                ui.painter().circle(
                    rect.center(),
                    self.diameter / 24.0,
                    visuals.text_color(), // TODO: Semantically correct color
                    visuals.fg_stroke,    // TODO: Semantically correct color
                );

                ui.painter().circle(
                    rect.center() + value_vec2,
                    self.diameter / 24.0,
                    visuals.text_color(), // TODO: Semantically correct color
                    visuals.fg_stroke,    // TODO: Semantically correct color
                );
            }
        }

        response
    }
}
