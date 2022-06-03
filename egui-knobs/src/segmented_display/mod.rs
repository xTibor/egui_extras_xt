mod seven_segment;
mod sixteen_segment;
mod widget;

pub use seven_segment::SevenSegment;
pub use sixteen_segment::SixteenSegment;
pub use widget::SegmentedDisplayWidget;

use egui::{Color32, Pos2, Stroke, Ui};

// ----------------------------------------------------------------------------

#[derive(Copy, Clone)]
pub struct SegmentedDisplayMetrics {
    pub segment_spacing: f32,
    pub segment_thickness: f32,

    pub digit_median: f32,
    pub digit_ratio: f32,
    pub digit_shearing: f32,
    pub digit_spacing: f32,

    pub margin_horizontal: f32,
    pub margin_vertical: f32,

    pub colon_separation: f32,
}

impl Default for SegmentedDisplayMetrics {
    fn default() -> Self {
        SegmentedDisplayMetricsPreset::Default.metrics()
    }
}

// ----------------------------------------------------------------------------

#[non_exhaustive]
#[derive(Copy, Clone)]
pub enum SegmentedDisplayMetricsPreset {
    Default,
    KnightRider,
}

impl SegmentedDisplayMetricsPreset {
    pub fn metrics(&self) -> SegmentedDisplayMetrics {
        match *self {
            SegmentedDisplayMetricsPreset::Default => SegmentedDisplayMetrics {
                segment_spacing: 0.01,
                segment_thickness: 0.1,
                digit_median: -0.05,
                digit_ratio: 0.6,
                digit_shearing: 0.1,
                digit_spacing: 0.35,
                margin_horizontal: 0.3,
                margin_vertical: 0.1,
                colon_separation: 0.25,
            },
            SegmentedDisplayMetricsPreset::KnightRider => SegmentedDisplayMetrics {
                segment_spacing: 0.02,
                segment_thickness: 0.12,
                digit_median: -0.05,
                digit_ratio: 1.0,
                digit_shearing: 0.1,
                digit_spacing: 0.20,
                margin_horizontal: 0.3,
                margin_vertical: 0.1,
                colon_separation: 0.25,
            },
        }
    }
}

// ----------------------------------------------------------------------------

#[derive(Copy, Clone)]
pub struct SegmentedDisplayStyle {
    pub background_color: Color32,
    pub segment_on_color: Color32,
    pub segment_off_color: Color32,
    pub segment_on_stroke: Stroke,
    pub segment_off_stroke: Stroke,
}

impl SegmentedDisplayStyle {
    fn segment_color(&self, active: bool) -> Color32 {
        if active {
            self.segment_on_color
        } else {
            self.segment_off_color
        }
    }

    fn segment_stroke(&self, active: bool) -> Stroke {
        if active {
            self.segment_on_stroke
        } else {
            self.segment_off_stroke
        }
    }

    pub fn system_style(ui: &Ui) -> Self {
        SegmentedDisplayStyle {
            background_color: Color32::TRANSPARENT,
            segment_on_color: ui.style().visuals.text_color(),
            segment_off_color: ui.style().visuals.faint_bg_color,
            segment_on_stroke: Stroke::none(),
            segment_off_stroke: Stroke::none(),
        }
    }
}

impl Default for SegmentedDisplayStyle {
    fn default() -> Self {
        SegmentedDisplayStylePreset::Default.style()
    }
}

// ----------------------------------------------------------------------------

#[non_exhaustive]
#[derive(Copy, Clone)]
pub enum SegmentedDisplayStylePreset {
    Default,
    Calculator,
    NintendoGameBoy,
    KnightRider,
    BlueNegative,
    Amber,
    DeLoreanRed,
    DeLoreanGreen,
    DeLoreanAmber,
}

