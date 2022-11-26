use std::borrow::Borrow;
use std::sync::Arc;

use egui::util::cache::{ComputerMut, FrameCache};
use egui::{vec2, Color32, Rect, Response, Sense, Stroke, Ui, Vec2, Widget};

use datamatrix::data::DataEncodingError;
use datamatrix::placement::Bitmap;
use datamatrix::{DataMatrix, SymbolList};

// ----------------------------------------------------------------------------

type DataMatrixCacheKey<'a> = &'a str;
type DataMatrixCacheValue = Arc<Result<Bitmap<bool>, DataEncodingError>>;

#[derive(Default)]
struct DataMatrixComputer;

impl<'a> ComputerMut<DataMatrixCacheKey<'a>, DataMatrixCacheValue> for DataMatrixComputer {
    fn compute(&mut self, key: DataMatrixCacheKey) -> DataMatrixCacheValue {
        Arc::new(
            DataMatrix::encode_str(key, SymbolList::default())
                .map(|datamatrix| datamatrix.bitmap()),
        )
    }
}

type DataMatrixCache<'a> = FrameCache<DataMatrixCacheValue, DataMatrixComputer>;

// ----------------------------------------------------------------------------

#[must_use = "You should put this widget in an ui with `ui.add(widget);`"]
pub struct DataMatrixWidget<'a> {
    value: &'a str,
    module_size: usize,
    quiet_zone: usize,
    foreground_color: Color32,
    background_color: Color32,
}

impl<'a> DataMatrixWidget<'a> {
    pub fn new(value: &'a str) -> Self {
        Self {
            value,
            module_size: 6,
            quiet_zone: 1,
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

impl<'a> Widget for DataMatrixWidget<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        let cached_bitmap = {
            let mut memory = ui.memory();
            let cache = memory.caches.cache::<DataMatrixCache<'_>>();
            cache.get(self.value)
        };

        if let Ok(bitmap) = cached_bitmap.borrow() {
            let module_size = self.module_size as f32 / ui.ctx().pixels_per_point();

            let desired_size = vec2(
                (bitmap.width() + self.quiet_zone * 2) as f32,
                (bitmap.height() + self.quiet_zone * 2) as f32,
            ) * module_size;

            let (rect, response) = ui.allocate_exact_size(desired_size, Sense::hover());

            if ui.is_rect_visible(rect) {
                ui.painter().rect(
                    rect,
                    ui.style().visuals.noninteractive().rounding,
                    self.background_color,
                    Stroke::none(),
                );

                bitmap
                    .pixels()
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
                            .rect(module_rect, 0.0, self.foreground_color, Stroke::none());
                    });
            }

            response
        } else {
            ui.colored_label(
                ui.style().visuals.error_fg_color,
                "\u{1F525} Failed to render data matrix code",
            )
        }
    }
}
