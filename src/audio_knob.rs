use std::f32::consts::TAU;
use std::ops::RangeInclusive;

use eframe::egui;
use eframe::emath::{almost_equal, lerp, remap_clamp, Pos2, Vec2};
use eframe::epaint::{Color32, Shape, Stroke};
use itertools::Itertools;

use crate::common::{KnobDirection, KnobOrientation};

fn paint_arc(
    ui: &mut egui::Ui,
    center: Pos2,
    inner_radius: f32,
    outer_radius: f32,
    start_angle: f32,
    end_angle: f32,
    fill: Color32,
    stroke: Stroke,
) {
    // NOTE: convex_polygon() is broken, spews rendering artifacts all over
    //   the window when it tries to render degenerate polygons:
    //     ∃(P1,P2) ∈ Poly (dist(P1,P2) ≈ 0)

    // HACK: convex_polygon() workaround
    if almost_equal(start_angle, end_angle, 0.001) {
        ui.painter().add(Shape::line_segment(
            [
                center + Vec2::angled(start_angle) * inner_radius,
                center + Vec2::angled(start_angle) * outer_radius,
            ],
            stroke,
        ));
        return;
    }

    let n_points = 32;

    let generate_arc_points = |radius| {
        (0..=n_points).map(move |i| {
            let angle = lerp(start_angle..=end_angle, i as f32 / n_points as f32);
            center + Vec2::angled(angle) * radius
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

pub fn audio_knob(
    ui: &mut egui::Ui,
    diameter: f32,
    orientation: KnobOrientation,
    direction: KnobDirection,
    value: &mut f32,
    range: RangeInclusive<f32>,
    spread: f32,
    thickness: f32,
) -> egui::Response {
    let desired_size = Vec2::splat(diameter);
    let (rect, mut response) = ui.allocate_exact_size(desired_size, egui::Sense::click_and_drag());

    if response.dragged() {
        let drag_delta = orientation.rot2().inverse() * response.drag_delta();
        let delta = drag_delta.x + drag_delta.y * direction.to_float();
        *value = (*value + delta * (*range.end() - *range.start()) / diameter)
            .clamp(*range.start(), *range.end());
        response.mark_changed();
    }

    if ui.is_rect_visible(rect) {
        let visuals = *ui.style().interact(&response);

        let center_angle = (orientation.rot2() * Vec2::RIGHT).angle();
        let spread_angle = (TAU / 2.0) * spread.clamp(0.0, 1.0);

        let (min_angle, max_angle) = (
            center_angle - spread_angle * direction.to_float(),
            center_angle + spread_angle * direction.to_float(),
        );

        let outer_radius = diameter / 2.0;
        let inner_radius = outer_radius * (1.0 - thickness.clamp(0.0, 1.0));

        paint_arc(
            ui,
            rect.center(),
            inner_radius,
            outer_radius,
            min_angle,
            max_angle,
            ui.style().visuals.faint_bg_color,
            ui.style().visuals.window_stroke(),
        );

        paint_arc(
            ui,
            rect.center(),
            (inner_radius - visuals.expansion).max(0.0),
            outer_radius + visuals.expansion,
            remap_clamp(0.0, range.clone(), min_angle..=max_angle),
            remap_clamp(*value, range, min_angle..=max_angle),
            visuals.bg_fill,
            visuals.fg_stroke,
        );
    }

    response
}
