use eframe::egui::{self, global_dark_light_mode_switch};
use eframe::emath::vec2;
use egui_extras_xt::segmented_display::{DisplayStylePreset, SegmentedDisplayWidget};

use chrono::{DateTime, TimeZone, Timelike, Utc};

struct DeLoreanDemoApp {
    destination_time: DateTime<Utc>,
    present_time: DateTime<Utc>,
    last_time_departed: DateTime<Utc>,
}

impl Default for DeLoreanDemoApp {
    fn default() -> Self {
        Self {
            destination_time: Utc.ymd(1885, 1, 1).and_hms(12, 0, 0),
            present_time: Utc.ymd(1955, 11, 12).and_hms(9, 28, 0),
            last_time_departed: Utc.ymd(1985, 10, 27).and_hms(14, 42, 0),
        }
    }
}

impl eframe::App for DeLoreanDemoApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                global_dark_light_mode_switch(ui);
                ui.heading("DeLorean Time Machine");
            });

            ui.separator();

            let mut add_time_machine_segment = |datetime: DateTime<_>, label, style_preset| {
                let str_month = datetime.format("%b").to_string().to_uppercase();
                let str_day = datetime.format("%d").to_string();
                let str_year = datetime.format("%Y").to_string();
                let (ampm, _) = datetime.hour12();
                let str_hour = datetime.format("%I").to_string();
                let str_min = datetime.format("%M").to_string();

                ui.separator();

                ui.vertical_centered(|ui| {
                    ui.horizontal(|ui| {
                        ui.vertical(|ui| {
                            ui.label("MONTH");
                            ui.add(
                                SegmentedDisplayWidget::sixteen_segment(&str_month)
                                    .style_preset(style_preset)
                                    .show_dots(false)
                                    .show_colons(false)
                                    .show_apostrophes(false),
                            );
                        });

                        ui.vertical(|ui| {
                            ui.label("DAY");
                            ui.add(
                                SegmentedDisplayWidget::seven_segment(&str_day)
                                    .style_preset(style_preset)
                                    .show_dots(true)
                                    .show_colons(false)
                                    .show_apostrophes(false),
                            );
                        });

                        ui.vertical(|ui| {
                            ui.label("YEAR");
                            ui.add(
                                SegmentedDisplayWidget::seven_segment(&str_year)
                                    .style_preset(style_preset)
                                    .show_dots(true)
                                    .show_colons(false)
                                    .show_apostrophes(false),
                            );
                        });

                        ui.vertical(|ui| {
                            ui.label("HOUR");
                            ui.add(
                                SegmentedDisplayWidget::seven_segment(&str_hour)
                                    .style_preset(style_preset)
                                    .show_dots(true)
                                    .show_colons(false)
                                    .show_apostrophes(false),
                            );
                        });

                        ui.vertical(|ui| {
                            ui.label("MIN");
                            ui.add(
                                SegmentedDisplayWidget::seven_segment(&str_min)
                                    .style_preset(style_preset)
                                    .show_dots(true)
                                    .show_colons(false)
                                    .show_apostrophes(false),
                            );
                        });
                    });
                    ui.heading(label);
                });

                ui.separator();
            };

            add_time_machine_segment(
                self.destination_time,
                "DESTINATION TIME",
                DisplayStylePreset::DeLoreanRed,
            );

            add_time_machine_segment(
                self.present_time,
                "PRESENT TIME",
                DisplayStylePreset::DeLoreanGreen,
            );

            add_time_machine_segment(
                self.last_time_departed,
                "LAST TIME DEPARTED",
                DisplayStylePreset::DeLoreanAmber,
            );
        });
    }
}

fn main() {
    let options = eframe::NativeOptions {
        initial_window_size: Some(vec2(800.0, 600.0)),
        ..Default::default()
    };

    eframe::run_native(
        "DeLorean Time Machine",
        options,
        Box::new(|_cc| Box::new(DeLoreanDemoApp::default())),
    );
}
