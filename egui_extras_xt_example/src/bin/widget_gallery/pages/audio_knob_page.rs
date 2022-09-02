use std::ops::RangeInclusive;

use eframe::egui::{DragValue, Grid, Ui};
use egui_extras_xt::common::{Orientation, Winding};
use egui_extras_xt::knobs::AudioKnob;
use egui_extras_xt::ui::drag_option_value::DragOptionValue;
use egui_extras_xt::ui::drag_rangeinclusive::DragRangeInclusive;
use egui_extras_xt::ui::widgets_from::{WidgetsFromIterator, WidgetsFromSlice};
use strum::IntoEnumIterator;

use crate::pages::PageImpl;

pub struct AudioKnobPage {
    value: f32,
    interactive: bool,
    diameter: f32,
    winding: Winding,
    orientation: Orientation,
    range: RangeInclusive<f32>,
    spread: f32,
    thickness: f32,
    //shape: WidgetShape<'_>,
    animated: bool,
    snap: Option<f32>,
    shift_snap: Option<f32>,
}

impl Default for AudioKnobPage {
    fn default() -> AudioKnobPage {
        AudioKnobPage {
            value: 0.0,
            interactive: true,
            diameter: 32.0,
            orientation: Orientation::Top,
            winding: Winding::Clockwise,
            range: 0.0..=1.0,
            spread: 1.0,
            thickness: 0.66,
            //shape: ,
            animated: true,
            snap: None,
            shift_snap: None,
        }
    }
}

impl PageImpl for AudioKnobPage {
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
                //.shape(self.shape)
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

                // TODO: Orientation::Custom
                ui.label("Orientation");
                ui.horizontal(|ui| {
                    ui.selectable_value_from_slice(
                        &mut self.orientation,
                        &[
                            Orientation::Top,
                            Orientation::Bottom,
                            Orientation::Left,
                            Orientation::Right,
                        ],
                    );
                });
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

                // TODO: Shape

                ui.label("Animated");
                ui.checkbox(&mut self.animated, "");
                ui.end_row();

                ui.label("Snap");
                ui.drag_option_value(&mut self.snap);
                ui.end_row();

                ui.label("Shift snap");
                ui.drag_option_value(&mut self.shift_snap);
                ui.end_row();
            });
    }
}
