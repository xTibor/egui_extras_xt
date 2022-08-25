use std::sync::Arc;

use egui::util::cache::{ComputerMut, FrameCache};
use egui::{vec2, Color32, Rect, Response, Sense, Stroke, Ui, Vec2, Widget};

use qrcode::{Color, QrCode};

// ----------------------------------------------------------------------------

type QrCodeCacheKey<'a> = &'a str;
type QrCodeCacheValue = Arc<QrCode>;

#[derive(Default)]
struct QrCodeComputer;

impl<'a> ComputerMut<QrCodeCacheKey<'a>, QrCodeCacheValue> for QrCodeComputer {
    fn compute(&mut self, key: QrCodeCacheKey) -> QrCodeCacheValue {
        Arc::new(QrCode::new(key).unwrap())
    }
}

type QrCodeCache<'a> = FrameCache<QrCodeCacheValue, QrCodeComputer>;

// ----------------------------------------------------------------------------

#[must_use = "You should put this widget in an ui with `ui.add(widget);`"]
pub struct QrCodeWidget<'a> {
    value: &'a str,
    module_size: usize,
    quiet_zone: usize,
    foreground_color: Color32,
    background_color: Color32,
}

impl<'a> QrCodeWidget<'a> {
    pub fn new(value: &'a str) -> Self {
        Self {
            value,
            module_size: 6,
            quiet_zone: 4,
            foreground_color: Color32::BLACK,
            background_color: Color32::WHITE,
        }
    }

    pub fn module_size(mut self, module_size: impl Into<usize>) -> Self {
        self.module_size = module_size.into();
        self
    }

    pub fn quiet_zone(mut self, quiet_zone: impl Into<usize>) -> Self {
        self.quiet_zone = quiet_zone.into();
        self
    }

    pub fn foreground_color(mut self, foreground_color: impl Into<Color32>) -> Self {
        self.foreground_color = foreground_color.into();
        self
    }

    pub fn background_color(mut self, background_color: impl Into<Color32>) -> Self {
        self.background_color = background_color.into();
        self
    }
}

impl<'a> Widget for QrCodeWidget<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        let qr_code = {
            let mut memory = ui.memory();
            let cache = memory.caches.cache::<QrCodeCache<'_>>();
            cache.get(self.value)
        };

        let module_size = self.module_size as f32 / ui.ctx().pixels_per_point();

        let desired_size =
            Vec2::splat((qr_code.width() + self.quiet_zone * 2) as f32 * module_size);

        let (rect, response) = ui.allocate_exact_size(desired_size, Sense::hover());

        if ui.is_rect_visible(rect) {
            ui.painter().rect(
                rect,
                ui.style().visuals.noninteractive().rounding,
                self.background_color,
                Stroke::none(),
            );

            qr_code
                .to_colors()
                .into_iter()
                .enumerate()
                .filter(|&(_module_index, module_value)| module_value == Color::Dark)
                .map(|(module_index, _module_value)| {
                    (
                        module_index % qr_code.width(),
                        module_index / qr_code.width(),
                    )
                })
                .map(|(x, y)| {
                    Rect::from_min_size(
                        ui.painter().round_pos_to_pixels(
                            rect.left_top() + Vec2::splat(self.quiet_zone as f32 * module_size),
                        ) + vec2(x as f32, y as f32) * module_size,
                        Vec2::splat(module_size),
                    )
                })
                .for_each(|module_rect| {
                    ui.painter()
                        .rect(module_rect, 0.0, self.foreground_color, Stroke::none())
                });
        }

        response
    }
}
