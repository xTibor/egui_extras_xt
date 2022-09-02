use std::f32::consts::TAU;

use eframe::egui::{DragValue, Grid, Ui};
use egui_extras_xt::common::{Orientation, WidgetShape, Winding, WrapMode};
use egui_extras_xt::knobs::{AngleKnob, AngleKnobPreset};
use egui_extras_xt::ui::optional_value_widget::OptionalValueWidget;
use egui_extras_xt::ui::widgets_from::WidgetsFromIterator;
use strum::IntoEnumIterator;

use crate::pages::ui::{widget_orientation_ui, widget_shape_ui};
use crate::pages::PageImpl;

pub struct AngleKnobPage<'a> {
    value: f32,
    interactive: bool,
    diameter: f32,
    preset: AngleKnobPreset,
    orientation: Orientation,
    winding: Winding,
    wrap: WrapMode,
    shape: WidgetShape<'a>,
    min: Option<f32>,
    max: Option<f32>,
    snap: Option<f32>,
    shift_snap: Option<f32>,
    animated: bool,
    show_axes: bool,
    axis_count: usize,
}

impl<'a> Default for AngleKnobPage<'a> {
    fn default() -> AngleKnobPage<'a> {
        AngleKnobPage {
            value: 0.0,
            preset: AngleKnobPreset::AdobePhotoshop,
            interactive: true,
            diameter: 32.0,
            orientation: Orientation::Top,
            winding: Winding::Clockwise,
            wrap: WrapMode::Unsigned,
            shape: WidgetShape::Circle,
            min: None,
            max: None,
            snap: None,
            shift_snap: Some(TAU / 24.0),
            animated: false,
            show_axes: true,
            axis_count: 4,
        }
    }
}

impl<'a> PageImpl for AngleKnobPage<'a> {
    fn ui(&mut self, ui: &mut Ui) {
        ui.add(
            AngleKnob::new(&mut self.value)
                .interactive(self.interactive)
                .diameter(self.diameter)
                .orientation(self.orientation)
                .winding(self.winding)
                .shape(self.shape.clone())
                .wrap(self.wrap)
                .min(self.min)
                .max(self.max)
                .snap(self.snap)
                .shift_snap(self.shift_snap)
                .animated(self.animated)
                .show_axes(self.show_axes)
                .axis_count(self.axis_count),
        );
        ui.separator();

        Grid::new("angle_knob_properties")
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

                ui.label("Diameter");
                ui.add(DragValue::new(&mut self.diameter));
                ui.end_row();

                ui.label("Preset");
                ui.horizontal(|ui| {
                    ui.combobox_from_iter("", &mut self.preset, AngleKnobPreset::iter());
                    if ui.button("\u{2714} Apply").clicked() {
                        (self.orientation, self.winding, self.wrap) = self.preset.properties();
                    }
                });
                ui.end_row();

                ui.label("Orientation");
                widget_orientation_ui(ui, &mut self.orientation);
                ui.end_row();

                ui.label("Winding");
                ui.horizontal(|ui| {
                    ui.selectable_value_from_iter(&mut self.winding, Winding::iter());
                });
                ui.end_row();

                ui.label("Wrap");
                ui.horizontal(|ui| {
                    ui.selectable_value_from_iter(&mut self.wrap, WrapMode::iter());
                });
                ui.end_row();

                ui.label("Shape");
                widget_shape_ui(ui, &mut self.shape);
                ui.end_row();

                ui.label("Minimum");
                ui.optional_value_widget(&mut self.min, |ui, value| ui.drag_angle(value));
                ui.end_row();

                ui.label("Maximum");
                ui.optional_value_widget(&mut self.max, |ui, value| ui.drag_angle(value));
                ui.end_row();

                ui.label("Snap");
                ui.optional_value_widget(&mut self.snap, |ui, value| ui.drag_angle(value));
                ui.end_row();

                ui.label("Shift snap");
                ui.optional_value_widget(&mut self.shift_snap, |ui, value| ui.drag_angle(value));
                ui.end_row();

                ui.label("Animated");
                ui.checkbox(&mut self.animated, "");
                ui.end_row();

                ui.label("Show axes");
                ui.checkbox(&mut self.show_axes, "");
                ui.end_row();

                ui.label("Axis count");
                ui.add(DragValue::new(&mut self.axis_count));
                ui.end_row();
            });
    }
}
