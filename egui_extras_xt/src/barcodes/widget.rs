use egui::{pos2, vec2, Color32, Rect, Response, Sense, Stroke, Ui, Vec2, Widget};

#[must_use = "You should put this widget in an ui with `ui.add(widget);`"]
pub struct BarcodeWidget {
    value: String,
    size: Vec2,
    padding: f32,
}

impl BarcodeWidget {
    pub fn new(value: &str) -> Self {
        Self {
            value: value.to_owned(),
            size: vec2(256.0, 64.0),
            padding: 16.0,
        }
    }
}

impl Widget for BarcodeWidget {
    fn ui(self, ui: &mut Ui) -> Response {
        let barcode = barcoders::sym::code39::Code39::new(&self.value)
            .unwrap()
            .encode();

        let desired_size = self.size + Vec2::splat(self.padding) * 2.0;

        let (rect, response) = ui.allocate_exact_size(desired_size, Sense::hover());

        if ui.is_rect_visible(rect) {
            ui.painter().rect(
                rect,
                ui.style().visuals.noninteractive().rounding,
                Color32::WHITE,
                Stroke::none(),
            );

            let bar_width = self.size.x / barcode.len() as f32;
            let bar_height = self.size.y;

            for (i, b) in barcode.into_iter().enumerate() {
                match b {
                    0 => {}
                    1 => ui.painter().rect(
                        Rect::from_min_size(
                            rect.left_top()
                                + vec2(self.padding + bar_width * i as f32, self.padding),
                            vec2(bar_width, bar_height),
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
