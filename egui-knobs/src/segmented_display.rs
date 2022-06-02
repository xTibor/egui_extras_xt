use egui::{vec2, Color32, Pos2, Response, Sense, Shape, Stroke, Ui, Widget};

use itertools::Itertools;

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

#[non_exhaustive]
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum SegmentedDisplayKind {
    SevenSegment,
    SixteenSegment,
}

impl SegmentedDisplayKind {
    pub fn default_font<'a>(&self) -> &'a SegmentedDisplayFont {
        match *self {
            #[rustfmt::skip]
            SegmentedDisplayKind::SevenSegment => &[
                0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, // 00-07: × × × × × × × ×
                0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, // 08-0F: × × × × × × × ×
                0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, // 10-17: × × × × × × × ×
                0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, // 18-1F: × × × × × × × ×
                0x0000, 0x0000, 0x0022, 0x0000, 0x0000, 0x0000, 0x0000, 0x0002, // 20-27:   × " × × × × '
                0x0039, 0x000F, 0x0000, 0x0000, 0x000C, 0x0040, 0x0004, 0x0052, // 28-2F: ( ) × × , - . /
                0x003F, 0x0006, 0x005B, 0x004F, 0x0066, 0x006D, 0x007D, 0x0027, // 30-37: 0 1 2 3 4 5 6 7
                0x007F, 0x006F, 0x0000, 0x0000, 0x0039, 0x0048, 0x000F, 0x0053, // 38-3F: 8 9 × × < = > ?
                0x007B, 0x0077, 0x007C, 0x0039, 0x005E, 0x0079, 0x0071, 0x003D, // 40-47: @ A B C D E F G
                0x0076, 0x0030, 0x001E, 0x0076, 0x0038, 0x002B, 0x0037, 0x003F, // 48-4F: H I J K L M N O
                0x0073, 0x0067, 0x0077, 0x006D, 0x0007, 0x003E, 0x003E, 0x007E, // 50-57: P Q R S T U V W
                0x0076, 0x006E, 0x005B, 0x0039, 0x0064, 0x000F, 0x0023, 0x0008, // 58-5F: X Y Z [ \ ] ^ _
                0x0020, 0x005F, 0x007C, 0x0058, 0x005E, 0x007B, 0x0071, 0x006F, // 60-67: ` a b c d e f g
                0x0074, 0x0010, 0x000E, 0x0076, 0x0006, 0x0055, 0x0054, 0x005C, // 68-6F: h i j k l m n o
                0x0073, 0x0067, 0x0050, 0x006D, 0x0078, 0x001C, 0x001C, 0x007E, // 70-77: p q r s t u v w
                0x0076, 0x006E, 0x005B, 0x0046, 0x0030, 0x0070, 0x0040, 0x0000, // 78-7F: x y z { | } ~ ×
            ],
            #[rustfmt::skip]
            SegmentedDisplayKind::SixteenSegment => &[
                0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, // 00-07: × × × × × × × ×
                0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, // 08-0F: × × × × × × × ×
                0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, // 10-17: × × × × × × × ×
                0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, // 18-1F: × × × × × × × ×
                0x0000, 0x2200, 0x0280, 0xAA3C, 0xAABB, 0xEE99, 0x9371, 0x0080, // 20-27:   ! " # $ % & '
                0x1400, 0x4100, 0xDD00, 0xAA00, 0xC000, 0x8800, 0x0020, 0x4400, // 28-2F: ( ) * + , - . /
                0x44FF, 0x040C, 0x8877, 0x883F, 0x888C, 0x88BB, 0x88FB, 0x2483, // 30-37: 0 1 2 3 4 5 6 7
                0x88FF, 0x88BF, 0x8020, 0xC001, 0x9400, 0x8830, 0x4900, 0x2887, // 38-3F: 8 9 : ; < = > ?
                0x28DF, 0x88CF, 0x2A3F, 0x00F3, 0x223F, 0x80F3, 0x80C3, 0x08FB, // 40-47: @ A B C D E F G
                0x88CC, 0x2233, 0x007E, 0x94C0, 0x00F0, 0x05CC, 0x11CC, 0x00FF, // 48-4F: H I J K L M N O
                0x88C7, 0x10FF, 0x98C7, 0x093B, 0x2203, 0x00FC, 0x44C0, 0x50CC, // 50-57: P Q R S T U V W
                0x5500, 0x2500, 0x4433, 0x2212, 0x1100, 0x2221, 0x0404, 0x0030, // 58-5F: X Y Z [ \ ] ^ _
                0x0100, 0xA070, 0xA0E0, 0x8060, 0xA260, 0xC060, 0xAA02, 0x1818, // 60-67: ` a b c d e f g
                0xA0C0, 0x0040, 0x2220, 0x3A00, 0x00E0, 0xA848, 0xA040, 0xA060, // 68-6F: h i j k l m n o
                0x82C1, 0xA281, 0x8040, 0x1810, 0xAA10, 0x2060, 0x4040, 0x5048, // 70-77: p q r s t u v w
                0xD800, 0x1018, 0xC020, 0xA212, 0x2200, 0x2A21, 0x0A85, 0x0000, // 78-7F: x y z { | } ~ ×
            ],
        }
    }

