use eframe::egui::{self, global_dark_light_mode_switch, DragValue, Style, Visuals};
use eframe::epaint::Color32;

use itertools::Itertools;

use egui_extras_xt::segmented_display::{
    DisplayKind, DisplayMetrics, DisplayStyle, DisplayStylePreset, SegmentedDisplayWidget,
};
use egui_extras_xt::{
    AngleKnob, AudioKnob, CompassMarker, CompassMarkerShape, LinearCompass, Orientation,
    PolarCompass, PolarCompassOverflow, ThumbstickKnob, WidgetShape, Winding, WrapMode,
};

struct EguiExtrasXtExampleApp {
    // Common properties
    common_interactive: bool,
    common_orientation: Orientation,
    common_winding: Winding,
    common_wrap: WrapMode,
    common_animated: bool,
    common_snap: Option<f32>,
    common_shift_snap: Option<f32>,
    common_minimum_angle: Option<f32>,
    common_maximum_angle: Option<f32>,

    // AngleKnob
    angle_knob_value: f32,

    // AudioKnob
    audio_knob_value: f32,
    audio_knob_spread: f32,
    audio_knob_thickness: f32,

    // LinearCompass
    linear_compass_value: f32,
    linear_compass_spread: f32,
    linear_compass_show_cursor: bool,

    // PolarCompass
    polar_compass_value: f32,
    polar_compass_overflow: PolarCompassOverflow,
    polar_compass_max_distance: f32,
    polar_compass_scale_log_base: f32,
    polar_compass_scale_log_mult: f32,
    polar_compass_diameter: f32,
    polar_compass_label_height: f32,
    polar_compass_marker_near_size: f32,
    polar_compass_marker_far_size: f32,
    polar_compass_show_axes: bool,
    polar_compass_show_rings: bool,
    polar_compass_show_cursor: bool,
    polar_compass_show_marker_labels: bool,
    polar_compass_show_marker_lines: bool,

    // SegmentedDisplayWidget
    segmented_display_display_kind: DisplayKind,
    segmented_display_display_string: String,
    segmented_display_digit_height: f32,
    segmented_display_style: DisplayStyle,
    segmented_display_metrics: DisplayMetrics,
    segmented_display_show_dots: bool,
    segmented_display_show_colons: bool,
    segmented_display_show_apostrophes: bool,

    // ThumbstickKnob
    thumbstick_knob_value: (f32, f32),
}

impl Default for EguiExtrasXtExampleApp {
    fn default() -> Self {
        Self {
            // Common properties
            common_interactive: true,
            common_orientation: Orientation::Top,
            common_winding: Winding::Clockwise,
            common_wrap: WrapMode::Signed,
            common_animated: false,
            common_snap: None,
            common_shift_snap: None,
            common_minimum_angle: None,
            common_maximum_angle: None,

            // AngleKnob
            angle_knob_value: 20.0f32.to_radians(),

            // AudioKnob
            audio_knob_value: 0.75,
            audio_knob_spread: 1.0,
            audio_knob_thickness: 0.66,

            // LinearCompass
            linear_compass_value: 0.0,
            linear_compass_spread: 180.0f32.to_radians(),
            linear_compass_show_cursor: true,

            // PolarCompass
            polar_compass_value: 0.0,
            polar_compass_overflow: PolarCompassOverflow::Saturate,
            polar_compass_max_distance: 20000.0,
            polar_compass_scale_log_base: 10.0,
            polar_compass_scale_log_mult: 2.0,
            polar_compass_diameter: 256.0,
            polar_compass_label_height: 24.0,
            polar_compass_marker_near_size: 16.0,
            polar_compass_marker_far_size: 8.0,
            polar_compass_show_axes: true,
            polar_compass_show_rings: true,
            polar_compass_show_cursor: true,
            polar_compass_show_marker_labels: true,
            polar_compass_show_marker_lines: true,

            // SegmentedDisplayWidget
            segmented_display_display_kind: DisplayKind::SixteenSegment,
            segmented_display_display_string: String::from("12.34:5' HELLO"),
            segmented_display_digit_height: 128.0,
            segmented_display_style: DisplayStylePreset::NintendoGameBoy.style(),
            segmented_display_metrics: DisplayMetrics::default(),
            segmented_display_show_dots: true,
            segmented_display_show_colons: true,
            segmented_display_show_apostrophes: true,

            // ThumbstickKnob
            thumbstick_knob_value: (0.0, 0.0),
        }
    }
}

impl eframe::App for EguiExtrasXtExampleApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                global_dark_light_mode_switch(ui);
                ui.heading("Knobs");

                if ui.button("Reset").clicked() {
                    *self = Self::default();
                }
            });

            ui.separator();

            ui.heading("Common properties");
            ui.add_space(8.0);

            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.common_orientation, Orientation::Top, "⬆ Top");
                ui.selectable_value(&mut self.common_orientation, Orientation::Right, "➡ Right");
                ui.selectable_value(
                    &mut self.common_orientation,
                    Orientation::Bottom,
                    "⬇ Bottom",
                );
                ui.selectable_value(&mut self.common_orientation, Orientation::Left, "⬅ Left");

                {
                    let mut is_custom_orientation =
                        matches!(self.common_orientation, Orientation::Custom(..));

                    ui.selectable_value(&mut is_custom_orientation, true, "✏ Custom(..)");

                    if is_custom_orientation
                        && !matches!(self.common_orientation, Orientation::Custom(..))
                    {
                        self.common_orientation = Orientation::Custom(0.0);
                    }

                    if let Orientation::Custom(value) = &mut self.common_orientation {
                        ui.drag_angle(value);
                    }
                }
            });

            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.common_winding, Winding::Clockwise, "⟳ Clockwise");
                ui.selectable_value(
                    &mut self.common_winding,
                    Winding::Counterclockwise,
                    "⟲ Counterclockwise",
                );
            });

            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.common_wrap, WrapMode::None, "🔃 None");

                ui.selectable_value(&mut self.common_wrap, WrapMode::Signed, "± Signed");

                ui.selectable_value(&mut self.common_wrap, WrapMode::Unsigned, "+ Unsigned");
            });

            ui.horizontal(|ui| {
                {
                    let mut snap_enabled = self.common_snap.is_some();
                    ui.toggle_value(&mut snap_enabled, "Snap");

                    self.common_snap = match (snap_enabled, self.common_snap) {
                        (true, None) => Some(15.0f32.to_radians()),
                        (false, Some(_)) => None,
                        _ => self.common_snap,
                    };

                    if let Some(value) = &mut self.common_snap {
                        ui.drag_angle(value);
                        ui.add(DragValue::new(value).speed(0.1));
                        *value = value.max(0.0);
                    }
                }

                {
                    let mut shift_snap_enabled = self.common_shift_snap.is_some();
                    ui.toggle_value(&mut shift_snap_enabled, "Shift snap");

                    self.common_shift_snap = match (shift_snap_enabled, self.common_shift_snap) {
                        (true, None) => Some(15.0f32.to_radians()),
                        (false, Some(_)) => None,
                        _ => self.common_shift_snap,
                    };

                    if let Some(value) = &mut self.common_shift_snap {
                        ui.drag_angle(value);
                        ui.add(DragValue::new(value).speed(0.1));
                        *value = value.max(0.0);
                    }
                }
            });

            ui.horizontal(|ui| {
                {
                    let mut minimum_enabled = self.common_minimum_angle.is_some();
                    ui.toggle_value(&mut minimum_enabled, "Minimum");

                    self.common_minimum_angle = match (minimum_enabled, self.common_minimum_angle) {
                        (true, None) => Some(-360.0f32.to_radians()),
                        (false, Some(_)) => None,
                        _ => self.common_minimum_angle,
                    };

                    if let Some(value) = &mut self.common_minimum_angle {
                        ui.drag_angle(value);
                    }
                }

                {
                    let mut maximum_enabled = self.common_maximum_angle.is_some();
                    ui.toggle_value(&mut maximum_enabled, "Maximum");

                    self.common_maximum_angle = match (maximum_enabled, self.common_maximum_angle) {
                        (true, None) => Some(360.0f32.to_radians()),
                        (false, Some(_)) => None,
                        _ => self.common_maximum_angle,
                    };

                    if let Some(value) = &mut self.common_maximum_angle {
                        ui.drag_angle(value);
                    }
                }
            });

            ui.horizontal(|ui| {
                ui.checkbox(&mut self.common_animated, "Animated");
                ui.checkbox(&mut self.common_interactive, "Interactive");
            });

            ui.add_space(8.0);
            ui.separator();

            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.heading("ThumbstickKnob");
                ui.add_space(8.0);

                ui.horizontal(|ui| {
                    ui.add(
                        DragValue::new(&mut self.thumbstick_knob_value.0)
                            .clamp_range(-1.0..=1.0)
                            .speed(0.05),
                    );
                    ui.add(
                        DragValue::new(&mut self.thumbstick_knob_value.1)
                            .clamp_range(-1.0..=1.0)
                            .speed(0.05),
                    );
                });

                {
                    let thumb_response = ui.add(
                        ThumbstickKnob::new(&mut self.thumbstick_knob_value)
                            .range(-1.0..=1.0)
                            .interactive(self.common_interactive)
                            .animated(self.common_animated),
                    );

                    if thumb_response.drag_released() {
                        self.thumbstick_knob_value = (0.0, 0.0);
                    }
                }

                ui.add_space(8.0);
                ui.separator();

                ui.heading("PolarCompass");
                ui.add_space(8.0);

                ui.horizontal(|ui| {
                    ui.selectable_value(
                        &mut self.polar_compass_overflow,
                        PolarCompassOverflow::Clip,
                        "✂ Clip",
                    );
                    ui.selectable_value(
                        &mut self.polar_compass_overflow,
                        PolarCompassOverflow::Saturate,
                        "➡| Saturate",
                    );
                });

                ui.drag_angle(&mut self.polar_compass_value);

                ui.horizontal(|ui| {
                    ui.add(DragValue::new(&mut self.polar_compass_max_distance));
                    ui.add(DragValue::new(&mut self.polar_compass_scale_log_base));
                    ui.add(DragValue::new(&mut self.polar_compass_scale_log_mult));
                });

                ui.horizontal(|ui| {
                    ui.add(DragValue::new(&mut self.polar_compass_diameter));
                    ui.add(DragValue::new(&mut self.polar_compass_label_height));
                });

                ui.horizontal(|ui| {
                    ui.add(DragValue::new(&mut self.polar_compass_marker_near_size));
                    ui.add(DragValue::new(&mut self.polar_compass_marker_far_size));
                });

                ui.horizontal(|ui| {
                    ui.checkbox(&mut self.polar_compass_show_axes, "Show axes");
                    ui.checkbox(&mut self.polar_compass_show_rings, "Show rings");
                    ui.checkbox(&mut self.polar_compass_show_cursor, "Show cursor");
                    ui.checkbox(
                        &mut self.polar_compass_show_marker_labels,
                        "Show marker labels",
                    );
                    ui.checkbox(
                        &mut self.polar_compass_show_marker_lines,
                        "Show marker lines",
                    );
                });

                ui.add_space(8.0);

                ui.add(
                    PolarCompass::new(&mut self.polar_compass_value)
                        .interactive(self.common_interactive)
                        .orientation(self.common_orientation)
                        .winding(self.common_winding)
                        .overflow(self.polar_compass_overflow)
                        .diameter(self.polar_compass_diameter)
                        .wrap(self.common_wrap)
                        .min(self.common_minimum_angle)
                        .max(self.common_maximum_angle)
                        .snap(self.common_snap)
                        .shift_snap(self.common_shift_snap)
                        .animated(self.common_animated)
                        .labels(["N", "E", "S", "W"])
                        .label_height(self.polar_compass_label_height)
                        .max_distance(self.polar_compass_max_distance)
                        .scale_log_base(self.polar_compass_scale_log_base)
                        .scale_log_mult(self.polar_compass_scale_log_mult)
                        .marker_near_size(self.polar_compass_marker_near_size)
                        .marker_far_size(self.polar_compass_marker_far_size)
                        .show_axes(self.polar_compass_show_axes)
                        .show_rings(self.polar_compass_show_rings)
                        .show_cursor(self.polar_compass_show_cursor)
                        .show_marker_labels(self.polar_compass_show_marker_labels)
                        .show_marker_lines(self.polar_compass_show_marker_lines)
                        .markers(&[
                            CompassMarker::new(0.0f32.to_radians())
                                .distance(20.0)
                                .color(Color32::from_rgb(0xF0, 0xBF, 0x89))
                                .shape(CompassMarkerShape::Diamond)
                                .label("Haibara"),
                            CompassMarker::new(15.0f32.to_radians())
                                .distance(200.0)
                                .color(Color32::from_rgb(0x9C, 0xCF, 0xEE))
                                .shape(CompassMarkerShape::DownArrow)
                                .label("Mitsuhiko"),
                            CompassMarker::new(30.0f32.to_radians())
                                .distance(2000.0)
                                .color(Color32::from_rgb(0x8A, 0xDC, 0x71))
                                .shape(CompassMarkerShape::Circle)
                                .label("Genta"),
                            CompassMarker::new(45.0f32.to_radians())
                                .distance(20000.0)
                                .color(Color32::from_rgb(0xEF, 0xBB, 0xC4))
                                .shape(CompassMarkerShape::UpArrow)
                                .label("Ayumi"),
                        ]),
                );

                ui.add_space(8.0);
                ui.separator();

                ui.heading("SegmentedDisplayWidget");
                ui.add_space(8.0);

                ui.add(egui::Slider::new(
                    &mut self.segmented_display_metrics.segment_thickness,
                    0.0..=1.0,
                ));
                ui.add(egui::Slider::new(
                    &mut self.segmented_display_metrics.segment_spacing,
                    0.0..=1.0,
                ));
                ui.add(egui::Slider::new(
                    &mut self.segmented_display_metrics.digit_shearing,
                    -1.0..=1.0,
                ));
                ui.add(egui::Slider::new(
                    &mut self.segmented_display_digit_height,
                    16.0..=256.0,
                ));
                ui.add(egui::Slider::new(
                    &mut self.segmented_display_metrics.digit_ratio,
                    0.25..=1.0,
                ));
                ui.add(egui::Slider::new(
                    &mut self.segmented_display_metrics.digit_spacing,
                    0.0..=1.0,
                ));
                ui.add(egui::Slider::new(
                    &mut self.segmented_display_metrics.digit_median,
                    -1.0..=1.0,
                ));
                ui.add(egui::Slider::new(
                    &mut self.segmented_display_metrics.margin_horizontal,
                    0.0..=1.0,
                ));
                ui.add(egui::Slider::new(
                    &mut self.segmented_display_metrics.margin_vertical,
                    0.0..=1.0,
                ));
                ui.add(egui::Slider::new(
                    &mut self.segmented_display_metrics.colon_separation,
                    0.0..=1.0,
                ));

                ui.color_edit_button_srgba(&mut self.segmented_display_style.background_color);

                ui.horizontal(|ui| {
                    ui.add(
                        DragValue::new(&mut self.segmented_display_style.segment_on_stroke.width)
                            .speed(0.1),
                    );
                    ui.color_edit_button_srgba(
                        &mut self.segmented_display_style.segment_on_stroke.color,
                    );
                    ui.color_edit_button_srgba(&mut self.segmented_display_style.segment_on_color);
                });

                ui.horizontal(|ui| {
                    ui.add(
                        DragValue::new(&mut self.segmented_display_style.segment_off_stroke.width)
                            .speed(0.1),
                    );
                    ui.color_edit_button_srgba(
                        &mut self.segmented_display_style.segment_off_stroke.color,
                    );
                    ui.color_edit_button_srgba(&mut self.segmented_display_style.segment_off_color);
                });

                ui.horizontal(|ui| {
                    ui.toggle_value(&mut self.segmented_display_show_dots, "Dots");
                    ui.toggle_value(&mut self.segmented_display_show_colons, "Colons");
                    ui.toggle_value(&mut self.segmented_display_show_apostrophes, "Apostrophes");
                });

                ui.horizontal(|ui| {
                    ui.selectable_value(
                        &mut self.segmented_display_display_kind,
                        DisplayKind::SevenSegment,
                        "7-segment",
                    );
                    ui.selectable_value(
                        &mut self.segmented_display_display_kind,
                        DisplayKind::SixteenSegment,
                        "16-segment",
                    );
                });

                ui.add(egui::TextEdit::singleline(
                    &mut self.segmented_display_display_string,
                ));

                ui.add_space(8.0);

                ui.add(
                    SegmentedDisplayWidget::new(self.segmented_display_display_kind)
                        .style(self.segmented_display_style)
                        .metrics(self.segmented_display_metrics)
                        .digit_height(self.segmented_display_digit_height)
                        .show_dots(self.segmented_display_show_dots)
                        .show_colons(self.segmented_display_show_colons)
                        .show_apostrophes(self.segmented_display_show_apostrophes)
                        .push_string(&self.segmented_display_display_string),
                );

                ui.add_space(8.0);
                ui.separator();

                ui.heading("AudioKnob");
                ui.add_space(8.0);
                ui.add(egui::Slider::new(&mut self.audio_knob_value, -1.0..=1.0));
                ui.add(egui::Slider::new(&mut self.audio_knob_spread, 0.0..=1.0));
                ui.add(egui::Slider::new(&mut self.audio_knob_thickness, 0.0..=1.0));

                ui.add_space(8.0);

                ui.horizontal(|ui| {
                    for (audio_knob_range, audio_knob_size) in [0.0..=1.0, -1.0..=1.0]
                        .into_iter()
                        .cartesian_product([64.0, 32.0])
                    {
                        ui.add(
                            AudioKnob::new(&mut self.audio_knob_value)
                                .range(audio_knob_range)
                                .interactive(self.common_interactive)
                                .diameter(audio_knob_size)
                                .orientation(self.common_orientation)
                                .winding(self.common_winding)
                                .spread(self.audio_knob_spread)
                                .thickness(self.audio_knob_thickness)
                                .shape(WidgetShape::Squircle(4.0))
                                .animated(self.common_animated)
                                .snap(self.common_snap)
                                .shift_snap(self.common_shift_snap),
                        );
                    }
                });

                ui.add_space(8.0);
                ui.separator();

                ui.heading("AngleKnob");
                ui.add_space(8.0);

                ui.drag_angle(&mut self.angle_knob_value);
                ui.add_space(8.0);

                ui.horizontal(|ui| {
                    for angle_knob_size in [64.0, 32.0] {
                        ui.add(
                            AngleKnob::new(&mut self.angle_knob_value)
                                .interactive(self.common_interactive)
                                .diameter(angle_knob_size)
                                .orientation(self.common_orientation)
                                .winding(self.common_winding)
                                .shape(WidgetShape::Circle)
                                .wrap(self.common_wrap)
                                .min(self.common_minimum_angle)
                                .max(self.common_maximum_angle)
                                .snap(self.common_snap)
                                .shift_snap(self.common_shift_snap)
                                .animated(self.common_animated)
                                .show_axes(true)
                                .axis_count(4),
                        );
                    }
                });

                ui.add_space(8.0);
                ui.separator();

                ui.heading("LinearCompass");
                ui.add_space(8.0);

                ui.drag_angle(&mut self.linear_compass_value);
                ui.drag_angle(&mut self.linear_compass_spread);
                ui.checkbox(&mut self.linear_compass_show_cursor, "Show cursor");
                ui.add_space(8.0);

                ui.add(
                    LinearCompass::new(&mut self.linear_compass_value)
                        .interactive(self.common_interactive)
                        .wrap(self.common_wrap)
                        .winding(self.common_winding)
                        .width(512.0)
                        .height(48.0)
                        .spread(self.linear_compass_spread)
                        .labels(["N", "E", "S", "W"])
                        .snap(self.common_snap)
                        .shift_snap(self.common_shift_snap)
                        .min(self.common_minimum_angle)
                        .max(self.common_maximum_angle)
                        .animated(self.common_animated)
                        .show_cursor(self.linear_compass_show_cursor)
                        .markers(&[
                            CompassMarker::new(0.0f32.to_radians())
                                .shape(CompassMarkerShape::Star(5, 0.5))
                                .label("Test")
                                .color(Color32::from_rgb(0x00, 0xA0, 0x00)),
                            // Grand Theft Auto style markers
                            CompassMarker::new(70.0f32.to_radians())
                                .shape(CompassMarkerShape::Square)
                                .label("Sweet")
                                .color(Color32::from_rgb(0x00, 0x00, 0xFF)),
                            CompassMarker::new(85.0f32.to_radians())
                                .shape(CompassMarkerShape::DownArrow)
                                .label("Reece's")
                                .color(Color32::from_rgb(0xFF, 0xFF, 0x00)),
                            CompassMarker::new(100.0f32.to_radians())
                                .shape(CompassMarkerShape::UpArrow)
                                .label("Big Smoke")
                                .color(Color32::from_rgb(0xFF, 0x00, 0x00)),
                            // Emoji markers
                            CompassMarker::new(553.0f32.to_radians())
                                .shape(CompassMarkerShape::Emoji('🐱'))
                                .label("Cat")
                                .color(Color32::from_rgb(0xF8, 0xE9, 0xFF)),
                            CompassMarker::new(563.0f32.to_radians())
                                .shape(CompassMarkerShape::Emoji('🐶'))
                                .label("Dog")
                                .color(Color32::from_rgb(0xC0, 0x8C, 0x85)),
                            // All marker shapes
                            CompassMarker::new(240.0f32.to_radians())
                                .shape(CompassMarkerShape::Square),
                            CompassMarker::new(250.0f32.to_radians())
                                .shape(CompassMarkerShape::Circle),
                            CompassMarker::new(260.0f32.to_radians())
                                .shape(CompassMarkerShape::RightArrow),
                            CompassMarker::new(270.0f32.to_radians())
                                .shape(CompassMarkerShape::UpArrow),
                            CompassMarker::new(280.0f32.to_radians())
                                .shape(CompassMarkerShape::LeftArrow),
                            CompassMarker::new(290.0f32.to_radians())
                                .shape(CompassMarkerShape::DownArrow),
                            CompassMarker::new(300.0f32.to_radians())
                                .shape(CompassMarkerShape::Diamond),
                            CompassMarker::new(310.0f32.to_radians())
                                .shape(CompassMarkerShape::Star(5, 0.5)),
                            CompassMarker::new(320.0f32.to_radians())
                                .shape(CompassMarkerShape::Emoji('🗿')),
                            // Transparent colors
                            CompassMarker::new(30.0f32.to_radians())
                                .shape(CompassMarkerShape::Square)
                                .label("Near")
                                .color(Color32::from_rgb(0x40, 0x80, 0x80).linear_multiply(1.0)),
                            CompassMarker::new(40.0f32.to_radians())
                                .shape(CompassMarkerShape::Square)
                                .label("Far")
                                .color(Color32::from_rgb(0x40, 0x80, 0x80).linear_multiply(0.5)),
                            CompassMarker::new(50.0f32.to_radians())
                                .shape(CompassMarkerShape::Square)
                                .label("Very far")
                                .color(Color32::from_rgb(0x40, 0x80, 0x80).linear_multiply(0.25)),
                        ]),
                );

                ui.add_space(8.0);
                ui.separator();
            });
        });
    }
}

fn main() {
    let options = eframe::NativeOptions::default();

    eframe::run_native(
        "Knobs",
        options,
        Box::new(|cc| {
            cc.egui_ctx.set_style(Style {
                visuals: Visuals::dark(),
                ..Style::default()
            });

            Box::new(EguiExtrasXtExampleApp::default())
        }),
    );
}