impl SegmentedDisplayStylePreset {
    pub fn style(&self) -> SegmentedDisplayStyle {
        match *self {
            SegmentedDisplayStylePreset::Default => SegmentedDisplayStyle {
                background_color: Color32::from_rgb(0x00, 0x20, 0x00),
                segment_on_color: Color32::from_rgb(0x00, 0xF0, 0x00),
                segment_off_color: Color32::from_rgb(0x00, 0x30, 0x00),
                segment_on_stroke: Stroke::none(),
                segment_off_stroke: Stroke::none(),
            },
            SegmentedDisplayStylePreset::Calculator => SegmentedDisplayStyle {
                background_color: Color32::from_rgb(0xC5, 0xCB, 0xB6),
                segment_on_color: Color32::from_rgb(0x00, 0x00, 0x00),
                segment_off_color: Color32::from_rgb(0xB9, 0xBE, 0xAB),
                segment_on_stroke: Stroke::none(),
                segment_off_stroke: Stroke::none(),
            },
            SegmentedDisplayStylePreset::NintendoGameBoy => SegmentedDisplayStyle {
                background_color: Color32::from_rgb(0x9B, 0xBC, 0x0F),
                segment_on_color: Color32::from_rgb(0x0F, 0x38, 0x0F),
                segment_off_color: Color32::from_rgb(0x8B, 0xAC, 0x0F),
                segment_on_stroke: Stroke::none(),
                segment_off_stroke: Stroke::none(),
            },
            SegmentedDisplayStylePreset::KnightRider => SegmentedDisplayStyle {
                background_color: Color32::from_rgb(0x10, 0x00, 0x00),
                segment_on_color: Color32::from_rgb(0xC8, 0x00, 0x00),
                segment_off_color: Color32::from_rgb(0x20, 0x00, 0x00),
                segment_on_stroke: Stroke::none(),
                segment_off_stroke: Stroke::none(),
            },
            SegmentedDisplayStylePreset::BlueNegative => SegmentedDisplayStyle {
                background_color: Color32::from_rgb(0x00, 0x00, 0xFF),
                segment_on_color: Color32::from_rgb(0xE0, 0xFF, 0xFF),
                segment_off_color: Color32::from_rgb(0x28, 0x28, 0xFF),
                segment_on_stroke: Stroke::none(),
                segment_off_stroke: Stroke::none(),
            },
            SegmentedDisplayStylePreset::Amber => SegmentedDisplayStyle {
                background_color: Color32::from_rgb(0x1D, 0x12, 0x07),
                segment_on_color: Color32::from_rgb(0xFF, 0x9A, 0x21),
                segment_off_color: Color32::from_rgb(0x33, 0x20, 0x00),
                segment_on_stroke: Stroke::none(),
                segment_off_stroke: Stroke::none(),
            },
            SegmentedDisplayStylePreset::DeLoreanRed => todo!(),
            SegmentedDisplayStylePreset::DeLoreanGreen => todo!(),
            SegmentedDisplayStylePreset::DeLoreanAmber => todo!(),
        }
    }
}

// ----------------------------------------------------------------------------

pub type SegmentedDisplayFont = [u16; 128];

// ----------------------------------------------------------------------------

pub type SegmentGeometryTransformFn = dyn Fn(f32, f32) -> Pos2;

// ----------------------------------------------------------------------------

pub trait SegmentedDisplayKind {
    fn default_font<'a>(&self) -> &'a SegmentedDisplayFont;

    fn geometry(
        &self,
        tr: &SegmentGeometryTransformFn,
        digit_width: f32,
        digit_height: f32,
        segment_thickness: f32,
        segment_spacing: f32,
        digit_median: f32,
    ) -> Vec<Vec<Pos2>>;
}

// ----------------------------------------------------------------------------

#[derive(Copy, Clone, Default)]
pub struct SegmentedDisplayDigit {
    pub segments: u16,
    pub dot: bool,
    pub colon: bool,
    pub apostrophe: bool,
}

// ----------------------------------------------------------------------------