    fn geometry(
        &self,
        tr: &SegmentGeometryTransformFn,
        digit_width: f32,
        digit_height: f32,
        segment_thickness: f32,
        segment_spacing: f32,
        digit_median: f32,
    ) -> Vec<Vec<Pos2>> {
        match *self {
            #[rustfmt::skip]
            #[allow(unused_parens)]
            SegmentedDisplayKind::SevenSegment => vec![
                vec![
                    tr(-(digit_width / 2.0) + (segment_thickness / 4.0) + segment_spacing, -(digit_height / 2.0) + (segment_thickness / 4.0)                                 ),
                    tr(-(digit_width / 2.0) + (segment_thickness / 2.0) + segment_spacing, -(digit_height / 2.0)                                                             ),
                    tr( (digit_width / 2.0) - (segment_thickness / 2.0) - segment_spacing, -(digit_height / 2.0)                                                             ),
                    tr( (digit_width / 2.0) - (segment_thickness / 4.0) - segment_spacing, -(digit_height / 2.0) + (segment_thickness / 4.0)                                 ),
                    tr( (digit_width / 2.0) - (segment_thickness / 1.0) - segment_spacing, -(digit_height / 2.0) + (segment_thickness / 1.0)                                 ),
                    tr(-(digit_width / 2.0) + (segment_thickness / 1.0) + segment_spacing, -(digit_height / 2.0) + (segment_thickness / 1.0)                                 ),
                ],
                vec![
                    tr( (digit_width / 2.0) - (segment_thickness / 1.0)                  , -(digit_height / 2.0) + (segment_thickness / 1.0) + segment_spacing               ),
                    tr( (digit_width / 2.0) - (segment_thickness / 4.0)                  , -(digit_height / 2.0) + (segment_thickness / 4.0) + segment_spacing               ),
                    tr( (digit_width / 2.0)                                              , -(digit_height / 2.0) + (segment_thickness / 2.0) + segment_spacing               ),
                    tr( (digit_width / 2.0)                                              ,                       - (segment_thickness / 2.0) - segment_spacing + digit_median),
                    tr( (digit_width / 2.0) - (segment_thickness / 2.0)                  ,                                                   - segment_spacing + digit_median),
                    tr( (digit_width / 2.0) - (segment_thickness / 1.0)                  ,                       - (segment_thickness / 2.0) - segment_spacing + digit_median),
                ],
                vec![
                    tr( (digit_width / 2.0) - (segment_thickness / 1.0)                  ,  (digit_height / 2.0) - (segment_thickness / 1.0) - segment_spacing               ),
                    tr( (digit_width / 2.0) - (segment_thickness / 4.0)                  ,  (digit_height / 2.0) - (segment_thickness / 4.0) - segment_spacing               ),
                    tr( (digit_width / 2.0)                                              ,  (digit_height / 2.0) - (segment_thickness / 2.0) - segment_spacing               ),
                    tr( (digit_width / 2.0)                                              ,                         (segment_thickness / 2.0) + segment_spacing + digit_median),
                    tr( (digit_width / 2.0) - (segment_thickness / 2.0)                  ,                                                     segment_spacing + digit_median),
                    tr( (digit_width / 2.0) - (segment_thickness / 1.0)                  ,                         (segment_thickness / 2.0) + segment_spacing + digit_median),
                ],
                vec![
                    tr(-(digit_width / 2.0) + (segment_thickness / 4.0) + segment_spacing,  (digit_height / 2.0) - (segment_thickness / 4.0)                                 ),
                    tr(-(digit_width / 2.0) + (segment_thickness / 2.0) + segment_spacing,  (digit_height / 2.0)                                                             ),
                    tr( (digit_width / 2.0) - (segment_thickness / 2.0) - segment_spacing,  (digit_height / 2.0)                                                             ),
                    tr( (digit_width / 2.0) - (segment_thickness / 4.0) - segment_spacing,  (digit_height / 2.0) - (segment_thickness / 4.0)                                 ),
                    tr( (digit_width / 2.0) - (segment_thickness / 1.0) - segment_spacing,  (digit_height / 2.0) - (segment_thickness / 1.0)                                 ),
                    tr(-(digit_width / 2.0) + (segment_thickness / 1.0) + segment_spacing,  (digit_height / 2.0) - (segment_thickness / 1.0)                                 ),
                ],
                vec![
                    tr(-(digit_width / 2.0) + (segment_thickness / 1.0)                  ,  (digit_height / 2.0) - (segment_thickness / 1.0) - segment_spacing               ),
                    tr(-(digit_width / 2.0) + (segment_thickness / 4.0)                  ,  (digit_height / 2.0) - (segment_thickness / 4.0) - segment_spacing               ),
                    tr(-(digit_width / 2.0)                                              ,  (digit_height / 2.0) - (segment_thickness / 2.0) - segment_spacing               ),
                    tr(-(digit_width / 2.0)                                              ,                         (segment_thickness / 2.0) + segment_spacing + digit_median),
                    tr(-(digit_width / 2.0) + (segment_thickness / 2.0)                  ,                                                     segment_spacing + digit_median),
                    tr(-(digit_width / 2.0) + (segment_thickness / 1.0)                  ,                         (segment_thickness / 2.0) + segment_spacing + digit_median),
                ],
                vec![
                    tr(-(digit_width / 2.0) + (segment_thickness / 1.0)                  , -(digit_height / 2.0) + (segment_thickness / 1.0) + segment_spacing               ),
                    tr(-(digit_width / 2.0) + (segment_thickness / 4.0)                  , -(digit_height / 2.0) + (segment_thickness / 4.0) + segment_spacing               ),
                    tr(-(digit_width / 2.0)                                              , -(digit_height / 2.0) + (segment_thickness / 2.0) + segment_spacing               ),
                    tr(-(digit_width / 2.0)                                              ,                       - (segment_thickness / 2.0) - segment_spacing + digit_median),
                    tr(-(digit_width / 2.0) + (segment_thickness / 2.0)                  ,                                                   - segment_spacing + digit_median),
                    tr(-(digit_width / 2.0) + (segment_thickness / 1.0)                  ,                       - (segment_thickness / 2.0) - segment_spacing + digit_median),
                ],
                vec![
                    tr(-(digit_width / 2.0) + (segment_thickness / 2.0) + segment_spacing,                                                                       digit_median),
                    tr(-(digit_width / 2.0) + (segment_thickness / 1.0) + segment_spacing,                       - (segment_thickness / 2.0)                   + digit_median),
                    tr( (digit_width / 2.0) - (segment_thickness / 1.0) - segment_spacing,                       - (segment_thickness / 2.0)                   + digit_median),
                    tr( (digit_width / 2.0) - (segment_thickness / 2.0) - segment_spacing,                                                                       digit_median),
                    tr( (digit_width / 2.0) - (segment_thickness / 1.0) - segment_spacing,                         (segment_thickness / 2.0)                   + digit_median),
                    tr(-(digit_width / 2.0) + (segment_thickness / 1.0) + segment_spacing,                         (segment_thickness / 2.0)                   + digit_median),
                ],
            ],
            #[rustfmt::skip]
            #[allow(unused_parens)]
            SegmentedDisplayKind::SixteenSegment => vec![
                vec![
                    tr(-(digit_width / 2.0) + (segment_thickness / 4.0) + segment_spacing, -(digit_height / 2.0) + (segment_thickness / 4.0)                                 ),
                    tr(-(digit_width / 2.0) + (segment_thickness / 2.0) + segment_spacing, -(digit_height / 2.0)                                                             ),
                    tr(                     - (segment_thickness / 2.0) - segment_spacing, -(digit_height / 2.0)                                                             ),
                    tr(                                                 - segment_spacing, -(digit_height / 2.0) + (segment_thickness / 2.0)                                 ),
                    tr(                     - (segment_thickness / 2.0) - segment_spacing, -(digit_height / 2.0) + (segment_thickness / 1.0)                                 ),
                    tr(-(digit_width / 2.0) + (segment_thickness / 1.0) + segment_spacing, -(digit_height / 2.0) + (segment_thickness / 1.0)                                 ),
                ],
                vec![
                    tr( (digit_width / 2.0) - (segment_thickness / 4.0) - segment_spacing, -(digit_height / 2.0) + (segment_thickness / 4.0)                                 ),
                    tr( (digit_width / 2.0) - (segment_thickness / 2.0) - segment_spacing, -(digit_height / 2.0)                                                             ),
                    tr(                       (segment_thickness / 2.0) + segment_spacing, -(digit_height / 2.0)                                                             ),
                    tr(                                                   segment_spacing, -(digit_height / 2.0) + (segment_thickness / 2.0)                                 ),
                    tr(                       (segment_thickness / 2.0) + segment_spacing, -(digit_height / 2.0) + (segment_thickness / 1.0)                                 ),
                    tr( (digit_width / 2.0) - (segment_thickness / 1.0) - segment_spacing, -(digit_height / 2.0) + (segment_thickness / 1.0)                                 ),
                ],
                vec![
                    tr( (digit_width / 2.0) - (segment_thickness / 1.0)                  , -(digit_height / 2.0) + (segment_thickness / 1.0) + segment_spacing               ),
                    tr( (digit_width / 2.0) - (segment_thickness / 4.0)                  , -(digit_height / 2.0) + (segment_thickness / 4.0) + segment_spacing               ),
                    tr( (digit_width / 2.0)                                              , -(digit_height / 2.0) + (segment_thickness / 2.0) + segment_spacing               ),
                    tr( (digit_width / 2.0)                                              ,                       - (segment_thickness / 2.0) - segment_spacing + digit_median),
                    tr( (digit_width / 2.0) - (segment_thickness / 2.0)                  ,                                                   - segment_spacing + digit_median),
                    tr( (digit_width / 2.0) - (segment_thickness / 1.0)                  ,                       - (segment_thickness / 2.0) - segment_spacing + digit_median),
                ],
                vec![
                    tr( (digit_width / 2.0) - (segment_thickness / 1.0)                  ,  (digit_height / 2.0) - (segment_thickness / 1.0) - segment_spacing               ),
                    tr( (digit_width / 2.0) - (segment_thickness / 4.0)                  ,  (digit_height / 2.0) - (segment_thickness / 4.0) - segment_spacing               ),
                    tr( (digit_width / 2.0)                                              ,  (digit_height / 2.0) - (segment_thickness / 2.0) - segment_spacing               ),
                    tr( (digit_width / 2.0)                                              ,                         (segment_thickness / 2.0) + segment_spacing + digit_median),
                    tr( (digit_width / 2.0) - (segment_thickness / 2.0)                  ,                                                     segment_spacing + digit_median),
                    tr( (digit_width / 2.0) - (segment_thickness / 1.0)                  ,                         (segment_thickness / 2.0) + segment_spacing + digit_median),
                ],
                vec![
                    tr( (digit_width / 2.0) - (segment_thickness / 4.0) - segment_spacing,  (digit_height / 2.0) - (segment_thickness / 4.0)                                 ),
                    tr( (digit_width / 2.0) - (segment_thickness / 2.0) - segment_spacing,  (digit_height / 2.0)                                                             ),
                    tr(                       (segment_thickness / 2.0) + segment_spacing,  (digit_height / 2.0)                                                             ),
                    tr(                                                   segment_spacing,  (digit_height / 2.0) - (segment_thickness / 2.0)                                 ),
                    tr(                       (segment_thickness / 2.0) + segment_spacing,  (digit_height / 2.0) - (segment_thickness / 1.0)                                 ),
                    tr( (digit_width / 2.0) - (segment_thickness / 1.0) - segment_spacing,  (digit_height / 2.0) - (segment_thickness / 1.0)                                 ),
                ],
                vec![
                    tr(-(digit_width / 2.0) + (segment_thickness / 4.0) + segment_spacing,  (digit_height / 2.0) - (segment_thickness / 4.0)                                 ),
                    tr(-(digit_width / 2.0) + (segment_thickness / 2.0) + segment_spacing,  (digit_height / 2.0)                                                             ),
                    tr(                     - (segment_thickness / 2.0) - segment_spacing,  (digit_height / 2.0)                                                             ),
                    tr(                                                 - segment_spacing,  (digit_height / 2.0) - (segment_thickness / 2.0)                                 ),
                    tr(                     - (segment_thickness / 2.0) - segment_spacing,  (digit_height / 2.0) - (segment_thickness / 1.0)                                 ),
                    tr(-(digit_width / 2.0) + (segment_thickness / 1.0) + segment_spacing,  (digit_height / 2.0) - (segment_thickness / 1.0)                                 ),
                ],
                vec![
                    tr(-(digit_width / 2.0) + (segment_thickness / 1.0)                  ,  (digit_height / 2.0) - (segment_thickness / 1.0) - segment_spacing               ),
                    tr(-(digit_width / 2.0) + (segment_thickness / 4.0)                  ,  (digit_height / 2.0) - (segment_thickness / 4.0) - segment_spacing               ),
                    tr(-(digit_width / 2.0)                                              ,  (digit_height / 2.0) - (segment_thickness / 2.0) - segment_spacing               ),
                    tr(-(digit_width / 2.0)                                              ,                         (segment_thickness / 2.0) + segment_spacing + digit_median),
                    tr(-(digit_width / 2.0) + (segment_thickness / 2.0)                  ,                                                     segment_spacing + digit_median),
                    tr(-(digit_width / 2.0) + (segment_thickness / 1.0)                  ,                         (segment_thickness / 2.0) + segment_spacing + digit_median),
                ],
                vec![
                    tr(-(digit_width / 2.0) + (segment_thickness / 1.0)                  , -(digit_height / 2.0) + (segment_thickness / 1.0) + segment_spacing               ),
                    tr(-(digit_width / 2.0) + (segment_thickness / 4.0)                  , -(digit_height / 2.0) + (segment_thickness / 4.0) + segment_spacing               ),
                    tr(-(digit_width / 2.0)                                              , -(digit_height / 2.0) + (segment_thickness / 2.0) + segment_spacing               ),
                    tr(-(digit_width / 2.0)                                              ,                       - (segment_thickness / 2.0) - segment_spacing + digit_median),
                    tr(-(digit_width / 2.0) + (segment_thickness / 2.0)                  ,                                                   - segment_spacing + digit_median),
                    tr(-(digit_width / 2.0) + (segment_thickness / 1.0)                  ,                       - (segment_thickness / 2.0) - segment_spacing + digit_median),
                ],
                vec![
                    tr(                                                 - segment_spacing,                                                   - segment_spacing + digit_median),
                    tr(                     - (segment_thickness / 2.0) - segment_spacing,                       - (segment_thickness / 1.0) - segment_spacing + digit_median),
                    tr(-(digit_width / 2.0) + (segment_thickness * 1.5) + segment_spacing, -(digit_height / 2.0) + (segment_thickness / 1.0) + segment_spacing               ),
                    tr(-(digit_width / 2.0) + (segment_thickness / 1.0) + segment_spacing, -(digit_height / 2.0) + (segment_thickness / 1.0) + segment_spacing               ),
                    tr(-(digit_width / 2.0) + (segment_thickness / 1.0) + segment_spacing, -(digit_height / 2.0) + (segment_thickness * 1.5) + segment_spacing               ),
                    tr(                     - (segment_thickness / 1.0) - segment_spacing,                       - (segment_thickness / 2.0) - segment_spacing + digit_median),
                ],
                vec![
                    tr(                     - (segment_thickness / 2.0)                  , -(digit_height / 2.0) + (segment_thickness / 1.0) + segment_spacing               ),
                    tr(                                                               0.0, -(digit_height / 2.0) + (segment_thickness / 2.0) + segment_spacing               ),
                    tr(                       (segment_thickness / 2.0)                  , -(digit_height / 2.0) + (segment_thickness / 1.0) + segment_spacing               ),
                    tr(                       (segment_thickness / 2.0)                  ,                       - (segment_thickness / 1.0) - segment_spacing + digit_median),
                    tr(                                                               0.0,                                                   - segment_spacing + digit_median),
                    tr(                     - (segment_thickness / 2.0)                  ,                       - (segment_thickness / 1.0) - segment_spacing + digit_median),
                ],
                vec![
                    tr(                       (segment_thickness / 2.0) + segment_spacing,                       - (segment_thickness / 1.0) - segment_spacing + digit_median),
                    tr( (digit_width / 2.0) - (segment_thickness * 1.5) - segment_spacing, -(digit_height / 2.0) + (segment_thickness / 1.0) + segment_spacing               ),
                    tr( (digit_width / 2.0) - (segment_thickness / 1.0) - segment_spacing, -(digit_height / 2.0) + (segment_thickness / 1.0) + segment_spacing               ),
                    tr( (digit_width / 2.0) - (segment_thickness / 1.0) - segment_spacing, -(digit_height / 2.0) + (segment_thickness * 1.5) + segment_spacing               ),
                    tr(                       (segment_thickness / 1.0) + segment_spacing,                       - (segment_thickness / 2.0) - segment_spacing + digit_median),
                    tr(                                                   segment_spacing,                                                   - segment_spacing + digit_median),
                ],
                vec![
                    tr(                       (segment_thickness / 1.0) + segment_spacing,                         (segment_thickness / 2.0)                   + digit_median),
                    tr(                                                   segment_spacing,                                                                       digit_median),
                    tr(                       (segment_thickness / 1.0) + segment_spacing,                       - (segment_thickness / 2.0)                   + digit_median),
                    tr( (digit_width / 2.0) - (segment_thickness / 1.0) - segment_spacing,                       - (segment_thickness / 2.0)                   + digit_median),
                    tr( (digit_width / 2.0) - (segment_thickness / 2.0) - segment_spacing,                                                                       digit_median),
                    tr( (digit_width / 2.0) - (segment_thickness / 1.0) - segment_spacing,                         (segment_thickness / 2.0)                   + digit_median),
                ],
                vec![
                    tr(                                                   segment_spacing,                                                     segment_spacing + digit_median),
                    tr(                       (segment_thickness / 2.0) + segment_spacing,                         (segment_thickness / 1.0) + segment_spacing + digit_median),
                    tr( (digit_width / 2.0) - (segment_thickness * 1.5) - segment_spacing,  (digit_height / 2.0) - (segment_thickness / 1.0) - segment_spacing               ),
                    tr( (digit_width / 2.0) - (segment_thickness / 1.0) - segment_spacing,  (digit_height / 2.0) - (segment_thickness / 1.0) - segment_spacing               ),
                    tr( (digit_width / 2.0) - (segment_thickness / 1.0) - segment_spacing,  (digit_height / 2.0) - (segment_thickness * 1.5) - segment_spacing               ),
                    tr(                       (segment_thickness / 1.0) + segment_spacing,                         (segment_thickness / 2.0) + segment_spacing + digit_median),
                ],
                vec![
                    tr(                     - (segment_thickness / 2.0)                  ,  (digit_height / 2.0) - (segment_thickness / 1.0) - segment_spacing               ),
                    tr(                                                               0.0,  (digit_height / 2.0) - (segment_thickness / 2.0) - segment_spacing               ),
                    tr(                       (segment_thickness / 2.0)                  ,  (digit_height / 2.0) - (segment_thickness / 1.0) - segment_spacing               ),
                    tr(                       (segment_thickness / 2.0)                  ,                         (segment_thickness / 1.0) + segment_spacing + digit_median),
                    tr(                                                               0.0,                                                     segment_spacing + digit_median),
                    tr(                     - (segment_thickness / 2.0)                  ,                         (segment_thickness / 1.0) + segment_spacing + digit_median),
                ],
                vec![
                    tr(                     - (segment_thickness / 2.0) - segment_spacing,                         (segment_thickness / 1.0) + segment_spacing + digit_median),
                    tr(-(digit_width / 2.0) + (segment_thickness * 1.5) + segment_spacing,  (digit_height / 2.0) - (segment_thickness / 1.0) - segment_spacing               ),
                    tr(-(digit_width / 2.0) + (segment_thickness / 1.0) + segment_spacing,  (digit_height / 2.0) - (segment_thickness / 1.0) - segment_spacing               ),
                    tr(-(digit_width / 2.0) + (segment_thickness / 1.0) + segment_spacing,  (digit_height / 2.0) - (segment_thickness * 1.5) - segment_spacing               ),
                    tr(                     - (segment_thickness / 1.0) - segment_spacing,                         (segment_thickness / 2.0) + segment_spacing + digit_median),
                    tr(                                                 - segment_spacing,                                                     segment_spacing + digit_median),
                ],
                vec![
                    tr(                     - (segment_thickness / 1.0) - segment_spacing,                         (segment_thickness / 2.0)                   + digit_median),
                    tr(                                                 - segment_spacing,                                                                       digit_median),
                    tr(                     - (segment_thickness / 1.0) - segment_spacing,                       - (segment_thickness / 2.0)                   + digit_median),
                    tr(-(digit_width / 2.0) + (segment_thickness / 1.0) + segment_spacing,                       - (segment_thickness / 2.0)                   + digit_median),
                    tr(-(digit_width / 2.0) + (segment_thickness / 2.0) + segment_spacing,                                                                       digit_median),
                    tr(-(digit_width / 2.0) + (segment_thickness / 1.0) + segment_spacing,                         (segment_thickness / 2.0)                   + digit_median),
                ],
            ],
        }
    }
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

#[must_use = "You should put this widget in an ui with `ui.add(widget);`"]
pub struct SegmentedDisplayWidget<'a> {
    display_kind: SegmentedDisplayKind,
    digits: Vec<SegmentedDisplayDigit>,
    digit_height: f32,
    metrics: SegmentedDisplayMetrics,
    style: SegmentedDisplayStyle,
    font: &'a SegmentedDisplayFont,
    show_dots: bool,
    show_colons: bool,
    show_apostrophes: bool,
}

