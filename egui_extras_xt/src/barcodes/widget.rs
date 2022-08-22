use egui::{
    vec2, Align2, Color32, FontFamily, FontId, Rect, Response, Sense, Stroke, Ui, Vec2, Widget,
};

use crate::barcodes::BarcodeKind;

#[must_use = "You should put this widget in an ui with `ui.add(widget);`"]
pub struct BarcodeWidget<'a> {
    value: &'a str,
    barcode_kind: BarcodeKind,
    horizontal_padding: f32,
    vertical_padding: f32,
    bar_width: usize,
    bar_height: f32,
    label: Option<&'a str>,
    label_height: f32,
    label_top_margin: f32,
}

impl<'a> BarcodeWidget<'a> {
    pub fn new(value: &'a str) -> Self {
        Self {
            value,
            barcode_kind: BarcodeKind::Code39,
            bar_width: 2,
            bar_height: 64.0,
            horizontal_padding: 50.0,
            vertical_padding: 10.0,
            label: None,
            label_height: 20.0,
            label_top_margin: 4.0,
        }
    }

    pub fn barcode_kind(mut self, barcode_kind: BarcodeKind) -> Self {
        self.barcode_kind = barcode_kind;
        self
    }

    pub fn bar_width(mut self, bar_width: impl Into<usize>) -> Self {
        self.bar_width = bar_width.into();
        self
    }

    pub fn bar_height(mut self, bar_height: impl Into<f32>) -> Self {
        self.bar_height = bar_height.into();
        self
    }

    pub fn horizontal_padding(mut self, horizontal_padding: impl Into<f32>) -> Self {
        self.horizontal_padding = horizontal_padding.into();
        self
    }

    pub fn vertical_padding(mut self, vertical_padding: impl Into<f32>) -> Self {
        self.vertical_padding = vertical_padding.into();
        self
    }

    pub fn label(mut self, label: &'a str) -> Self {
        self.label = Some(label);
        self
    }

    pub fn label_height(mut self, label_height: impl Into<f32>) -> Self {
        self.label_height = label_height.into();
        self
    }

    pub fn label_top_margin(mut self, label_top_margin: impl Into<f32>) -> Self {
        self.label_top_margin = label_top_margin.into();
        self
    }
}

impl<'a> Widget for BarcodeWidget<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        let barcode = self.barcode_kind.encode(self.value).unwrap_or_default();
        let bar_width = self.bar_width as f32 / ui.ctx().pixels_per_point();

        let desired_size = {
            let mut size = vec2(bar_width * barcode.len() as f32, self.bar_height)
                + vec2(self.horizontal_padding, self.vertical_padding) * 2.0;

            if self.label.is_some() {
                size += vec2(0.0, self.label_height + self.label_top_margin)
            }

            size
        };

        let (rect, response) = ui.allocate_exact_size(desired_size, Sense::hover());

        if ui.is_rect_visible(rect) {
            ui.painter().rect(
                rect,
                ui.style().visuals.noninteractive().rounding,
                Color32::WHITE,
                Stroke::none(),
            );

            barcode
                .into_iter()
                .enumerate()
                .filter(|&(_bar_index, bar_value)| bar_value == 1)
                .for_each(|(bar_index, _bar_value)| {
                    ui.painter().rect(
                        Rect::from_min_size(
                            ui.painter().round_pos_to_pixels(
                                rect.left_top()
                                    + vec2(self.horizontal_padding, self.vertical_padding),
                            ) + vec2(bar_width * bar_index as f32, 0.0),
                            vec2(bar_width, self.bar_height),
                        ),
                        0.0,
                        Color32::BLACK,
                        Stroke::none(),
                    );
                });

            if let Some(label) = self.label {
                ui.painter().text(
                    rect.center_bottom() - vec2(0.0, self.vertical_padding),
                    Align2::CENTER_BOTTOM,
                    label,
                    FontId::new(self.label_height, FontFamily::Proportional),
                    Color32::BLACK,
                );
            }
        }

        response
    }
}
