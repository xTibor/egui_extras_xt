use std::ops::RangeInclusive;

use eframe::egui::{DragValue, Grid, Ui};
use egui_extras_xt::common::{Orientation, WidgetShape, Winding};
use egui_extras_xt::knobs::AudioKnob;
use egui_extras_xt::ui::drag_rangeinclusive::DragRangeInclusive;
use egui_extras_xt::ui::optional_value_widget::OptionalValueWidget;
use egui_extras_xt::ui::widgets_from::{WidgetsFromIterator, WidgetsFromSlice};
use strum::IntoEnumIterator;

use crate::pages::ui::{widget_orientation_ui, widget_shape_ui};
use crate::pages::PageImpl;

pub struct AudioKnobPage<'a> {
    value: f32,
    interactive: bool,
    diameter: f32,
    winding: Winding,
    orientation: Orientation,
    range: RangeInclusive<f32>,
    spread: f32,
    thickness: f32,
    shape: WidgetShape<'a>,
    animated: bool,
    snap: Option<f32>,
    shift_snap: Option<f32>,
}

impl<'a> Default for AudioKnobPage<'a> {
    fn default() -> AudioKnobPage<'a> {
        AudioKnobPage {
            value: 0.0,
            interactive: true,
            diameter: 32.0,
            orientation: Orientation::Top,
            winding: Winding::Clockwise,
            range: 0.0..=1.0,
            spread: 1.0,
            thickness: 0.66,
            shape: WidgetShape::Squircle(4.0),
            animated: true,
            snap: None,
            shift_snap: None,
        }
    }
}

impl<'a> PageImpl for AudioKnobPage<'a> {
    fn ui(&mut self, ui: &mut Ui) {
        ui.add(
            AudioKnob::new(&mut self.value)
                .interactive(self.interactive)
                .diameter(self.diameter)
                .orientation(self.orientation)
                .winding(self.winding)
                .range(self.range.clone())
                .spread(self.spread)
                .thickness(self.thickness)
                .shape(self.shape.clone())
                .animated(self.animated)
                .snap(self.snap)
                .shift_snap(self.shift_snap),
        );
        ui.separator();

        Grid::new("audio_knob_properties")
            .num_columns(2)
            .spacing([20.0, 10.0])
            .striped(true)
            .show(ui, |ui| {
                ui.label("Value");
                ui.add(DragValue::new(&mut self.value));
                ui.end_row();

                ui.label("Interactive");
                ui.checkbox(&mut self.interactive, "");
                ui.end_row();

                ui.label("Diameter");
                ui.add(DragValue::new(&mut self.diameter));
                ui.end_row();

                ui.label("Winding");
                ui.horizontal(|ui| {
                    ui.selectable_value_from_iter(&mut self.winding, Winding::iter());
                });
                ui.end_row();

                ui.label("Orientation");
                widget_orientation_ui(ui, &mut self.orientation);
                ui.end_row();

                ui.label("Range");
                ui.drag_rangeinclusive(&mut self.range);
                ui.end_row();

                ui.label("Spread");
                ui.add(DragValue::new(&mut self.spread));
                ui.end_row();

                ui.label("Thickness");
                ui.add(DragValue::new(&mut self.thickness));
                ui.end_row();

                ui.label("Shape");
                widget_shape_ui(ui, &mut self.shape);
                ui.end_row();

                ui.label("Animated");
                ui.checkbox(&mut self.animated, "");
                ui.end_row();

                ui.label("Snap");
                ui.optional_value_widget(&mut self.snap, |ui, value| ui.add(DragValue::new(value)));
                ui.end_row();

                ui.label("Shift snap");
                ui.optional_value_widget(&mut self.shift_snap, |ui, value| {
                    ui.add(DragValue::new(value))
                });
                ui.end_row();
            });
    }
}
