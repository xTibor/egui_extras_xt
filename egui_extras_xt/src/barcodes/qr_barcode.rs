use egui::{vec2, Color32, Rect, Response, Sense, Stroke, Ui, Vec2, Widget};

use qrcode::{Color, QrCode};

// ----------------------------------------------------------------------------

#[must_use = "You should put this widget in an ui with `ui.add(widget);`"]
pub struct QrBarcodeWidget<'a> {
    value: &'a str,
    module_size: usize,
}

impl<'a> QrBarcodeWidget<'a> {
    pub fn new(value: &'a str) -> Self {
        Self {
            value,
            module_size: 6,
        }
    }

    pub fn module_size(mut self, module_size: impl Into<usize>) -> Self {
        self.module_size = module_size.into();
        self
    }
}

impl<'a> Widget for QrBarcodeWidget<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        let qr_code = QrCode::new(self.value).unwrap(); // TODO: Cache

        let module_size = self.module_size as f32 / ui.ctx().pixels_per_point();
        let quiet_zone_modules = 4;

        let desired_size =
            Vec2::splat((qr_code.width() + quiet_zone_modules * 2) as f32 * module_size);

        let (rect, response) = ui.allocate_exact_size(desired_size, Sense::hover());

        if ui.is_rect_visible(rect) {
            ui.painter().rect(
                rect,
                ui.style().visuals.noninteractive().rounding,
                Color32::WHITE,
                Stroke::none(),
            );

            qr_code
                .to_colors()
                .into_iter()
                .enumerate()
                .filter(|&(_index, value)| value == Color::Dark)
                .map(|(index, _value)| (index % qr_code.width(), index / qr_code.width()))
                .for_each(|(x, y)| {
                    ui.painter().rect(
                        Rect::from_min_size(
                            ui.painter().round_pos_to_pixels(
                                rect.left_top()
                                    + Vec2::splat(quiet_zone_modules as f32 * module_size),
                            ) + vec2(x as f32, y as f32) * module_size,
                            Vec2::splat(module_size),
                        ),
                        0.0,
                        Color32::BLACK,
                        Stroke::none(),
                    )
                });
        }

        response
    }
}
