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

        let desired_size = vec2(self.bar_width * barcode.len() as f32, self.bar_height)
            + Vec2::splat(self.padding) * 2.0;

        let (rect, response) = ui.allocate_exact_size(desired_size, Sense::hover());

        if ui.is_rect_visible(rect) {
            ui.painter().rect(
                rect,
                ui.style().visuals.noninteractive().rounding,
                Color32::WHITE,
                Stroke::none(),
            );

            for (i, b) in barcode.into_iter().enumerate() {
                match b {
                    0 => {}
                    1 => ui.painter().rect(
                        Rect::from_min_size(
                            rect.left_top()
                                + vec2(self.padding + self.bar_width * i as f32, self.padding),
                            vec2(self.bar_width, self.bar_height),
                        ),
                        0.0,
                        Color32::BLACK,
                        Stroke::none(),
                    ),
                    _ => unreachable!(),
                }
            }
        }

        response
    }
}
