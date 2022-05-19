use std::f32::consts::{PI, TAU};
use std::ops::RangeInclusive;

use eframe::egui::{self, global_dark_light_mode_switch};
use eframe::emath::{lerp, vec2, Pos2, Rot2, Vec2};
use eframe::epaint::{Color32, Shape, Stroke};

use itertools::Itertools;

pub fn paint_arc(
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
}

pub fn knob_variant_a(ui: &mut egui::Ui, diameter: f32, value: &mut f32) -> egui::Response {
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

pub fn knob_variant_b(
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

    let visuals = ui.style().interact(&response).clone();

    let angle = -135.0 + 270.0 * *value;
    let arc_radius = (diameter / 2.0) * (3.0 / 4.0);
    let arc_width_1 = diameter / 16.0;
    let arc_width_2 = diameter / 8.0 + visuals.expansion * 2.0;

    paint_arc(
        ui,
        rect.center(),
        arc_radius - (arc_width_1 / 2.0),
        arc_radius + (arc_width_1 / 2.0),
        -135.0,
        135.0,
        ui.style().visuals.faint_bg_color.clone(),
        ui.style().visuals.window_stroke().clone(),
    );

    paint_arc(
        ui,
        rect.center(),
        arc_radius - (arc_width_2 / 2.0),
        arc_radius + (arc_width_2 / 2.0),
        zero_angle,
        angle,
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

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum KnobOrientation {
    Right,
    Bottom,
    Left,
    Top,
    Custom(f32),
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum KnobDirection {
    Clockwise,
    Counterclockwise,
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum KnobMode {
    Signed,
    Unsigned,
    SpinAround,
}

pub fn knob_variant_c(
    ui: &mut egui::Ui,
    diameter: f32,
    orientation: KnobOrientation,
    direction: KnobDirection,
    mode: KnobMode,
    value: &mut f32,
    min: Option<f32>,
    max: Option<f32>,
) -> egui::Response {
    let desired_size = egui::vec2(diameter, diameter);
    let (rect, mut response) = ui.allocate_exact_size(desired_size, egui::Sense::click_and_drag());

    let value_direction = match direction {
        KnobDirection::Clockwise => 1.0,
        KnobDirection::Counterclockwise => -1.0,
    };

    let widget_rotation = match orientation {
        KnobOrientation::Right => Rot2::from_angle(PI * 0.0),
        KnobOrientation::Bottom => Rot2::from_angle(PI * 0.5),
        KnobOrientation::Left => Rot2::from_angle(PI * 1.0),
        KnobOrientation::Top => Rot2::from_angle(PI * 1.5),
        KnobOrientation::Custom(angle) => Rot2::from_angle(angle),
    };

    if response.clicked() || response.dragged() {
        let mut new_value = (widget_rotation.inverse()
            * (response.interact_pointer_pos().unwrap() - rect.center()))
        .angle()
            * value_direction;

        if mode == KnobMode::Unsigned {
            new_value = (new_value + TAU) % TAU;
        }

        if mode == KnobMode::SpinAround {
            let prev_turns = (*value / TAU).round();
            new_value = new_value + prev_turns as f32 * TAU;

            if new_value - *value > PI {
                new_value -= TAU;
            } else if new_value - *value < -PI {
                new_value += TAU;
            }
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

pub fn knob_variant_d(
    ui: &mut egui::Ui,
    diameter: f32,
    value: &mut f32,
    range: RangeInclusive<f32>,
) -> egui::Response {
    let desired_size = egui::vec2(diameter, diameter);
    let (rect, mut response) = ui.allocate_exact_size(desired_size, egui::Sense::click_and_drag());

    if response.dragged() {
        let delta = response.drag_delta().x - response.drag_delta().y;
        *value = (*value + delta * (*range.end() - *range.start()) / diameter)
            .clamp(*range.start(), *range.end());
        response.mark_changed();
    }

    if ui.is_rect_visible(rect) {
        let (min_angle, max_angle) = (-135.0, 135.0);

        let value_to_angle = |value: f32| {
            let value = value.clamp(*range.start(), *range.end());
            let t = (value - *range.start()) / (*range.end() - *range.start());
            min_angle + (max_angle - min_angle) * t
        };

        let visuals = ui.style().interact(&response).clone();

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
            value_to_angle(0.0),
            value_to_angle(*value),
            visuals.bg_fill,
            visuals.fg_stroke,
        );
    }

    response
}

struct MyApp {
    knob_a: f32,
    knob_b: f32,
    knob_c: f32,
    knob_c_orientation: KnobOrientation,
    knob_c_direction: KnobDirection,
    knob_c_mode: KnobMode,
    knob_c_minimum: Option<f32>,
    knob_c_maximum: Option<f32>,
    knob_d: f32,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            knob_a: 0.0,
            knob_b: 0.5,
            knob_c: PI / 9.0,
            knob_c_orientation: KnobOrientation::Top,
            knob_c_direction: KnobDirection::Clockwise,
            knob_c_mode: KnobMode::Signed,
            knob_c_minimum: None,
            knob_c_maximum: None,
            knob_d: 0.75,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                global_dark_light_mode_switch(ui);
                ui.heading("Knobs");
            });

            ui.separator();

            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.heading("Variant D");
                ui.add_space(16.0);
                ui.add(egui::Slider::new(&mut self.knob_d, -1.0..=1.0));
                ui.add_space(16.0);

                ui.horizontal(|ui| {
                    knob_variant_d(ui, 64.0, &mut self.knob_d, 0.0..=1.0);
                    knob_variant_d(ui, 32.0, &mut self.knob_d, 0.0..=1.0);

                    knob_variant_d(ui, 64.0, &mut self.knob_d, -1.0..=1.0);
                    knob_variant_d(ui, 32.0, &mut self.knob_d, -1.0..=1.0);
                });

                ui.separator();

                ui.heading("Variant C");
                ui.add_space(16.0);

                ui.drag_angle(&mut self.knob_c);

                ui.horizontal(|ui| {
                    ui.selectable_value(&mut self.knob_c_mode, KnobMode::Signed, "± Signed");

                    ui.selectable_value(&mut self.knob_c_mode, KnobMode::Unsigned, "+ Unsigned");

                    ui.selectable_value(
                        &mut self.knob_c_mode,
                        KnobMode::SpinAround,
                        "🔃 SpinAround",
                    );
                });

                ui.horizontal(|ui| {
                    ui.selectable_value(
                        &mut self.knob_c_orientation,
                        KnobOrientation::Top,
                        "⬆ Top",
                    );
                    ui.selectable_value(
                        &mut self.knob_c_orientation,
                        KnobOrientation::Right,
                        "➡ Right",
                    );
                    ui.selectable_value(
                        &mut self.knob_c_orientation,
                        KnobOrientation::Bottom,
                        "⬇ Bottom",
                    );
                    ui.selectable_value(
                        &mut self.knob_c_orientation,
                        KnobOrientation::Left,
                        "⬅ Left",
                    );

                    {
                        let mut is_custom_orientation =
                            matches!(self.knob_c_orientation, KnobOrientation::Custom(..));

                        ui.selectable_value(&mut is_custom_orientation, true, "✏ Custom(..)");

                        if is_custom_orientation
                            && !matches!(self.knob_c_orientation, KnobOrientation::Custom(..))
                        {
                            self.knob_c_orientation = KnobOrientation::Custom(0.0);
                        }

                        if let KnobOrientation::Custom(value) = &mut self.knob_c_orientation {
                            ui.drag_angle(value);
                        }
                    }
                });

                ui.horizontal(|ui| {
                    ui.selectable_value(
                        &mut self.knob_c_direction,
                        KnobDirection::Clockwise,
                        "⟳ Clockwise",
                    );
                    ui.selectable_value(
                        &mut self.knob_c_direction,
                        KnobDirection::Counterclockwise,
                        "⟲ Counterclockwise",
                    );
                });

                ui.horizontal(|ui| {
                    {
                        let mut minimum_enabled = self.knob_c_minimum.is_some();
                        ui.toggle_value(&mut minimum_enabled, "Minimum");

                        self.knob_c_minimum = match (minimum_enabled, self.knob_c_minimum) {
                            (true, None) => Some(-TAU),
                            (false, Some(_)) => None,
                            _ => self.knob_c_minimum,
                        };

                        if let Some(value) = &mut self.knob_c_minimum {
                            ui.drag_angle(value);
                        }
                    }

                    {
                        let mut maximum_enabled = self.knob_c_maximum.is_some();
                        ui.toggle_value(&mut maximum_enabled, "Maximum");

                        self.knob_c_maximum = match (maximum_enabled, self.knob_c_maximum) {
                            (true, None) => Some(TAU),
                            (false, Some(_)) => None,
                            _ => self.knob_c_maximum,
                        };

                        if let Some(value) = &mut self.knob_c_maximum {
                            ui.drag_angle(value);
                        }
                    }
                });

                ui.add_space(16.0);

                ui.horizontal(|ui| {
                    knob_variant_c(
                        ui,
                        64.0,
                        self.knob_c_orientation,
                        self.knob_c_direction,
                        self.knob_c_mode,
                        &mut self.knob_c,
                        self.knob_c_minimum,
                        self.knob_c_maximum,
                    );
                    knob_variant_c(
                        ui,
                        32.0,
                        self.knob_c_orientation,
                        self.knob_c_direction,
                        self.knob_c_mode,
                        &mut self.knob_c,
                        self.knob_c_minimum,
                        self.knob_c_maximum,
                    );
                });

                ui.separator();

                ui.collapsing("Old designs", |ui| {
                    ui.separator();

                    ui.heading("Variant A");
                    ui.label("Display style: tick marks");
                    ui.label("Mouse control: absolute");
                    ui.label("Knob range: -180°..180°");
                    ui.add_space(16.0);

                    ui.add(egui::Slider::new(&mut self.knob_a, -PI..=PI));

                    ui.horizontal(|ui| {
                        knob_variant_a(ui, 64.0, &mut self.knob_a);
                        knob_variant_a(ui, 32.0, &mut self.knob_a);
                    });

                    ui.separator();

                    ui.heading("Variant B");
                    ui.label("Display style: filled arc");
                    ui.label("Mouse control: relative");
                    ui.label("Knob range: -135°..135°");
                    ui.add_space(16.0);

                    ui.add(egui::Slider::new(&mut self.knob_b, 0.0..=1.0));

                    ui.horizontal(|ui| {
                        knob_variant_b(ui, 128.0, &mut self.knob_b, -135.0, "VOL");
                        knob_variant_b(ui, 64.0, &mut self.knob_b, -135.0, "VOL");
                        knob_variant_b(ui, 32.0, &mut self.knob_b, -135.0, "VOL");

                        knob_variant_b(ui, 128.0, &mut self.knob_b, 0.0, "PAN");
                        knob_variant_b(ui, 64.0, &mut self.knob_b, 0.0, "PAN");
                        knob_variant_b(ui, 32.0, &mut self.knob_b, 0.0, "PAN");
                    });

                    ui.separator();
                });
            });
        });
    }
}

fn main() {
    let options = eframe::NativeOptions::default();

    eframe::run_native("Knobs", options, Box::new(|_cc| Box::new(MyApp::default())));
}