impl<'a> SegmentedDisplayWidget<'a> {
    pub fn new(display_kind: SegmentedDisplayKind) -> Self {
        Self {
            display_kind,
            digits: Vec::new(),
            digit_height: 48.0,
            metrics: SegmentedDisplayMetrics::default(),
            style: SegmentedDisplayStylePreset::Default.style(),
            font: display_kind.default_font(),
            show_dots: true,
            show_colons: true,
            show_apostrophes: true,
        }
    }

    pub fn push_string(mut self, value: &str) -> Self {
        self.digits.extend(
            [None]
                .into_iter()
                .chain(value.chars().map(|e| Some(e)))
                .chain([None])
                .tuple_windows()
                .flat_map(|(prev, curr, next)| match curr {
                    Some('.') if self.show_dots => None,
                    Some(':') if self.show_colons => None,
                    Some('\'') if self.show_apostrophes => None,
                    Some(c) if c.is_ascii() => Some(SegmentedDisplayDigit {
                        segments: self.font[c as usize],
                        dot: (next == Some('.')) && self.show_dots,
                        colon: (prev == Some(':')) && self.show_colons,
                        apostrophe: (prev == Some('\'')) && self.show_apostrophes,
                    }),
                    _ => None,
                }),
        );
        self
    }

    pub fn push_digit(mut self, digit: SegmentedDisplayDigit) -> Self {
        self.digits.push(digit);
        self
    }

