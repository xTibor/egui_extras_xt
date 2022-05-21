use std::f32::consts::{PI, TAU};
use std::ops::RangeInclusive;

use eframe::egui::{self, Response, Ui, Widget};
use eframe::emath::{almost_equal, lerp, remap_clamp, Pos2, Rot2, Vec2};
use eframe::epaint::{Color32, Shape, Stroke};
use itertools::Itertools;

use crate::common::{KnobDirection, KnobOrientation};

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

type AudioKnobShapeFn<'a> = Box<dyn 'a + Fn(f32) -> f32>;

pub enum AudioKnobShape<'a> {
    Circle,
    Squircle(f32),
    Custom(AudioKnobShapeFn<'a>),
}

impl AudioKnobShape<'_> {
    pub fn eval(&self, theta: f32) -> f32 {
        match self {
            AudioKnobShape::Circle => 1.0,
            AudioKnobShape::Squircle(factor) => {
                1.0 / (theta.cos().abs().powf(*factor) + theta.sin().abs().powf(*factor))
                    .powf(1.0 / *factor)
            }
            AudioKnobShape::Custom(callback) => callback(theta),
        }
    }
}

// ----------------------------------------------------------------------------

fn paint_arc(
    ui: &mut egui::Ui,
    center: Pos2,
    inner_radius: f32,
    outer_radius: f32,
    start_angle: f32,
    end_angle: f32,
    fill: Color32,
    stroke: Stroke,
    rotation: Rot2,
    shape: &AudioKnobShape,
) {
    // NOTE: convex_polygon() is broken, spews rendering artifacts all over
    //   the window when it tries to render degenerate polygons:
    //     ∃(P1,P2) ∈ Poly (dist(P1,P2) ≈ 0)

    // HACK: convex_polygon() workaround
    if almost_equal(start_angle, end_angle, 0.001) {
        let shape_radius = shape.eval((rotation * Vec2::RIGHT).angle() - start_angle);

        ui.painter().add(Shape::line_segment(
            [
                center + Vec2::angled(start_angle) * inner_radius * shape_radius,
                center + Vec2::angled(start_angle) * outer_radius * shape_radius,
            ],
            stroke,
        ));
        return;
    }

    let n_points = 32;

    let generate_arc_points = |radius| {
        (0..=n_points).map(move |i| {
            let angle = lerp(start_angle..=end_angle, i as f32 / n_points as f32);
            let shape_radius = shape.eval((rotation * Vec2::RIGHT).angle() - angle);

            center + Vec2::angled(angle) * radius * shape_radius
        })
    };

    // HACK: convex_polygon() workaround
    let inner_radius = inner_radius.max(0.1);

    let outer_arc = generate_arc_points(outer_radius).collect::<Vec<_>>();
    let inner_arc = generate_arc_points(inner_radius).collect::<Vec<_>>();

    // https://github.com/emilk/egui/issues/513
    outer_arc
        .iter()
        .zip(inner_arc.iter())
        .tuple_windows()
        .for_each(|((outer_1, inner_1), (outer_2, inner_2))| {
            ui.painter().add(Shape::convex_polygon(
                vec![*outer_1, *inner_1, *inner_2, *outer_2],
                fill,
                Stroke::new(1.0, fill),
            ));
        });

    let outline_points: Vec<Pos2> = outer_arc
        .iter()
        .chain(inner_arc.iter().rev())
        .cloned()
        .collect();

    ui.painter().add(Shape::closed_line(outline_points, stroke));
}

// ----------------------------------------------------------------------------

#[must_use = "You should put this widget in an ui with `ui.add(widget);`"]
pub struct AudioKnob<'a> {
    get_set_value: GetSetValue<'a>,
    diameter: f32,
    direction: KnobDirection,
    orientation: KnobOrientation,
    range: RangeInclusive<f32>,
    spread: f32,
    thickness: f32,
    shape: AudioKnobShape<'a>,
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
            diameter: 32.0,
            orientation: KnobOrientation::Top,
            direction: KnobDirection::Clockwise,
            range,
            spread: 1.0,
            thickness: 0.66,
            shape: AudioKnobShape::Squircle(4.0),
        }
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

    pub fn spread(mut self, spread: impl Into<f32>) -> Self {
        self.spread = spread.into();
        self
    }

    pub fn thickness(mut self, thickness: impl Into<f32>) -> Self {
        self.thickness = thickness.into();
        self
    }

    pub fn shape(mut self, shape: AudioKnobShape<'a>) -> Self {
        self.shape = shape;
        self
    }
}

impl<'a> Widget for AudioKnob<'a> {
    fn ui(mut self, ui: &mut Ui) -> Response {
        let desired_size = Vec2::splat(self.diameter);
        let (rect, mut response) =
            ui.allocate_exact_size(desired_size, egui::Sense::click_and_drag());

        if response.dragged() {
            let drag_delta = self.orientation.rot2().inverse() * response.drag_delta();
            let delta = drag_delta.x + drag_delta.y * self.direction.to_float();

            let mut value = get(&mut self.get_set_value);

            value = (value + delta * (self.range.end() - self.range.start()) / self.diameter)
                .clamp(*self.range.start(), *self.range.end());

            set(&mut self.get_set_value, value);

            response.mark_changed();
        }

        if ui.is_rect_visible(rect) {
            let visuals = *ui.style().interact(&response);

            let center_angle = (self.orientation.rot2() * Vec2::RIGHT).angle();
            let spread_angle = (TAU / 2.0) * self.spread.clamp(0.0, 1.0);

            let (min_angle, max_angle) = (
                center_angle - spread_angle * self.direction.to_float(),
                center_angle + spread_angle * self.direction.to_float(),
            );

            let outer_radius = self.diameter / 2.0;
            let inner_radius = outer_radius * (1.0 - self.thickness.clamp(0.0, 1.0));

            paint_arc(
                ui,
                rect.center(),
                inner_radius,
                outer_radius,
                min_angle,
                max_angle,
                ui.style().visuals.faint_bg_color,
                ui.style().visuals.window_stroke(),
                self.orientation.rot2(),
                &self.shape,
            );

            paint_arc(
                ui,
                rect.center(),
                (inner_radius - visuals.expansion).max(0.0),
                outer_radius + visuals.expansion,
                remap_clamp(0.0, self.range.clone(), min_angle..=max_angle),
                remap_clamp(
                    get(&mut self.get_set_value),
                    self.range,
                    min_angle..=max_angle,
                ),
                visuals.bg_fill,
                visuals.fg_stroke,
                self.orientation.rot2(),
                &self.shape,
            );
        }

        response
    }
}
