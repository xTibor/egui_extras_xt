use std::f32::consts::PI;

use eframe::egui::{self, global_dark_light_mode_switch};
use eframe::emath::{lerp, vec2, Pos2, Vec2};
use eframe::epaint::{Shape, Stroke};

use itertools::Itertools;

pub fn potmeter_a(ui: &mut egui::Ui, diameter: f32, value: &mut f32) -> egui::Response {
    let desired_size = egui::vec2(diameter + 16.0, diameter + 16.0);

    let (rect, mut response) = ui.allocate_exact_size(desired_size, egui::Sense::click_and_drag());

    if response.clicked() || response.dragged() {
        *value = (rect.center() - response.interact_pointer_pos().unwrap())
            .rot90()
            .angle();
        response.mark_changed();
    }

    let visuals = ui.style().interact(&response);
    let diameter2 = diameter + visuals.expansion;

    for i in 0..40 {
        let direction = egui::Vec2::angled(PI * 2.0 / 40.0 * (i as f32));

        let tick_length = if i % 5 == 0 { 8.0 } else { 4.0 };

        ui.painter().line_segment(
            [
                rect.center() + direction * ((diameter / 2.0) + 4.0),
                rect.center() + direction * ((diameter / 2.0) + 4.0 + tick_length),
            ],
            ui.style().visuals.window_stroke(),
        );
    }

    ui.painter().circle(
        rect.center(),
        diameter2 / 2.0,
        visuals.bg_fill,
        visuals.fg_stroke,
    );

    let direction = egui::Vec2::angled(*value).rot90();

    ui.painter().line_segment(
        [
            rect.center() + direction * diameter2 / 8.0,
            rect.center() + direction * diameter2 / 2.0,
        ],
        visuals.fg_stroke,
    );

    response
}

