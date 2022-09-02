use eframe::egui::{DragValue, Grid, Ui};
use egui_extras_xt::displays::segmented_display::DisplayMetricsPreset;
use egui_extras_xt::displays::{
    DisplayKind, DisplayMetrics, DisplayStyle, DisplayStylePreset, SegmentedDisplayWidget,
};
use egui_extras_xt::ui::widgets_from::WidgetsFromIterator;
use strum::IntoEnumIterator;

use crate::pages::ui::{display_metrics_ui, display_style_ui};
use crate::pages::PageImpl;

pub struct SegmentedDisplayPage {
    value: String,
    display_kind: DisplayKind,
    digit_height: f32,
    metrics: DisplayMetrics,
    metrics_preset: DisplayMetricsPreset,
    style: DisplayStyle,
    style_preset: DisplayStylePreset,
    show_dots: bool,
    show_colons: bool,
    show_apostrophes: bool,
}

impl Default for SegmentedDisplayPage {
    fn default() -> SegmentedDisplayPage {
        SegmentedDisplayPage {
            value: "EGUI_EXTRAS_XT".to_owned(),
            display_kind: DisplayKind::SixteenSegment,
            digit_height: 80.0,
            metrics: DisplayMetricsPreset::Default.metrics(),
            metrics_preset: DisplayMetricsPreset::Default,
            style: DisplayStylePreset::Default.style(),
            style_preset: DisplayStylePreset::Default,
            show_dots: true,
            show_colons: true,
            show_apostrophes: true,
        }
    }
}

impl PageImpl for SegmentedDisplayPage {
    fn ui(&mut self, ui: &mut Ui) {
        ui.add(
            SegmentedDisplayWidget::new(self.display_kind)
                .digit_height(self.digit_height)
                .metrics(self.metrics)
                .style(self.style)
                .show_dots(self.show_dots)
                .show_colons(self.show_colons)
                .show_apostrophes(self.show_apostrophes)
                .push_string(&self.value),
        );
        ui.separator();

        Grid::new("segmented_display_properties")
            .num_columns(2)
            .spacing([20.0, 10.0])
            .striped(true)
            .show(ui, |ui| {
                ui.label("Value");
                ui.text_edit_singleline(&mut self.value);
                ui.end_row();

                ui.label("Display kind");
                ui.horizontal(|ui| {
                    ui.selectable_value_from_iter(&mut self.display_kind, DisplayKind::iter());
                });
                ui.end_row();

                ui.label("Digit height");
                ui.add(DragValue::new(&mut self.digit_height));
                ui.end_row();

                ui.label("Metrics");
                display_metrics_ui(ui, &mut self.metrics, &mut self.metrics_preset);
                ui.end_row();

                ui.label("Style");
                display_style_ui(ui, &mut self.style, &mut self.style_preset);
                ui.end_row();

                ui.label("Show dots");
                ui.checkbox(&mut self.show_dots, "");
                ui.end_row();

                ui.label("Show colons");
                ui.checkbox(&mut self.show_colons, "");
                ui.end_row();

                ui.label("Show apostrophes");
                ui.checkbox(&mut self.show_apostrophes, "");
                ui.end_row();
            });
    }
}
