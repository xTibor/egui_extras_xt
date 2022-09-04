use eframe::egui::{DragValue, Grid, TextEdit, Ui};
use eframe::epaint::Color32;
use egui_extras_xt::common::{Orientation, WidgetShape};
use egui_extras_xt::compasses::{CompassMarkerShape, DefaultCompassMarkerColor};
use egui_extras_xt::displays::segmented_display::DisplayMetricsPreset;
use egui_extras_xt::displays::{DisplayMetrics, DisplayStyle, DisplayStylePreset};
use egui_extras_xt::ui::widgets_from::{WidgetsFromIterator, WidgetsFromSlice};
use strum::IntoEnumIterator;

pub fn display_style_ui(
    ui: &mut Ui,
    style: &mut DisplayStyle,
    style_preset: &mut DisplayStylePreset,
) {
    Grid::new("style_properties")
        .num_columns(2)
        .spacing([20.0, 10.0])
        .striped(true)
        .show(ui, |ui| {
            ui.label("Style preset");
            ui.horizontal(|ui| {
                ui.push_id("style_preset_combo", |ui| {
                    ui.combobox_from_iter("", style_preset, DisplayStylePreset::iter())
                });
                // `.changed()` responses of combobox are broken.
                if ui.button("\u{2714} Apply").clicked() {
                    *style = style_preset.style();
                }
            });
            ui.end_row();

            ui.label("Background color");
            ui.color_edit_button_srgba(&mut style.background_color);
            ui.end_row();

            ui.label("Inactive foreground color");
            ui.color_edit_button_srgba(&mut style.inactive_foreground_color);
            ui.end_row();

            ui.label("Inactive foreground stroke");
            ui.horizontal(|ui| {
                ui.color_edit_button_srgba(&mut style.inactive_foreground_stroke.color);
                ui.add(DragValue::new(&mut style.inactive_foreground_stroke.width));
            });
            ui.end_row();

            ui.label("Active foreground color");
            ui.color_edit_button_srgba(&mut style.active_foreground_color);
            ui.end_row();

            ui.label("Active foreground stroke");
            ui.horizontal(|ui| {
                ui.color_edit_button_srgba(&mut style.active_foreground_stroke.color);
                ui.add(DragValue::new(&mut style.active_foreground_stroke.width));
            });
            ui.end_row();
        });
}

pub fn display_metrics_ui(
    ui: &mut Ui,
    metrics: &mut DisplayMetrics,
    metrics_preset: &mut DisplayMetricsPreset,
) {
    Grid::new("metrics_properties")
        .num_columns(2)
        .spacing([20.0, 10.0])
        .striped(true)
        .show(ui, |ui| {
            ui.label("Metrics preset");
            ui.horizontal(|ui| {
                ui.push_id("metrics_preset_combo", |ui| {
                    ui.combobox_from_iter("", metrics_preset, DisplayMetricsPreset::iter())
                });
                // `.changed()` responses of combobox are broken.
                if ui.button("\u{2714} Apply").clicked() {
                    *metrics = metrics_preset.metrics();
                }
            });
            ui.end_row();

            ui.label("Segment spacing");
            ui.add(DragValue::new(&mut metrics.segment_spacing));
            ui.end_row();

            ui.label("Segment thickness");
            ui.add(DragValue::new(&mut metrics.segment_thickness));
            ui.end_row();

            ui.label("Digit median");
            ui.add(DragValue::new(&mut metrics.digit_median));
            ui.end_row();

            ui.label("Digit ratio");
            ui.add(DragValue::new(&mut metrics.digit_ratio));
            ui.end_row();

            ui.label("Digit shearing");
            ui.add(DragValue::new(&mut metrics.digit_shearing));
            ui.end_row();

            ui.label("Digit spacing");
            ui.add(DragValue::new(&mut metrics.digit_spacing));
            ui.end_row();

            ui.label("Horizontal margin");
            ui.add(DragValue::new(&mut metrics.margin_horizontal));
            ui.end_row();

            ui.label("Vertical margin");
            ui.add(DragValue::new(&mut metrics.margin_vertical));
            ui.end_row();

            ui.label("Colon separation");
            ui.add(DragValue::new(&mut metrics.colon_separation));
            ui.end_row();
        });
}

