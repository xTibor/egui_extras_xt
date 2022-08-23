use eframe::egui::{self, Style, Visuals};
use eframe::emath::vec2;
use egui_extras_xt::displays::{DisplayStylePreset, LedDisplay, SegmentedDisplayWidget};

struct DeloreanDateTime<'a>(&'a str, usize, usize, bool, usize, usize);

struct TimeCircuitSegment<'a> {
    label: String,
    datetime: DeloreanDateTime<'a>,
    style_preset: DisplayStylePreset,
}

struct DeLoreanDemoApp<'a> {
    time_circuit_segments: Vec<TimeCircuitSegment<'a>>,
}

impl<'a> Default for DeLoreanDemoApp<'a> {
    fn default() -> Self {
        Self {
            time_circuit_segments: vec![
                TimeCircuitSegment {
                    label: "DESTINATION TIME".to_owned(),
                    datetime: DeloreanDateTime("JAN", 1, 1885, true, 12, 0),
                    style_preset: DisplayStylePreset::DeLoreanRed,
                },
                TimeCircuitSegment {
                    label: "PRESENT TIME".to_owned(),
                    datetime: DeloreanDateTime("NOV", 12, 1955, false, 9, 28),
                    style_preset: DisplayStylePreset::DeLoreanGreen,
                },
                TimeCircuitSegment {
                    label: "LAST TIME DEPARTED".to_owned(),
                    datetime: DeloreanDateTime("OCT", 27, 1985, true, 2, 42),
                    style_preset: DisplayStylePreset::DeLoreanAmber,
                },
            ],
        }
    }
}

impl<'a> eframe::App for DeLoreanDemoApp<'a> {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            for TimeCircuitSegment {
                label,
                datetime: DeloreanDateTime(month, day, year, ampm, hour, minute),
                style_preset,
            } in &self.time_circuit_segments
            {
                ui.group(|ui| {
                    egui::Grid::new(label).min_col_width(20.0).show(ui, |ui| {
                        ui.vertical_centered(|ui| ui.label("MONTH"));
                        ui.vertical_centered(|ui| ui.label("DAY"));
                        ui.vertical_centered(|ui| ui.label("YEAR"));
                        ui.vertical_centered(|ui| ui.label(""));
                        ui.vertical_centered(|ui| ui.label("HOUR"));
                        ui.vertical_centered(|ui| ui.label(""));
                        ui.vertical_centered(|ui| ui.label("MIN"));
                        ui.end_row();

                        ui.add(
                            SegmentedDisplayWidget::sixteen_segment(month)
                                .style_preset(*style_preset)
                                .show_dots(false)
                                .show_colons(false)
                                .show_apostrophes(false)
                                .digit_height(64.0),
                        );
                        ui.add(
                            SegmentedDisplayWidget::seven_segment(&format!("{:02}", day))
                                .style_preset(*style_preset)
                                .show_dots(true)
                                .show_colons(false)
                                .show_apostrophes(false)
                                .digit_height(64.0),
                        );
                        ui.add(
                            SegmentedDisplayWidget::seven_segment(&format!("{:04}", year))
                                .style_preset(*style_preset)
                                .show_dots(true)
                                .show_colons(false)
                                .show_apostrophes(false)
                                .digit_height(64.0),
                        );

                        ui.vertical_centered(|ui| {
                            ui.label("AM");
                            ui.add(
                                LedDisplay::from_bool(!ampm)
                                    .style_preset(*style_preset)
                                    .diameter(12.0),
                            );
                            ui.label("PM");
                            ui.add(
                                LedDisplay::from_bool(*ampm)
                                    .style_preset(*style_preset)
                                    .diameter(12.0),
                            );
                        });

                        ui.add(
                            SegmentedDisplayWidget::seven_segment(&format!("{:02}", hour))
                                .style_preset(*style_preset)
                                .show_dots(true)
                                .show_colons(false)
                                .show_apostrophes(false)
                                .digit_height(64.0),
                        );

                        ui.vertical_centered(|ui| {
                            ui.add_space(15.0);
                            ui.add(
                                LedDisplay::from_bool(true)
                                    .style_preset(*style_preset)
                                    .diameter(12.0),
                            );
                            ui.add_space(10.0);
                            ui.add(
                                LedDisplay::from_bool(true)
                                    .style_preset(*style_preset)
                                    .diameter(12.0),
                            );
                        });

                        ui.add(
                            SegmentedDisplayWidget::seven_segment(&format!("{:02}", minute))
                                .style_preset(*style_preset)
                                .show_dots(true)
                                .show_colons(false)
                                .show_apostrophes(false)
                                .digit_height(64.0),
                        );
                        ui.end_row();
                    });

                    ui.shrink_width_to_current();
                    ui.vertical_centered(|ui| {
                        ui.heading(label);
                    });
                });
            }
        });
    }
}

fn main() {
    let options = eframe::NativeOptions {
        initial_window_size: Some(vec2(1000.0, 500.0)),
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
