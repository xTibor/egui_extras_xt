use eframe::egui::{self, DragValue, Response};
use eframe::emath::vec2;

use strum::{Display, EnumIter, IntoEnumIterator};

use egui_extras_xt::ui::optional_value_widget::OptionalValueWidget;
use egui_extras_xt::ui::widgets_from_iter::{
    ComboBoxFromIter, RadioValueFromIter, SelectableValueFromIter,
};
use egui_extras_xt::ui::widgets_from_range::{
    ComboBoxFromRange, RadioValueFromRange, SelectableValueFromRange,
};
use egui_extras_xt::ui::widgets_from_slice::{
    ComboBoxFromSlice, RadioValueFromSlice, SelectableValueFromSlice,
};

#[derive(Clone, Copy, Display, EnumIter, PartialEq)]
enum SevenSecretWeapons {
    #[strum(to_string = "Missile")]
    Missile,

    #[strum(to_string = "Metal detector")]
    MetalDetector,

    #[strum(to_string = "Fishing pole")]
    FishingPole,

    #[strum(to_string = "Mr. Analysis")]
    MrAnalysis,

    #[strum(to_string = "Magnet")]
    Magnet,

    #[strum(to_string = "Bug sweeper")]
    BugSweeper,
}

struct UiExtensionsExample {
    optional_usize: Option<usize>,
    optional_string: Option<String>,
    secret_weapon: SevenSecretWeapons,
    coffee_count: usize,
}

impl Default for UiExtensionsExample {
    fn default() -> Self {
        Self {
            optional_usize: Some(1234),
            optional_string: Some("Test".to_owned()),
            secret_weapon: SevenSecretWeapons::MetalDetector,
            coffee_count: 1,
        }
    }
}

impl eframe::App for UiExtensionsExample {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        fn debug_print_response(widget_name: &'static str, response: Response) {
            if response.changed() {
                println!("{widget_name:} changed");
            }
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.push_id("optional_value", |ui| {
                ui.group(|ui| {
                    debug_print_response(
                        "optional_value_widget",
                        ui.optional_value_widget(&mut self.optional_usize, |ui, value| {
                            ui.add(DragValue::new(value))
                        }),
                    );

                    ui.separator();

                    debug_print_response(
                        "optional_value_widget",
                        ui.optional_value_widget(&mut self.optional_string, |ui, value| {
                            ui.text_edit_singleline(value)
                        }),
                    );
                });
            });
            ui.add_space(16.0);

            ui.push_id("from_iter", |ui| {
                ui.group(|ui| {
                    ui.horizontal_wrapped(|ui| {
                        debug_print_response(
                            "selectable_value_from_iter",
                            ui.selectable_value_from_iter(
                                &mut self.secret_weapon,
                                SevenSecretWeapons::iter(),
                            ),
                        );
                    });

                    ui.separator();

                    ui.horizontal_wrapped(|ui| {
                        debug_print_response(
                            "combobox_from_iter",
                            ui.combobox_from_iter(
                                "Secret weapon",
                                &mut self.secret_weapon,
                                SevenSecretWeapons::iter(),
                            ),
                        );
                    });

                    ui.separator();

                    ui.horizontal_wrapped(|ui| {
                        debug_print_response(
                            "radio_value_from_iter",
                            ui.radio_value_from_iter(
                                &mut self.secret_weapon,
                                SevenSecretWeapons::iter(),
                            ),
                        );
                    });
                });
            });
            ui.add_space(16.0);

            ui.push_id("from_slice", |ui| {
                ui.group(|ui| {
                    ui.horizontal_wrapped(|ui| {
                        debug_print_response(
                            "selectable_value_from_slice",
                            ui.selectable_value_from_slice(
                                &mut self.secret_weapon,
                                &[
                                    SevenSecretWeapons::Missile,
                                    SevenSecretWeapons::MetalDetector,
                                    SevenSecretWeapons::FishingPole,
                                    SevenSecretWeapons::MrAnalysis,
                                    SevenSecretWeapons::Magnet,
                                    SevenSecretWeapons::BugSweeper,
                                ],
                            ),
                        );
                    });

                    ui.separator();

                    ui.horizontal_wrapped(|ui| {
                        debug_print_response(
                            "combobox_from_slice",
                            ui.combobox_from_slice(
                                "Secret weapon",
                                &mut self.secret_weapon,
                                &[
                                    SevenSecretWeapons::Missile,
                                    SevenSecretWeapons::MetalDetector,
                                    SevenSecretWeapons::FishingPole,
                                    SevenSecretWeapons::MrAnalysis,
                                    SevenSecretWeapons::Magnet,
                                    SevenSecretWeapons::BugSweeper,
                                ],
                            ),
                        );
                    });

                    ui.separator();

                    ui.horizontal_wrapped(|ui| {
                        debug_print_response(
                            "radio_value_from_slice",
                            ui.radio_value_from_slice(
                                &mut self.secret_weapon,
                                &[
                                    SevenSecretWeapons::Missile,
                                    SevenSecretWeapons::MetalDetector,
                                    SevenSecretWeapons::FishingPole,
                                    SevenSecretWeapons::MrAnalysis,
                                    SevenSecretWeapons::Magnet,
                                    SevenSecretWeapons::BugSweeper,
                                ],
                            ),
                        );
                    });
                });
            });
            ui.add_space(16.0);

            ui.push_id("from_range", |ui| {
                ui.group(|ui| {
                    ui.horizontal_wrapped(|ui| {
                        debug_print_response(
                            "selectable_value_from_range",
                            ui.selectable_value_from_range(&mut self.coffee_count, 1..=17),
                        );
                    });

                    ui.separator();

                    ui.horizontal_wrapped(|ui| {
                        debug_print_response(
                            "combobox_from_range",
                            ui.combobox_from_range("Coffee count", &mut self.coffee_count, 1..=17),
                        );
                    });

                    ui.separator();

                    ui.horizontal_wrapped(|ui| {
                        debug_print_response(
                            "radio_value_from_range",
                            ui.radio_value_from_range(&mut self.coffee_count, 1..=17),
                        );
                    });
                });
            });
            ui.add_space(16.0);
        });
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        initial_window_size: Some(vec2(640.0, 480.0)),
        ..Default::default()
    };

    eframe::run_native(
        "Ui extensions",
        options,
        Box::new(|_| Box::<UiExtensionsExample>::default()),
    )
}