pub fn widget_shape_ui(ui: &mut Ui, mut value: &mut WidgetShape) {
    ui.horizontal_centered(|ui| {
        ui.group(|ui| {
            ui.horizontal(|ui| {
                let selected = matches!(value, WidgetShape::Circle);
                let default_value = || WidgetShape::Circle;

                if ui.selectable_label(selected, "Circle").clicked() {
                    *value = default_value();
                }
            });
        });

        ui.group(|ui| {
            ui.horizontal(|ui| {
                let selected = matches!(value, WidgetShape::Square);
                let default_value = || WidgetShape::Square;

                if ui.selectable_label(selected, "Square").clicked() {
                    *value = default_value();
                }
            });
        });

        ui.group(|ui| {
            ui.horizontal(|ui| {
                let selected = matches!(value, WidgetShape::Squircle(..));
                let default_value = || WidgetShape::Squircle(4.0);

                if ui.selectable_label(selected, "Squircle").clicked() {
                    *value = default_value();
                }

                ui.add_enabled_ui(selected, |ui| {
                    let mut tmp = default_value();

                    if let WidgetShape::Squircle(ref mut factor) =
                        if selected { &mut value } else { &mut tmp }
                    {
                        ui.add(DragValue::new(factor));
                    }
                });
            });
        });

        ui.group(|ui| {
            ui.horizontal(|ui| {
                let selected = matches!(value, WidgetShape::Polygon(..));
                let default_value = || WidgetShape::Polygon(6);

                if ui.selectable_label(selected, "Polygon").clicked() {
                    *value = default_value();
                }

                ui.add_enabled_ui(selected, |ui| {
                    let mut tmp = default_value();

                    if let WidgetShape::Polygon(ref mut n) =
                        if selected { &mut value } else { &mut tmp }
                    {
                        ui.add(DragValue::new(n));
                    }
                });
            });
        });

        ui.group(|ui| {
            ui.horizontal(|ui| {
                let selected = matches!(value, WidgetShape::SuperPolygon(..));
                let default_value = || WidgetShape::SuperPolygon(6, 1.5);

                if ui.selectable_label(selected, "SuperPolygon").clicked() {
                    *value = default_value();
                }

                ui.add_enabled_ui(selected, |ui| {
                    let mut tmp = default_value();

                    if let WidgetShape::SuperPolygon(ref mut n, ref mut factor) =
                        if selected { &mut value } else { &mut tmp }
                    {
                        ui.add(DragValue::new(n));
                        ui.add(DragValue::new(factor));
                    }
                });
            });
        });
    });
}

pub fn widget_orientation_ui(ui: &mut Ui, mut value: &mut Orientation) {
    ui.horizontal_centered(|ui| {
        ui.selectable_value_from_slice(
            value,
            &[
                Orientation::Top,
                Orientation::Bottom,
                Orientation::Left,
                Orientation::Right,
            ],
        );

        ui.group(|ui| {
            ui.horizontal(|ui| {
                let selected = matches!(value, Orientation::Custom(..));
                let default_value = || Orientation::Custom(0.0);

                if ui.selectable_label(selected, "Custom").clicked() {
                    *value = default_value();
                }

                ui.add_enabled_ui(selected, |ui| {
                    let mut tmp = default_value();

                    if let Orientation::Custom(ref mut angle) =
                        if selected { &mut value } else { &mut tmp }
                    {
                        ui.drag_angle(angle);
                    }
                });
            });
        });
    });
}

