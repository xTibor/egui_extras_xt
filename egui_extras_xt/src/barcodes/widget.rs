use egui::{vec2, Color32, Rect, Response, Sense, Stroke, Ui, Vec2, Widget};

use crate::barcodes::BarcodeKind;

#[must_use = "You should put this widget in an ui with `ui.add(widget);`"]
pub struct BarcodeWidget {
    value: String,
    barcode_kind: BarcodeKind,
    padding: f32,
    bar_width: usize,
    bar_height: f32,
}

impl BarcodeWidget {
    pub fn new(value: &str) -> Self {
        Self {
            value: value.to_owned(),
            barcode_kind: BarcodeKind::Code39,
            bar_width: 2,
            bar_height: 64.0,
            padding: 12.0,
        }
    }

    pub fn barcode_kind(mut self, barcode_kind: BarcodeKind) -> Self {
        self.barcode_kind = barcode_kind;
        self
    }
}

impl Widget for BarcodeWidget {
    fn ui(self, ui: &mut Ui) -> Response {
        let barcode = self.barcode_kind.encode(&self.value);

        let bar_width = self.bar_width as f32 / ui.ctx().pixels_per_point();

        let desired_size = vec2(bar_width * barcode.len() as f32, self.bar_height)
            + Vec2::splat(self.padding) * 2.0;

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
                .filter(|&(i, b)| b == 1)
                .for_each(|(i, b)| {
                    ui.painter().rect(
                        Rect::from_min_size(
                            ui.painter()
                                .round_pos_to_pixels(rect.left_top() + Vec2::splat(self.padding))
                                + vec2(bar_width * i as f32, 0.0),
                            vec2(bar_width, self.bar_height),
                        ),
                        0.0,
                        Color32::BLACK,
                        Stroke::none(),
                    );
                });
        }

        response
    }
}
