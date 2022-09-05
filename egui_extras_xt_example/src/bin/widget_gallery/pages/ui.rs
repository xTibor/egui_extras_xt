use eframe::egui::{DragValue, Grid, TextEdit, Ui};
use eframe::epaint::Color32;
use egui_extras_xt::common::{Orientation, WidgetShape};
use egui_extras_xt::compasses::{CompassMarkerShape, DefaultCompassMarkerColor};
use egui_extras_xt::displays::segmented_display::DisplayMetricsPreset;
use egui_extras_xt::displays::{DisplayMetrics, DisplayStyle, DisplayStylePreset};
use egui_extras_xt::knobs::{ThumbstickDeadZone, ThumbstickSnap};
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

pub fn widget_shape_ui(ui: &mut Ui, value: &mut WidgetShape) {
    ui.group(|ui| {
        ui.horizontal_top(|ui| {
            ui.push_id("widget_shape_combo", |ui| {
                ui.combobox_from_slice(
                    "",
                    value,
                    &[
                        WidgetShape::Circle,
                        WidgetShape::Square,
                        WidgetShape::Squircle(4.0),
                        WidgetShape::Polygon(6),
                        WidgetShape::SuperPolygon(6, 1.5),
                        WidgetShape::Rotated(Box::new(WidgetShape::Square), 0.0f32.to_radians()),
                        WidgetShape::Scaled(Box::new(WidgetShape::Square), 1.0),
                        WidgetShape::Mix(
                            Box::new(WidgetShape::Circle),
                            Box::new(WidgetShape::Square),
                            0.5,
                        ),
                        WidgetShape::Min(
                            Box::new(WidgetShape::Circle),
                            Box::new(WidgetShape::Square),
                        ),
                        WidgetShape::Max(
                            Box::new(WidgetShape::Circle),
                            Box::new(WidgetShape::Square),
                        ),
                    ],
                );
            });

            match value {
                WidgetShape::Circle => {}
                WidgetShape::Square => {}
                WidgetShape::Squircle(factor) => {
                    ui.add(DragValue::new(factor));
                }
                WidgetShape::Polygon(n) => {
                    ui.add(DragValue::new(n));
                }
                WidgetShape::SuperPolygon(n, factor) => {
                    ui.vertical(|ui| {
                        ui.add(DragValue::new(n));
                        ui.add(DragValue::new(factor));
                    });
                }
                WidgetShape::Rotated(shape, rotation) => {
                    ui.vertical(|ui| {
                        widget_shape_ui(ui, shape);
                        ui.drag_angle(rotation);
                    });
                }
                WidgetShape::Scaled(shape, scale) => {
                    ui.vertical(|ui| {
                        widget_shape_ui(ui, shape);
                        ui.add(DragValue::new(scale));
                    });
                }
                WidgetShape::Mix(shape_a, shape_b, t) => {
                    ui.vertical(|ui| {
                        ui.push_id("shape_a", |ui| widget_shape_ui(ui, shape_a));
                        ui.add(DragValue::new(t));
                        ui.push_id("shape_b", |ui| widget_shape_ui(ui, shape_b));
                    });
                }
                WidgetShape::Min(shape_a, shape_b) | WidgetShape::Max(shape_a, shape_b) => {
                    ui.vertical(|ui| {
                        ui.push_id("shape_a", |ui| widget_shape_ui(ui, shape_a));
                        ui.push_id("shape_b", |ui| widget_shape_ui(ui, shape_b));
                    });
                }
                _ => unimplemented!(),
            }
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

pub fn default_compass_marker_color_ui(ui: &mut Ui, value: &mut DefaultCompassMarkerColor) {
    ui.horizontal_centered(|ui| {
        ui.push_id("compass_marker_color_combo", |ui| {
            ui.combobox_from_slice(
                "",
                value,
                &[
                    DefaultCompassMarkerColor::System,
                    DefaultCompassMarkerColor::Fixed(Color32::default()),
                    DefaultCompassMarkerColor::HsvByAngle {
                        saturation: 1.0,
                        value: 1.0,
                    },
                    DefaultCompassMarkerColor::HsvByLabel {
                        saturation: 1.0,
                        value: 1.0,
                    },
                ],
            );
        });

        match value {
            DefaultCompassMarkerColor::System => {}
            DefaultCompassMarkerColor::Fixed(color) => {
                ui.color_edit_button_srgba(color);
            }
            DefaultCompassMarkerColor::HsvByAngle { saturation, value } => {
                ui.add(DragValue::new(saturation));
                ui.add(DragValue::new(value));
            }
            DefaultCompassMarkerColor::HsvByLabel { saturation, value } => {
                ui.add(DragValue::new(saturation));
                ui.add(DragValue::new(value));
            }
            _ => unimplemented!(),
        }
    });
}

pub fn default_compass_marker_shape_ui(ui: &mut Ui, value: &mut CompassMarkerShape) {
    ui.horizontal_centered(|ui| {
        ui.push_id("compass_marker_shape_combo", |ui| {
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
                    CompassMarkerShape::Star(5, 0.5),
                    CompassMarkerShape::Emoji('?'),
                ],
            );
        });

        match value {
            CompassMarkerShape::Star(rays, ratio) => {
                ui.add(DragValue::new(rays));
                ui.add(DragValue::new(ratio));
            }
            CompassMarkerShape::Emoji(emoji) => {
                let mut tmp = emoji.to_string();
                ui.add(TextEdit::singleline(&mut tmp).desired_width(25.0));
                *emoji = tmp.chars().next().unwrap_or(' ');
            }
            CompassMarkerShape::Square
            | CompassMarkerShape::Circle
            | CompassMarkerShape::RightArrow
            | CompassMarkerShape::UpArrow
            | CompassMarkerShape::LeftArrow
            | CompassMarkerShape::DownArrow
            | CompassMarkerShape::Diamond => {}
            _ => unimplemented!(),
        }
    });
}

pub fn compass_axis_labels_ui(ui: &mut Ui, axis_labels: &mut Vec<String>) {
    ui.horizontal_centered(|ui| {
        for axis_label in axis_labels {
            ui.add(TextEdit::singleline(axis_label).desired_width(50.0));
        }
    });
}

pub fn thumbstick_snap_ui(ui: &mut Ui, value: &mut ThumbstickSnap) {
    ui.horizontal_centered(|ui| {
        ui.push_id("thumbstick_snap_combo", |ui| {
            ui.combobox_from_slice(
                "",
                value,
                &[
                    ThumbstickSnap::None,
                    ThumbstickSnap::Strict {
                        axes: 8,
                        rotation: 0.0f32.to_radians(),
                        threshold: 0.0,
                    },
                ],
            );
        });

        match value {
            ThumbstickSnap::None => {}
            ThumbstickSnap::Strict {
                axes,
                rotation,
                threshold,
            } => {
                ui.add(DragValue::new(axes));
                ui.drag_angle(rotation);
                ui.add(DragValue::new(threshold));
            }
            _ => unimplemented!(),
        }
    });
}

pub fn thumbstick_dead_zone_ui(ui: &mut Ui, value: &mut ThumbstickDeadZone) {
    ui.horizontal_centered(|ui| {
        ui.push_id("thumbstick_dead_zone_combo", |ui| {
            ui.combobox_from_slice(
                "",
                value,
                &[
                    ThumbstickDeadZone::None,
                    ThumbstickDeadZone::ScaledRadial { dead_zone: 0.25 },
                ],
            );
        });

        match value {
            ThumbstickDeadZone::None => {}
            ThumbstickDeadZone::ScaledRadial { dead_zone } => {
                ui.add(DragValue::new(dead_zone));
            }
            _ => unimplemented!(),
        }
    });
}
