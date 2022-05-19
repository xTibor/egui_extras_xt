use std::f32::consts::{PI, TAU};

use eframe::egui::{self, global_dark_light_mode_switch};

mod angle_knob;
use angle_knob::{angle_knob, AngleKnobDirection, AngleKnobMode, AngleKnobOrientation};

mod audio_knob;
use audio_knob::audio_knob;

struct MyApp {
    // AngleKnob
    angle_knob_value: f32,
    angle_knob_orientation: AngleKnobOrientation,
    angle_knob_direction: AngleKnobDirection,
    angle_knob_mode: AngleKnobMode,
    angle_knob_minimum: Option<f32>,
    angle_knob_maximum: Option<f32>,
    angle_knob_snap_angle: Option<f32>,
    angle_knob_shift_snap_angle: Option<f32>,

    // AudioKnob
    audio_knob_value: f32,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            // AngleKnob
            angle_knob_value: PI / 9.0,
            angle_knob_orientation: AngleKnobOrientation::Top,
            angle_knob_direction: AngleKnobDirection::Clockwise,
            angle_knob_mode: AngleKnobMode::Signed,
            angle_knob_minimum: None,
            angle_knob_maximum: None,
            angle_knob_snap_angle: None,
            angle_knob_shift_snap_angle: None,

            // AudioKnob
            audio_knob_value: 0.75,
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
                ui.heading("AudioKnob");
                ui.add_space(16.0);
                ui.add(egui::Slider::new(&mut self.audio_knob_value, -1.0..=1.0));
                ui.add_space(16.0);

                ui.horizontal(|ui| {
                    audio_knob(ui, 64.0, &mut self.audio_knob_value, 0.0..=1.0);
                    audio_knob(ui, 32.0, &mut self.audio_knob_value, 0.0..=1.0);

                    audio_knob(ui, 64.0, &mut self.audio_knob_value, -1.0..=1.0);
                    audio_knob(ui, 32.0, &mut self.audio_knob_value, -1.0..=1.0);
                });

                ui.separator();

                ui.heading("AngleKnob");
                ui.add_space(16.0);

                ui.drag_angle(&mut self.angle_knob_value);

                ui.horizontal(|ui| {
                    ui.selectable_value(
                        &mut self.angle_knob_mode,
                        AngleKnobMode::Signed,
                        "± Signed",
                    );

                    ui.selectable_value(
                        &mut self.angle_knob_mode,
                        AngleKnobMode::Unsigned,
                        "+ Unsigned",
                    );

                    ui.selectable_value(
                        &mut self.angle_knob_mode,
                        AngleKnobMode::SpinAround,
                        "🔃 SpinAround",
                    );
                });

                ui.horizontal(|ui| {
                    ui.selectable_value(
                        &mut self.angle_knob_orientation,
                        AngleKnobOrientation::Top,
                        "⬆ Top",
                    );
                    ui.selectable_value(
                        &mut self.angle_knob_orientation,
                        AngleKnobOrientation::Right,
                        "➡ Right",
                    );
                    ui.selectable_value(
                        &mut self.angle_knob_orientation,
                        AngleKnobOrientation::Bottom,
                        "⬇ Bottom",
                    );
                    ui.selectable_value(
                        &mut self.angle_knob_orientation,
                        AngleKnobOrientation::Left,
                        "⬅ Left",
                    );

                    {
                        let mut is_custom_orientation = matches!(
                            self.angle_knob_orientation,
                            AngleKnobOrientation::Custom(..)
                        );

                        ui.selectable_value(&mut is_custom_orientation, true, "✏ Custom(..)");

                        if is_custom_orientation
                            && !matches!(
                                self.angle_knob_orientation,
                                AngleKnobOrientation::Custom(..)
                            )
                        {
                            self.angle_knob_orientation = AngleKnobOrientation::Custom(0.0);
                        }

                        if let AngleKnobOrientation::Custom(value) =
                            &mut self.angle_knob_orientation
                        {
                            ui.drag_angle(value);
                        }
                    }
                });

                ui.horizontal(|ui| {
                    ui.selectable_value(
                        &mut self.angle_knob_direction,
                        AngleKnobDirection::Clockwise,
                        "⟳ Clockwise",
                    );
                    ui.selectable_value(
                        &mut self.angle_knob_direction,
                        AngleKnobDirection::Counterclockwise,
                        "⟲ Counterclockwise",
                    );
                });

                ui.horizontal(|ui| {
                    {
                        let mut minimum_enabled = self.angle_knob_minimum.is_some();
                        ui.toggle_value(&mut minimum_enabled, "Minimum");

                        self.angle_knob_minimum = match (minimum_enabled, self.angle_knob_minimum) {
                            (true, None) => Some(-TAU),
                            (false, Some(_)) => None,
                            _ => self.angle_knob_minimum,
                        };

                        if let Some(value) = &mut self.angle_knob_minimum {
                            ui.drag_angle(value);
                        }
                    }

                    {
                        let mut maximum_enabled = self.angle_knob_maximum.is_some();
                        ui.toggle_value(&mut maximum_enabled, "Maximum");

                        self.angle_knob_maximum = match (maximum_enabled, self.angle_knob_maximum) {
                            (true, None) => Some(TAU),
                            (false, Some(_)) => None,
                            _ => self.angle_knob_maximum,
                        };

                        if let Some(value) = &mut self.angle_knob_maximum {
                            ui.drag_angle(value);
                        }
                    }
                });

                ui.horizontal(|ui| {
                    {
                        let mut snap_enabled = self.angle_knob_snap_angle.is_some();
                        ui.toggle_value(&mut snap_enabled, "Snap");

                        self.angle_knob_snap_angle =
                            match (snap_enabled, self.angle_knob_snap_angle) {
                                (true, None) => Some(PI / 12.0),
                                (false, Some(_)) => None,
                                _ => self.angle_knob_snap_angle,
                            };

                        if let Some(value) = &mut self.angle_knob_snap_angle {
                            ui.drag_angle(value);
                        }
                    }

                    {
                        let mut shift_snap_enabled = self.angle_knob_shift_snap_angle.is_some();
                        ui.toggle_value(&mut shift_snap_enabled, "Shift snap");

                        self.angle_knob_shift_snap_angle =
                            match (shift_snap_enabled, self.angle_knob_shift_snap_angle) {
                                (true, None) => Some(PI / 12.0),
                                (false, Some(_)) => None,
                                _ => self.angle_knob_shift_snap_angle,
                            };

                        if let Some(value) = &mut self.angle_knob_shift_snap_angle {
                            ui.drag_angle(value);
                        }
                    }
                });

                ui.add_space(16.0);

                ui.horizontal(|ui| {
                    for angle_knob_size in [64.0, 32.0] {
                        angle_knob(
                            ui,
                            angle_knob_size,
                            self.angle_knob_orientation,
                            self.angle_knob_direction,
                            self.angle_knob_mode,
                            &mut self.angle_knob_value,
                            self.angle_knob_minimum,
                            self.angle_knob_maximum,
                            self.angle_knob_snap_angle,
                            self.angle_knob_shift_snap_angle,
                        );
                    }
                });

                ui.separator();
            });
        });
    }
}

fn main() {
    let options = eframe::NativeOptions::default();

    eframe::run_native(
        "AngleKnobs",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    );
}
