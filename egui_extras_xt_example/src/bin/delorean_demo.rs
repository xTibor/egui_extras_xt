use eframe::egui::{self, Style, Ui, Visuals};
use eframe::emath::vec2;
use egui_extras_xt::display::{DisplayStylePreset, LedDisplay, SegmentedDisplayWidget};

use chrono::{DateTime, TimeZone, Timelike};
use chrono_tz::Tz;

struct DeLoreanDemoApp {
    destination_time: DateTime<Tz>,
    present_time: DateTime<Tz>,
    last_time_departed: DateTime<Tz>,
}

impl Default for DeLoreanDemoApp {
    fn default() -> Self {
        use chrono_tz::US::Pacific;

        Self {
            destination_time: Pacific.ymd(1885, 1, 1).and_hms(12, 0, 0),
            present_time: Pacific.ymd(1955, 11, 12).and_hms(9, 28, 0),
            last_time_departed: Pacific.ymd(1985, 10, 27).and_hms(14, 42, 0),
        }
    }
}

impl eframe::App for DeLoreanDemoApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let add_time_machine_segment =
                |ui: &mut Ui, datetime: DateTime<_>, label, style_preset| {
                    let str_month = datetime.format("%b").to_string().to_uppercase();
                    let str_day = datetime.format("%d").to_string();
                    let str_year = datetime.format("%Y").to_string();
                    let (ampm, _) = datetime.hour12();
                    let str_hour = datetime.format("%I").to_string();
                    let str_min = datetime.format("%M").to_string();

                    ui.group(|ui| {
                        egui::Grid::new(label).show(ui, |ui| {
                            ui.vertical_centered(|ui| ui.label("MONTH"));
                            ui.vertical_centered(|ui| ui.label("DAY"));
                            ui.vertical_centered(|ui| ui.label("YEAR"));
                            ui.vertical_centered(|ui| ui.label(""));
                            ui.vertical_centered(|ui| ui.label("HOUR"));
                            ui.vertical_centered(|ui| ui.label(""));
                            ui.vertical_centered(|ui| ui.label("MIN"));
                            ui.end_row();

                            ui.add(
                                SegmentedDisplayWidget::sixteen_segment(&str_month)
                                    .style_preset(style_preset)
                                    .show_dots(false)
                                    .show_colons(false)
                                    .show_apostrophes(false),
                            );
                            ui.add(
                                SegmentedDisplayWidget::seven_segment(&str_day)
                                    .style_preset(style_preset)
                                    .show_dots(true)
                                    .show_colons(false)
                                    .show_apostrophes(false),
                            );
                            ui.add(
                                SegmentedDisplayWidget::seven_segment(&str_year)
                                    .style_preset(style_preset)
                                    .show_dots(true)
                                    .show_colons(false)
                                    .show_apostrophes(false),
                            );

                            ui.vertical_centered(|ui| {
                                ui.label("AM");
                                ui.add(
                                    LedDisplay::from_bool(!ampm)
                                        .style_preset(style_preset)
                                        .diameter(12.0),
                                );
                                ui.label("PM");
                                ui.add(
                                    LedDisplay::from_bool(ampm)
                                        .style_preset(style_preset)
                                        .diameter(12.0),
                                );
                            });

                            ui.add(
                                SegmentedDisplayWidget::seven_segment(&str_hour)
                                    .style_preset(style_preset)
                                    .show_dots(true)
                                    .show_colons(false)
                                    .show_apostrophes(false),
                            );

                            ui.vertical_centered(|ui| {
                                ui.add(
                                    LedDisplay::from_bool(true)
                                        .style_preset(style_preset)
                                        .diameter(12.0),
                                );
                                ui.add(
                                    LedDisplay::from_bool(true)
                                        .style_preset(style_preset)
                                        .diameter(12.0),
                                );
                            });

                            ui.add(
                                SegmentedDisplayWidget::seven_segment(&str_min)
                                    .style_preset(style_preset)
                                    .show_dots(true)
                                    .show_colons(false)
                                    .show_apostrophes(false),
                            );
                            ui.end_row();
                        });

                        ui.shrink_width_to_current();
                        ui.vertical_centered(|ui| {
                            ui.heading(label);
                        });
                    });
                };

            add_time_machine_segment(
                ui,
                self.destination_time,
                "DESTINATION TIME",
                DisplayStylePreset::DeLoreanRed,
            );

            add_time_machine_segment(
                ui,
                self.present_time,
                "PRESENT TIME",
                DisplayStylePreset::DeLoreanGreen,
            );

            add_time_machine_segment(
                ui,
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
        Box::new(|cc| {
            cc.egui_ctx.set_style(Style {
                visuals: Visuals::dark(),
                ..Style::default()
            });

            Box::new(DeLoreanDemoApp::default())
        }),
    );
}
