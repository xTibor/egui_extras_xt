use std::ops::RangeInclusive;

use eframe::egui;
use eframe::emath::{lerp, remap_clamp, vec2, Pos2, Vec2};
use eframe::epaint::{Color32, Shape, Stroke};
use itertools::Itertools;

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
    let n_points = 32;

    let generate_arc_points = |radius| {
        (0..=n_points).map(move |i| {
            let angle = lerp(start_angle..=end_angle, i as f32 / n_points as f32);
            let (sin, cos) = angle.to_radians().sin_cos();
            center + vec2(sin as f32, -cos as f32) * radius
        })
    };

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
                Stroke::none(),
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
    value: &mut f32,
    range: RangeInclusive<f32>,
) -> egui::Response {
    let desired_size = Vec2::splat(diameter);
    let (rect, mut response) = ui.allocate_exact_size(desired_size, egui::Sense::click_and_drag());

    if response.dragged() {
        let delta = response.drag_delta().x - response.drag_delta().y;
        *value = (*value + delta * (*range.end() - *range.start()) / diameter)
            .clamp(*range.start(), *range.end());
        response.mark_changed();
    }

    if ui.is_rect_visible(rect) {
        let (min_angle, max_angle) = (-135.0, 135.0);
        let visuals = *ui.style().interact(&response);

        paint_arc(
            ui,
            rect.center(),
            diameter / 6.0,
            diameter / 2.0,
            min_angle,
            max_angle,
            ui.style().visuals.faint_bg_color,
            ui.style().visuals.window_stroke(),
        );

        paint_arc(
            ui,
            rect.center(),
            diameter / 6.0 - visuals.expansion,
            diameter / 2.0 + visuals.expansion,
            remap_clamp(0.0, range.clone(), min_angle..=max_angle),
            remap_clamp(*value, range, min_angle..=max_angle),
            visuals.bg_fill,
            visuals.fg_stroke,
        );
    }

    response
}
