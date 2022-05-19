use std::f32::consts::{PI, TAU};

use eframe::egui;
use eframe::emath::{Rot2, Vec2};
use eframe::epaint::{Shape, Stroke};

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum AngleKnobOrientation {
    Right,
    Bottom,
    Left,
    Top,
    Custom(f32),
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum AngleKnobDirection {
    Clockwise,
    Counterclockwise,
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum AngleKnobMode {
    Signed,
    Unsigned,
    SpinAround,
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum AngleKnobPreset {
    AdobePhotoshop,
    GIMP,
    GoogleChromeDevTools,
    Krita,
    LibreOffice,
    VLC,
    // Multimedia software lacking knob widgets:
    // - Blender
    // - Inkscape
    // - Kdenlive
    // - MyPaint (canvas behaves Right/Clockwise/Signed)
}

impl AngleKnobPreset {
    fn properties(
        &self,
    ) -> (
        AngleKnobOrientation,
        AngleKnobDirection,
        AngleKnobMode,
        Option<f32>,
        Option<f32>,
        Option<f32>,
        Option<f32>,
    ) {
        match *self {
            // Knobs widgets are a clusterfuck in Krita, however a significant
            // number of them follow what Photoshop does.
            AngleKnobPreset::AdobePhotoshop | AngleKnobPreset::Krita => (
                AngleKnobOrientation::Right,
                AngleKnobDirection::Counterclockwise,
                AngleKnobMode::Signed,
                None,
                None,
                None,
                Some(PI / 12.0),
            ),
            AngleKnobPreset::GIMP | AngleKnobPreset::LibreOffice => (
                AngleKnobOrientation::Right,
                AngleKnobDirection::Counterclockwise,
                AngleKnobMode::Unsigned,
                None,
                None,
                None,
                Some(PI / 12.0),
            ),
            AngleKnobPreset::GoogleChromeDevTools => (
                AngleKnobOrientation::Top,
                AngleKnobDirection::Clockwise,
                AngleKnobMode::Unsigned,
                None,
                None,
                None,
                Some(PI / 12.0),
            ),
            AngleKnobPreset::VLC => (
                AngleKnobOrientation::Bottom,
                AngleKnobDirection::Clockwise,
                AngleKnobMode::Unsigned,
                None,
                None,
                None,
                None,
            ),
        }
    }
}

pub fn angle_knob(
    ui: &mut egui::Ui,
    diameter: f32,
    orientation: AngleKnobOrientation,
    direction: AngleKnobDirection,
    mode: AngleKnobMode,
    value: &mut f32,
    min: Option<f32>,
    max: Option<f32>,
    snap_angle: Option<f32>,
    shift_snap_angle: Option<f32>,
) -> egui::Response {
    let desired_size = Vec2::splat(diameter);
    let (rect, mut response) = ui.allocate_exact_size(desired_size, egui::Sense::click_and_drag());

    let value_direction = match direction {
        AngleKnobDirection::Clockwise => 1.0,
        AngleKnobDirection::Counterclockwise => -1.0,
    };

    let widget_rotation = match orientation {
        AngleKnobOrientation::Right => Rot2::from_angle(PI * 0.0),
        AngleKnobOrientation::Bottom => Rot2::from_angle(PI * 0.5),
        AngleKnobOrientation::Left => Rot2::from_angle(PI * 1.0),
        AngleKnobOrientation::Top => Rot2::from_angle(PI * 1.5),
        AngleKnobOrientation::Custom(angle) => Rot2::from_angle(angle),
    };

    if response.clicked() || response.dragged() {
        let mut new_value = (widget_rotation.inverse()
            * (response.interact_pointer_pos().unwrap() - rect.center()))
        .angle()
            * value_direction;

        if mode == AngleKnobMode::Unsigned {
            new_value = (new_value + TAU) % TAU;
        }

        if mode == AngleKnobMode::SpinAround {
            let prev_turns = (*value / TAU).round();
            new_value += prev_turns * TAU;

            if new_value - *value > PI {
                new_value -= TAU;
            } else if new_value - *value < -PI {
                new_value += TAU;
            }
        }

        if let Some(angle) = if ui.input().modifiers.shift_only() {
            shift_snap_angle
        } else {
            snap_angle
        } {
            assert!(angle > 0.0, "non-positive snap angles are not supported");
            new_value = (new_value / angle).round() * angle;
        }

        if let Some(min) = min {
            new_value = new_value.max(min);
        }

        if let Some(max) = max {
            new_value = new_value.min(max);
        }

        *value = new_value;
        response.mark_changed();
    }

    if ui.is_rect_visible(rect) {
        let visuals = ui.style().interact(&response);
        let radius = diameter / 2.0;

        ui.painter().circle(
            rect.center(),
            diameter / 2.0,
            visuals.bg_fill,
            visuals.fg_stroke,
        );

        {
            let axis1_vec2 = widget_rotation * Vec2::DOWN * radius;

            ui.painter().add(Shape::dashed_line(
                &[rect.center() + axis1_vec2, rect.center() - axis1_vec2],
                ui.visuals().window_stroke(), // TODO: Semantically correct color
                1.0,
                1.0,
            ));
        }

        {
            let axis2_vec2 = widget_rotation * Vec2::RIGHT * radius;

            ui.painter().add(Shape::dashed_line(
                &[rect.center() + axis2_vec2, rect.center() - axis2_vec2],
                ui.visuals().window_stroke(), // TODO: Semantically correct color
                1.0,
                1.0,
            ));
        }

        if let Some(min) = min {
            let min_vec2 = widget_rotation * Vec2::angled(min * value_direction) * radius;
            let min_alpha = 1.0
                - ((min - *value).abs() / (PI * 1.5))
                    .clamp(0.0, 1.0)
                    .powf(5.0);

            // TODO: Semantically correct color
            let min_stroke = Stroke::new(
                visuals.fg_stroke.width,
                visuals.fg_stroke.color.linear_multiply(min_alpha),
            );

            ui.painter()
                .line_segment([rect.center(), rect.center() + min_vec2], min_stroke);
        }

        if let Some(max) = max {
            let max_vec2 = widget_rotation * Vec2::angled(max * value_direction) * radius;
            let max_alpha = 1.0
                - ((max - *value).abs() / (PI * 1.5))
                    .clamp(0.0, 1.0)
                    .powf(5.0);

            // TODO: Semantically correct color
            let max_stroke = Stroke::new(
                visuals.fg_stroke.width,
                visuals.fg_stroke.color.linear_multiply(max_alpha),
            );

            ui.painter()
                .line_segment([rect.center(), rect.center() + max_vec2], max_stroke);
        }

        {
            let value_vec2 = widget_rotation * Vec2::angled(*value * value_direction) * radius;

            ui.painter().line_segment(
                [rect.center(), rect.center() + value_vec2],
                visuals.fg_stroke, // TODO: Semantically correct color
            );

            ui.painter().circle(
                rect.center(),
                diameter / 24.0,
                visuals.text_color(), // TODO: Semantically correct color
                visuals.fg_stroke,    // TODO: Semantically correct color
            );

            ui.painter().circle(
                rect.center() + value_vec2,
                diameter / 24.0,
                visuals.text_color(), // TODO: Semantically correct color
                visuals.fg_stroke,    // TODO: Semantically correct color
            );
        }
    }

    response
}