    pub fn digit_height(mut self, digit_height: impl Into<f32>) -> Self {
        self.digit_height = digit_height.into();
        self
    }

    pub fn style(mut self, style: SegmentedDisplayStyle) -> Self {
        self.style = style;
        self
    }

    pub fn style_preset(mut self, preset: SegmentedDisplayStylePreset) -> Self {
        self.style = preset.style();
        self
    }

    pub fn metrics(mut self, metrics: SegmentedDisplayMetrics) -> Self {
        self.metrics = metrics;
        self
    }

    pub fn metrics_preset(mut self, preset: SegmentedDisplayMetricsPreset) -> Self {
        self.metrics = preset.metrics();
        self
    }

    pub fn font(mut self, font: &'a SegmentedDisplayFont) -> Self {
        self.font = font;
        self
    }

    pub fn show_dots(mut self, show_dots: bool) -> Self {
        self.show_dots = show_dots;
        self
    }

    pub fn show_colons(mut self, show_colons: bool) -> Self {
        self.show_colons = show_colons;
        self
    }

    pub fn show_apostrophes(mut self, show_apostrophes: bool) -> Self {
        self.show_apostrophes = show_apostrophes;
        self
    }
}

impl<'a> Widget for SegmentedDisplayWidget<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        let digit_height = self.digit_height;
        let digit_width = digit_height * self.metrics.digit_ratio;

        // Turn relative metrics to absolute metrics
        let segment_thickness = self.metrics.segment_thickness * digit_height;
        let segment_spacing = self.metrics.segment_spacing * digit_height;
        let digit_shearing = self.metrics.digit_shearing * digit_width;
        let digit_spacing = self.metrics.digit_spacing * digit_width;
        let margin_horizontal = self.metrics.margin_horizontal * digit_width;
        let margin_vertical = self.metrics.margin_vertical * digit_height;
        let digit_median = self.metrics.digit_median * (digit_height / 2.0);
        let colon_separation = self.metrics.colon_separation * (digit_height / 2.0);

        let desired_size = vec2(
            (digit_width * self.digits.len() as f32)
                + (digit_spacing * (self.digits.len().saturating_sub(1)) as f32)
                + (2.0 * margin_horizontal)
                + (2.0 * digit_shearing.abs()),
            digit_height + (2.0 * margin_vertical),
        );

        let (rect, response) = ui.allocate_exact_size(desired_size, Sense::hover());

        if ui.is_rect_visible(rect) {
            ui.painter().rect(
                rect,
                ui.style().visuals.noninteractive().rounding,
                self.style.background_color,
                Stroke::none(),
            );

            let paint_digit = |digit: &SegmentedDisplayDigit, digit_center: Pos2| {
                let tr = move |dx, dy| {
                    digit_center + vec2(dx, dy)
                        - vec2((dy / (digit_height / 2.0)) * digit_shearing, 0.0)
                };

                let segment_points = self.display_kind.geometry(
                    &tr,
                    digit_width,
                    digit_height,
                    segment_thickness,
                    segment_spacing,
                    digit_median,
                );

                #[rustfmt::skip]
                let apostrophe_points: Vec<Pos2> = vec![
                    tr(-(digit_width / 2.0) - (digit_spacing / 2.0) - (segment_thickness / 2.0), -(digit_height / 2.0)                            ),
                    tr(-(digit_width / 2.0) - (digit_spacing / 2.0) + (segment_thickness / 2.0), -(digit_height / 2.0)                            ),
                    tr(-(digit_width / 2.0) - (digit_spacing / 2.0) - (segment_thickness / 2.0), -(digit_height / 2.0) + (segment_thickness * 2.0)),
                ];

                #[rustfmt::skip]
                let (colon_top_pos, colon_bottom_pos, dot_pos) = (
                    tr(-(digit_width / 2.0) - (digit_spacing / 2.0), digit_median - colon_separation),
                    tr(-(digit_width / 2.0) - (digit_spacing / 2.0), digit_median + colon_separation),
                    tr( (digit_width / 2.0) + (digit_spacing / 2.0), (digit_height / 2.0) - (segment_thickness / 2.0))
                );

                for (segment_index, segment_points) in segment_points.iter().enumerate() {
                    let segment_on = ((digit.segments >> segment_index) & 0x01) != 0x00;

                    // TODO: concave_polygon
                    // https://github.com/emilk/egui/issues/513
                    ui.painter().add(Shape::convex_polygon(
                        segment_points.to_vec(),
                        self.style.segment_color(segment_on),
                        self.style.segment_stroke(segment_on),
                    ));
                }

                if self.show_dots {
                    ui.painter().circle(
                        dot_pos,
                        segment_thickness / 2.0,
                        self.style.segment_color(digit.dot),
                        self.style.segment_stroke(digit.dot),
                    );
                }

                if self.show_colons {
                    ui.painter().circle(
                        colon_top_pos,
                        segment_thickness / 2.0,
                        self.style.segment_color(digit.colon),
                        self.style.segment_stroke(digit.colon),
                    );

                    ui.painter().circle(
                        colon_bottom_pos,
                        segment_thickness / 2.0,
                        self.style.segment_color(digit.colon),
                        self.style.segment_stroke(digit.colon),
                    );
                }

                if self.show_apostrophes {
                    ui.painter().add(Shape::convex_polygon(
                        apostrophe_points.to_vec(),
                        self.style.segment_color(digit.apostrophe),
                        self.style.segment_stroke(digit.apostrophe),
                    ));
                }
            };

            for (digit_index, digit) in self.digits.iter().enumerate() {
                let digit_center = rect.left_center()
                    + vec2(
                        margin_horizontal
                            + digit_shearing.abs()
                            + ((digit_width + digit_spacing) * digit_index as f32)
                            + (digit_width / 2.0),
                        0.0,
                    );

                paint_digit(digit, digit_center);
            }
        }

        response
    }
}
