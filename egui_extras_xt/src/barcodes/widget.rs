use egui::{vec2, Color32, Rect, Response, Sense, Stroke, Ui, Vec2, Widget};

#[must_use = "You should put this widget in an ui with `ui.add(widget);`"]
pub struct BarcodeWidget {
    value: String,
    padding: f32,
    bar_width: f32,
    bar_height: f32,
}

impl BarcodeWidget {
    pub fn new(value: &str) -> Self {
        Self {
            value: value.to_owned(),
            bar_width: 2.0,
            bar_height: 64.0,
            padding: 12.0,
        }
    }
}

impl Widget for BarcodeWidget {
    fn ui(self, ui: &mut Ui) -> Response {
        let barcode = match barcoders::sym::code39::Code39::new(&self.value) {
            Ok(code39) => code39.encode(),
            _ => Vec::new(),
        };

        let bar_width = self.bar_width / ui.ctx().pixels_per_point();
        let bar_height = self.bar_height / ui.ctx().pixels_per_point();
        let padding = self.padding / ui.ctx().pixels_per_point();

        let desired_size =
            vec2(bar_width * barcode.len() as f32, bar_height) + Vec2::splat(padding) * 2.0;

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
                            ui.painter().round_pos_to_pixels(rect.left_top())
                                + Vec2::splat(padding)
                                + vec2(bar_width * i as f32, 0.0),
                            vec2(bar_width, bar_height),
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
