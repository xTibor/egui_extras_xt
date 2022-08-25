use std::sync::Arc;

use egui::util::cache::{ComputerMut, FrameCache};
use egui::{vec2, Align2, Color32, FontFamily, FontId, Rect, Response, Sense, Stroke, Ui, Widget};

use barcoders::sym::codabar::Codabar;
use barcoders::sym::code11::Code11;
use barcoders::sym::code128::Code128;
use barcoders::sym::code39::Code39;
use barcoders::sym::code93::Code93;
use barcoders::sym::ean13::EAN13;
use barcoders::sym::ean8::EAN8;
use barcoders::sym::ean_supp::EANSUPP;
use barcoders::sym::tf::TF;

// ----------------------------------------------------------------------------

#[non_exhaustive]
#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub enum BarcodeKind {
    Codabar,
    Code11,
    Code39,
    Code39Checksum,
    Code93,
    Code128,
    EAN8,
    EAN13,
    EANSUPP,
    ITF,
    STF,
}

impl BarcodeKind {
    fn encode<T: AsRef<str>>(&self, data: T) -> Option<Vec<u8>> {
        match *self {
            BarcodeKind::Codabar => Some(Codabar::new(data).ok()?.encode()),
            BarcodeKind::Code11 => Some(Code11::new(data).ok()?.encode()),
            BarcodeKind::Code39 => Some(Code39::new(data).ok()?.encode()),
            BarcodeKind::Code39Checksum => Some(Code39::with_checksum(data).ok()?.encode()),
            BarcodeKind::Code93 => Some(Code93::new(data).ok()?.encode()),
            BarcodeKind::Code128 => Some(Code128::new(data).ok()?.encode()),
            BarcodeKind::EAN8 => Some(EAN8::new(data).ok()?.encode()),
            BarcodeKind::EAN13 => Some(EAN13::new(data).ok()?.encode()),
            BarcodeKind::EANSUPP => Some(EANSUPP::new(data).ok()?.encode()),
            BarcodeKind::ITF => Some(TF::interleaved(data).ok()?.encode()),
            BarcodeKind::STF => Some(TF::standard(data).ok()?.encode()),
        }
    }
}

// ----------------------------------------------------------------------------

type BarcodeCacheKey<'a> = (BarcodeKind, &'a str);
type BarcodeCacheValue = Arc<Vec<u8>>;

#[derive(Default)]
struct BarcodeComputer;

impl<'a> ComputerMut<BarcodeCacheKey<'a>, BarcodeCacheValue> for BarcodeComputer {
    fn compute(&mut self, key: BarcodeCacheKey) -> BarcodeCacheValue {
        let (barcode_kind, value) = key;
        Arc::new(barcode_kind.encode(value).unwrap_or_default())
    }
}

type BarcodeCache<'a> = FrameCache<BarcodeCacheValue, BarcodeComputer>;

// ----------------------------------------------------------------------------

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
    foreground_color: Color32,
    background_color: Color32,
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
            foreground_color: Color32::BLACK,
            background_color: Color32::WHITE,
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

    pub fn foreground_color(mut self, foreground_color: impl Into<Color32>) -> Self {
        self.foreground_color = foreground_color.into();
        self
    }

    pub fn background_color(mut self, background_color: impl Into<Color32>) -> Self {
        self.background_color = background_color.into();
        self
    }
}

impl<'a> Widget for BarcodeWidget<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        let barcode = {
            let mut memory = ui.memory();
            let cache = memory.caches.cache::<BarcodeCache<'_>>();
            cache.get((self.barcode_kind, self.value))
        };

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
                self.background_color,
                Stroke::none(),
            );

            barcode
                .iter()
                .enumerate()
                .filter(|&(_bar_index, bar_value)| *bar_value == 1)
                .map(|(bar_index, _bar_value)| {
                    Rect::from_min_size(
                        ui.painter().round_pos_to_pixels(
                            rect.left_top() + vec2(self.horizontal_padding, self.vertical_padding),
                        ) + vec2(bar_width * bar_index as f32, 0.0),
                        vec2(bar_width, self.bar_height),
                    )
                })
                .for_each(|bar_rect| {
                    ui.painter()
                        .rect(bar_rect, 0.0, self.foreground_color, Stroke::none());
                });

            if let Some(label) = self.label {
                ui.painter().text(
                    rect.center_bottom() - vec2(0.0, self.vertical_padding),
                    Align2::CENTER_BOTTOM,
                    label,
                    FontId::new(self.label_height, FontFamily::Proportional),
                    self.foreground_color,
                );
            }
        }

        response
    }
}
