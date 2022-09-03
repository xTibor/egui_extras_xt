use eframe::egui::{DragValue, Grid, Ui};
use eframe::epaint::Color32;
use egui_extras_xt::common::{Winding, WrapMode};
use egui_extras_xt::compasses::{
    CompassMarker, CompassMarkerShape, DefaultCompassMarkerColor, LinearCompass,
};
use egui_extras_xt::ui::optional_value_widget::OptionalValueWidget;
use egui_extras_xt::ui::widgets_from::WidgetsFromIterator;
use strum::IntoEnumIterator;

use crate::pages::PageImpl;

pub struct LinearCompassPage {
    value: f32,
    interactive: bool,
    wrap: WrapMode,
    winding: Winding,
    width: f32,
    height: f32,
    spread: f32,
    //labels: CompassLabels<'a>,
    snap: Option<f32>,
    shift_snap: Option<f32>,
    min: Option<f32>,
    max: Option<f32>,
    animated: bool,
    show_cursor: bool,
    default_marker_color: DefaultCompassMarkerColor,
    default_marker_shape: CompassMarkerShape,
}

impl Default for LinearCompassPage {
    fn default() -> LinearCompassPage {
        LinearCompassPage {
            value: 0.0,
            interactive: true,
            wrap: WrapMode::Unsigned,
            winding: Winding::Clockwise,
            width: 512.0,
            height: 48.0,
            spread: 180.0f32.to_radians(),
            //labels: ["N", "E", "S", "W"],
            snap: None,
            shift_snap: Some(10.0f32.to_radians()),
            min: None,
            max: None,
            animated: false,
            show_cursor: true,
            default_marker_color: DefaultCompassMarkerColor::HsvByAngle {
                saturation: 1.0,
                value: 1.0,
            },
            default_marker_shape: CompassMarkerShape::Square,
        }
    }
}

impl PageImpl for LinearCompassPage {
    fn ui(&mut self, ui: &mut Ui) {
        ui.add(
            LinearCompass::new(&mut self.value)
                .interactive(self.interactive)
                .wrap(self.wrap)
                .winding(self.winding)
                .width(self.width)
                .height(self.height)
                .spread(self.spread)
                .snap(self.snap)
                .shift_snap(self.shift_snap)
                .min(self.min)
                .max(self.max)
                .animated(self.animated)
                .show_cursor(self.show_cursor)
                .default_marker_color(self.default_marker_color)
                .default_marker_shape(self.default_marker_shape)
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
                    CompassMarker::new(240.0f32.to_radians()).shape(CompassMarkerShape::Square),
                    CompassMarker::new(250.0f32.to_radians()).shape(CompassMarkerShape::Circle),
                    CompassMarker::new(260.0f32.to_radians()).shape(CompassMarkerShape::RightArrow),
                    CompassMarker::new(270.0f32.to_radians()).shape(CompassMarkerShape::UpArrow),
                    CompassMarker::new(280.0f32.to_radians()).shape(CompassMarkerShape::LeftArrow),
                    CompassMarker::new(290.0f32.to_radians()).shape(CompassMarkerShape::DownArrow),
                    CompassMarker::new(300.0f32.to_radians()).shape(CompassMarkerShape::Diamond),
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
        ui.separator();

        Grid::new("linear_compass_properties")
            .num_columns(2)
            .spacing([20.0, 10.0])
            .striped(true)
            .show(ui, |ui| {
                ui.label("Value");
                ui.drag_angle(&mut self.value);
                ui.end_row();

                ui.label("Interactive");
                ui.checkbox(&mut self.interactive, "");
                ui.end_row();

                ui.label("Wrap");
                ui.horizontal(|ui| {
                    ui.selectable_value_from_iter(&mut self.wrap, WrapMode::iter());
                });
                ui.end_row();

                ui.label("Winding");
                ui.horizontal(|ui| {
                    ui.selectable_value_from_iter(&mut self.winding, Winding::iter());
                });
                ui.end_row();

                ui.label("Width");
                ui.add(DragValue::new(&mut self.width));
                ui.end_row();

                ui.label("Height");
                ui.add(DragValue::new(&mut self.height));
                ui.end_row();

                ui.label("Spread");
                ui.drag_angle(&mut self.spread);
                ui.end_row();

                // TODO: self.labels

                ui.label("Snap");
                ui.optional_value_widget(&mut self.snap, |ui, value| ui.drag_angle(value));
                ui.end_row();

                ui.label("Shift snap");
                ui.optional_value_widget(&mut self.shift_snap, |ui, value| ui.drag_angle(value));
                ui.end_row();

                ui.label("Minimum");
                ui.optional_value_widget(&mut self.min, |ui, value| ui.drag_angle(value));
                ui.end_row();

                ui.label("Maximum");
                ui.optional_value_widget(&mut self.max, |ui, value| ui.drag_angle(value));
                ui.end_row();

                ui.label("Animated");
                ui.checkbox(&mut self.animated, "");
                ui.end_row();

                ui.label("Show cursor");
                ui.checkbox(&mut self.show_cursor, "");
                ui.end_row();

                // TODO: self.default_marker_color

                // TODO: self.default_marker_shape
            });
    }
}