pub fn potmeter_b(
    ui: &mut egui::Ui,
    diameter: f32,
    value: &mut f32,
    zero_angle: f32,
    label: &str,
) -> egui::Response {
    let desired_size = egui::vec2(diameter, diameter);
    let (rect, mut response) = ui.allocate_exact_size(desired_size, egui::Sense::click_and_drag());

    if response.dragged() {
        *value = (*value + (response.drag_delta().x - response.drag_delta().y) / diameter)
            .clamp(0.0, 1.0);
        response.mark_changed();
    }

    let visuals = ui.style().interact(&response);

    let paint_arc = |start_angle: f32, end_angle: f32, arc_width, fill, stroke| {
        let n_points = 32;

        let generate_arc_points = |radius| {
            (0..=n_points).map(move |i| {
                let angle = lerp(start_angle..=end_angle, i as f32 / n_points as f32);
                let (sin, cos) = angle.to_radians().sin_cos();
                rect.center() + vec2(sin as f32, -cos as f32) * radius
            })
        };

        let arc_diameter = (diameter / 2.0) * (3.0 / 4.0);
        let outer_arc = generate_arc_points(arc_diameter + (arc_width / 2.0)).collect::<Vec<_>>();
        let inner_arc = generate_arc_points(arc_diameter - (arc_width / 2.0)).collect::<Vec<_>>();

        // https://github.com/emilk/egui/issues/513
        outer_arc
            .iter()
            .zip(inner_arc.iter())
            .tuple_windows()
            .for_each(|((o1, i1), (o2, i2))| {
                ui.painter().add(Shape::convex_polygon(
                    vec![*o1, *i1, *i2, *o2],
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
    };

    let angle = -135.0 + 270.0 * *value;

    paint_arc(
        -135.0,
        135.0,
        diameter / 16.0,
        ui.style().visuals.faint_bg_color,
        ui.style().visuals.window_stroke(),
    );

    paint_arc(
        zero_angle,
        angle,
        diameter / 8.0 + visuals.expansion * 2.0,
        ui.style().visuals.selection.bg_fill,
        ui.style().visuals.selection.stroke,
    );

    ui.painter().circle(
        rect.center(),
        diameter / 4.0 + visuals.expansion,
        visuals.bg_fill,
        visuals.fg_stroke,
    );

    let direction = egui::Vec2::angled(angle.to_radians()).rot90();

    ui.painter().line_segment(
        [rect.center(), rect.center() + direction * diameter / 4.0],
        visuals.fg_stroke,
    );

    ui.painter().text(
        rect.left_top() + egui::vec2(4.0 / 8.0, 7.0 / 8.0) * desired_size,
        egui::Align2::CENTER_CENTER,
        label,
        egui::FontId::proportional(diameter / 5.0),
        visuals.text_color(),
    );

    response
}


// Common orientations:
//  ___     ___     ___
// /-0+\   /  +\   /  -\
// |   |   |  0|   |  0|
// \___/   \__-/   \__+/
//                Current
pub fn potmeter_c(ui: &mut egui::Ui, diameter: f32, value: &mut f32) -> egui::Response {
    let desired_size = egui::vec2(diameter, diameter);

    let (rect, mut response) = ui.allocate_exact_size(desired_size, egui::Sense::click_and_drag());

    let visuals = ui.style().interact(&response);

    if response.clicked() || response.dragged() {
        *value = (response.interact_pointer_pos().unwrap() - rect.center()).angle();
        response.mark_changed();
    }

    let direction = Vec2::angled(*value) * (diameter / 2.0);

    ui.painter().circle(
        rect.center(),
        diameter / 2.0,
        visuals.bg_fill,
        visuals.fg_stroke,
    );

    ui.painter().add(Shape::dashed_line(
        &[rect.left_center(), rect.right_center()],
        ui.visuals().window_stroke(), // TODO: Semantically correct color
        1.0,
        1.0,
    ));

    ui.painter().add(Shape::dashed_line(
        &[rect.center_top(), rect.center_bottom()],
        ui.visuals().window_stroke(), // TODO: Semantically correct color
        1.0,
        1.0,
    ));

    ui.painter().line_segment(
        [rect.center(), rect.center() + direction],
        visuals.fg_stroke, // TODO: Semantically correct color
    );

    ui.painter().circle(
        rect.center(),
        diameter / 24.0,
        visuals.text_color(), // TODO: Semantically correct color
        visuals.fg_stroke,    // TODO: Semantically correct color
    );

    ui.painter().circle(
        rect.center() + direction,
        diameter / 24.0,
        visuals.text_color(), // TODO: Semantically correct color
        visuals.fg_stroke,    // TODO: Semantically correct color
    );

    response
}

struct MyApp {
    potmeter_a: f32,
    potmeter_b: f32,
    potmeter_c: f32,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            potmeter_a: 0.0,
            potmeter_b: 0.5,
            potmeter_c: 0.5,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                global_dark_light_mode_switch(ui);
                ui.heading("Potmeters");
            });
            ui.separator();

            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.heading("Variant C");
                ui.label("Adobe Photoshop and Krita style");
                ui.add_space(16.0);
                ui.add(egui::Slider::new(&mut self.potmeter_c, -PI..=PI));

                ui.horizontal(|ui| {
                    potmeter_c(ui, 64.0, &mut self.potmeter_c);
                    potmeter_c(ui, 32.0, &mut self.potmeter_c);
                });

                ui.separator();
                ui.heading("Variant A");
                ui.label("Display style: tick marks");
                ui.label("Mouse control: absolute");
                ui.label("Knob range: -180°..180°");
                ui.add_space(16.0);

                ui.add(egui::Slider::new(&mut self.potmeter_a, -PI..=PI));

                ui.horizontal(|ui| {
                    potmeter_a(ui, 64.0, &mut self.potmeter_a);
                    potmeter_a(ui, 32.0, &mut self.potmeter_a);
                });

                ui.separator();
                ui.heading("Variant B");
                ui.label("Display style: filled arc");
                ui.label("Mouse control: relative");
                ui.label("Knob range: -135°..135°");
                ui.add_space(16.0);

                ui.add(egui::Slider::new(&mut self.potmeter_b, 0.0..=1.0));

                ui.horizontal(|ui| {
                    potmeter_b(ui, 128.0, &mut self.potmeter_b, -135.0, "VOL");
                    potmeter_b(ui, 64.0, &mut self.potmeter_b, -135.0, "VOL");
                    potmeter_b(ui, 32.0, &mut self.potmeter_b, -135.0, "VOL");

                    potmeter_b(ui, 128.0, &mut self.potmeter_b, 0.0, "PAN");
                    potmeter_b(ui, 64.0, &mut self.potmeter_b, 0.0, "PAN");
                    potmeter_b(ui, 32.0, &mut self.potmeter_b, 0.0, "PAN");
                });

                ui.separator();
            });
        });
    }
}

fn main() {
    let options = eframe::NativeOptions::default();

    eframe::run_native(
        "Potmeters",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    );
}
