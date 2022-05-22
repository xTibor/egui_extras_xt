use std::f32::consts::TAU;

use eframe::egui::{self, Ui};
use eframe::emath::{normalized_angle, pos2, vec2, Align2};
use eframe::epaint::{Color32, FontFamily, FontId, Shape, Stroke};

pub struct CompassLabels<'a>(pub [&'a str; 4]);

pub fn compass_knob(
    ui: &mut Ui,
    value: &mut f32,
    width: f32,
    height: f32,
    labels: CompassLabels,
    spread: f32,
) -> egui::Response {
    let desired_size = egui::vec2(width, height);
    let (rect, mut response) = ui.allocate_exact_size(desired_size, egui::Sense::click_and_drag());

    if response.dragged() {
        *value -= response.drag_delta().x / rect.width() * spread;
        response.mark_changed();
    }

    let map_value_to_screen =
        |v: f32| rect.center().x - (normalized_angle(*value) - v) * (rect.width() / spread);

    if ui.is_rect_visible(rect) {
        let visuals = *ui.style().interact(&response);

        ui.painter().rect(
            rect,
            0.0,
            ui.style().visuals.faint_bg_color,
            ui.style().visuals.window_stroke(),
        );

        ui.painter().add(Shape::convex_polygon(
            vec![
                rect.center(),
                rect.center() - vec2(height / 6.0, height / 4.0),
                rect.center() - vec2(-height / 6.0, height / 4.0),
            ],
            visuals.bg_fill,
            visuals.fg_stroke,
        ));

        for i in -2..=3 {
            let x = map_value_to_screen(TAU / 4.0 * i as f32);

            ui.painter().line_segment(
                [
                    pos2(x, rect.top() + height * 0.5),
                    pos2(x, rect.top() + height * 0.75),
                ],
                ui.style().visuals.noninteractive().fg_stroke,
            );

            ui.painter().text(
                pos2(x, rect.bottom()),
                Align2::CENTER_BOTTOM,
                labels.0[((i + 4) % 4) as usize],
                FontId::new(height / 4.0, FontFamily::Proportional),
                ui.style().visuals.text_color(),
            );

            ui.painter().text(
                rect.center_top(),
                Align2::CENTER_TOP,
                format!("{}°", (((value.to_degrees() as isize) % 360) + 360) % 360),
                FontId::new(height / 4.0, FontFamily::Proportional),
                visuals.text_color(),
            );
        }
    }

    response
}
