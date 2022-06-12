mod display_metrics;
mod display_style;
mod seven_segment;
mod sixteen_segment;
mod widget;

pub use display_metrics::{DisplayMetrics, DisplayMetricsPreset};
pub use display_style::{DisplayStyle, DisplayStylePreset};
pub use seven_segment::SevenSegment;
pub use sixteen_segment::SixteenSegment;
pub use widget::SegmentedDisplayWidget;

use egui::Pos2;

// ----------------------------------------------------------------------------

pub type DisplayFontGlyph = u16;

#[derive(Copy, Clone, Default)]
pub struct DisplayDigit {
    pub glyph: DisplayFontGlyph,
    pub dot: bool,
    pub colon: bool,
    pub apostrophe: bool,
}

pub type SegmentGeometryTransformFn = dyn Fn(f32, f32) -> Pos2;

pub trait DisplayKind {
    fn glyph(&self, c: char) -> Option<DisplayFontGlyph>;

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
