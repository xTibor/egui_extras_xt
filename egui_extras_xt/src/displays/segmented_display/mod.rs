mod display_metrics;
mod widget;

mod nine_segment;
mod seven_segment;
mod sixteen_segment;

use strum::{Display, EnumIter};

pub use display_metrics::{DisplayMetrics, DisplayMetricsPreset};
pub use widget::SegmentedDisplayWidget;

use egui::Pos2;

// ----------------------------------------------------------------------------

pub type DisplayGlyph = u16;

#[derive(Clone, Copy, Debug, Default)]
pub struct DisplayDigit {
    pub glyph: DisplayGlyph,
    pub dot: bool,
    pub colon: bool,
    pub apostrophe: bool,
}

// ----------------------------------------------------------------------------

#[non_exhaustive]
#[derive(Clone, Copy, Debug, Display, EnumIter, Eq, PartialEq)]
pub enum DisplayKind {
    #[strum(to_string = "7-segment")]
    SevenSegment,

    #[strum(to_string = "9-segment")]
    NineSegment,

    #[strum(to_string = "16-segment")]
    SixteenSegment,
}

impl DisplayKind {
    pub(crate) fn display_impl(&self) -> Box<dyn DisplayImpl> {
        match *self {
            DisplayKind::SevenSegment => Box::new(seven_segment::SevenSegment),
            DisplayKind::NineSegment => Box::new(nine_segment::NineSegment),
            DisplayKind::SixteenSegment => Box::new(sixteen_segment::SixteenSegment),
        }
    }

    pub fn segment_count(&self) -> usize {
        self.display_impl().segment_count()
    }
}

// ----------------------------------------------------------------------------

pub(crate) type SegmentGeometryTransformFn = dyn Fn(f32, f32) -> Pos2;

pub(crate) trait DisplayImpl {
    fn segment_count(&self) -> usize;

    fn glyph(&self, c: char) -> Option<DisplayGlyph>;

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