pub fn default_compass_marker_color_ui(ui: &mut Ui, mut value: &mut DefaultCompassMarkerColor) {
    ui.horizontal_centered(|ui| {
        ui.group(|ui| {
            ui.horizontal(|ui| {
                let selected = matches!(value, DefaultCompassMarkerColor::System);
                let default_value = || DefaultCompassMarkerColor::System;

                if ui.selectable_label(selected, "System").clicked() {
                    *value = default_value();
                }
            });
        });

        ui.group(|ui| {
            ui.horizontal(|ui| {
                let selected = matches!(value, DefaultCompassMarkerColor::Fixed(..));
                let default_value = || DefaultCompassMarkerColor::Fixed(Color32::default());

                if ui.selectable_label(selected, "Fixed").clicked() {
                    *value = default_value();
                }

                ui.add_enabled_ui(selected, |ui| {
                    let mut tmp = default_value();

                    if let DefaultCompassMarkerColor::Fixed(ref mut color) =
                        if selected { &mut value } else { &mut tmp }
                    {
                        ui.color_edit_button_srgba(color);
                    }
                });
            });
        });

        ui.group(|ui| {
            ui.horizontal(|ui| {
                let selected = matches!(value, DefaultCompassMarkerColor::HsvByAngle { .. });
                let default_value = || DefaultCompassMarkerColor::HsvByAngle {
                    saturation: 1.0,
                    value: 1.0,
                };

                if ui.selectable_label(selected, "HSV by angle").clicked() {
                    *value = default_value();
                }

                ui.add_enabled_ui(selected, |ui| {
                    let mut tmp = default_value();

                    if let DefaultCompassMarkerColor::HsvByAngle { saturation, value } =
                        if selected { &mut value } else { &mut tmp }
                    {
                        ui.add(DragValue::new(saturation));
                        ui.add(DragValue::new(value));
                    }
                });
            });
        });

        ui.group(|ui| {
            ui.horizontal(|ui| {
                let selected = matches!(value, DefaultCompassMarkerColor::HsvByLabel { .. });
                let default_value = || DefaultCompassMarkerColor::HsvByLabel {
                    saturation: 1.0,
                    value: 1.0,
                };

                if ui.selectable_label(selected, "HSV by label").clicked() {
                    *value = default_value();
                }

                ui.add_enabled_ui(selected, |ui| {
                    let mut tmp = default_value();

                    if let DefaultCompassMarkerColor::HsvByLabel { saturation, value } =
                        if selected { &mut value } else { &mut tmp }
                    {
                        ui.add(DragValue::new(saturation));
                        ui.add(DragValue::new(value));
                    }
                });
            });
        });
    });
}

pub fn default_compass_marker_shape_ui(ui: &mut Ui, mut value: &mut CompassMarkerShape) {
    ui.horizontal_centered(|ui| {
        ui.combobox_from_slice(
            "",
            value,
            &[
                CompassMarkerShape::Square,
                CompassMarkerShape::Circle,
                CompassMarkerShape::RightArrow,
                CompassMarkerShape::UpArrow,
                CompassMarkerShape::LeftArrow,
                CompassMarkerShape::DownArrow,
                CompassMarkerShape::Diamond,
            ],
        );

        ui.group(|ui| {
            ui.horizontal(|ui| {
                let selected = matches!(value, CompassMarkerShape::Star(..));
                let default_value = || CompassMarkerShape::Star(5, 0.5);

                if ui.selectable_label(selected, "Star").clicked() {
                    *value = default_value();
                }

                ui.add_enabled_ui(selected, |ui| {
                    let mut tmp = default_value();

                    if let CompassMarkerShape::Star(ref mut rays, ref mut ratio) =
                        if selected { &mut value } else { &mut tmp }
                    {
                        ui.add(DragValue::new(rays));
                        ui.add(DragValue::new(ratio));
                    }
                });
            });
        });

        ui.group(|ui| {
            ui.horizontal(|ui| {
                let selected = matches!(value, CompassMarkerShape::Emoji(..));
                let default_value = || CompassMarkerShape::Emoji('?');

                if ui.selectable_label(selected, "Emoji").clicked() {
                    *value = default_value();
                }

                ui.add_enabled_ui(selected, |ui| {
                    let mut tmp = default_value();

                    if let CompassMarkerShape::Emoji(ref mut emoji) =
                        if selected { &mut value } else { &mut tmp }
                    {
                        let mut tmp = emoji.to_string();
                        ui.add(TextEdit::singleline(&mut tmp).desired_width(20.0));
                        *emoji = tmp.chars().next().unwrap_or(' ');
                    }
                });
            });
        });
    });
}

pub fn compass_axis_labels_ui(ui: &mut Ui, axis_labels: &mut Vec<String>) {
    ui.horizontal_centered(|ui| {
        for axis_label in axis_labels {
            ui.add(TextEdit::singleline(axis_label).desired_width(50.0));
        }
    });
}
