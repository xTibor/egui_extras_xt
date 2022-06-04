use egui::Pos2;

use crate::segmented_display::{DisplayFontGlyph, DisplayKind, SegmentGeometryTransformFn};

// ----------------------------------------------------------------------------

#[derive(Copy, Clone, Default)]
pub struct SixteenSegment;

impl DisplayKind for SixteenSegment {
    #[rustfmt::skip]
    fn glyph(&self, c: char) -> Option<DisplayFontGlyph> {
        match c {
            // Basic Latin
            '\u{000000}'..='\u{00007F}' => Some([
                0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, // 00-07:  ×  ×  ×  ×  ×  ×  ×  ×
                0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, // 08-0F:  ×  ×  ×  ×  ×  ×  ×  ×
                0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, // 10-17:  ×  ×  ×  ×  ×  ×  ×  ×
                0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, // 18-1F:  ×  ×  ×  ×  ×  ×  ×  ×
                0x0000, 0x2200, 0x0280, 0xAA3C, 0xAABB, 0xEE99, 0x9379, 0x0080, // 20-27:     !  "  #  $  %  &  '
                0x1400, 0x4100, 0xDD00, 0xAA00, 0xC000, 0x8800, 0x0020, 0x4400, // 28-2F:  (  )  *  +  ,  -  .  /
                0x44FF, 0x040C, 0x8877, 0x883F, 0x888C, 0x88BB, 0x88FB, 0x2483, // 30-37:  0  1  2  3  4  5  6  7
                0x88FF, 0x88BF, 0x8020, 0xC001, 0x9400, 0x8830, 0x4900, 0x2887, // 38-3F:  8  9  :  ;  <  =  >  ?
                0x28DF, 0x88CF, 0x2A3F, 0x00F3, 0x223F, 0x80F3, 0x80C3, 0x08FB, // 40-47:  @  A  B  C  D  E  F  G
                0x88CC, 0x2233, 0x007E, 0x94C0, 0x00F0, 0x05CC, 0x11CC, 0x00FF, // 48-4F:  H  I  J  K  L  M  N  O
                0x88C7, 0x10FF, 0x98C7, 0x093B, 0x2203, 0x00FC, 0x44C0, 0x50CC, // 50-57:  P  Q  R  S  T  U  V  W
                0x5500, 0x2500, 0x4433, 0x2212, 0x1100, 0x2221, 0x0404, 0x0030, // 58-5F:  X  Y  Z  [  \  ]  ^  _
                0x0100, 0xA070, 0xA0E0, 0x8060, 0xA260, 0xC060, 0xAA02, 0x1818, // 60-67:  `  a  b  c  d  e  f  g
                0xA0C0, 0x0040, 0x2220, 0x3A00, 0x00E0, 0xA848, 0xA040, 0xA060, // 68-6F:  h  i  j  k  l  m  n  o
                0x82C1, 0xA281, 0x8040, 0x1810, 0xAA10, 0x2060, 0x4040, 0x5048, // 70-77:  p  q  r  s  t  u  v  w
                0xD800, 0x1018, 0xC020, 0xA212, 0x2200, 0x2A21, 0x0A85, 0x0000, // 78-7F:  x  y  z  {  |  }  ~  ×
            ][c as usize]),

            // Currency symbols
            '¢' => Some(0x8060),
            '£' => Some(0xA276),
            '¥' => Some(0xAD00),
            '€' => Some(0xEA12),
            '元' => Some(0xD803),
            '円' => Some(0x8ADF),

            // Emoji
            '✔' => Some(0x4440),
            '🍀' => Some(0xFF55),
            '🎂' => Some(0x8AFC),
            '🎈' => Some(0xC887),
            '🐱' => Some(0x05FC),
            '👍' => Some(0xAC38),
            '👎' => Some(0x9A07),
            '🔫' => Some(0x988F),
            '🤣' => Some(0x0578),
            '🥵' => Some(0x50B4),
            '😂' => Some(0x0578),
            '😉' => Some(0x04B8),
            '😊' => Some(0x0578),
            '😏' => Some(0x04B8),
            '😐' => Some(0x00B4),
            '😑' => Some(0x00B4),
            '😒' => Some(0x00B4),
            '😕' => Some(0x5084),
            '😞' => Some(0x5084),
            '😢' => Some(0x5084),
            '😥' => Some(0x5084),
            '😩' => Some(0x50B4),
            '😫' => Some(0x50B4),
            '😭' => Some(0x50B4),
            '😲' => Some(0x50B4),
            '😳' => Some(0x8AB7),
            '🙁' => Some(0x5084),

            // Other
            '§' => Some(0x11BB),
            '¶' => Some(0xA28F),
            '±' => Some(0xAA30),
            '²' => Some(0x0C02),
            '³' => Some(0x0C06),
            'µ' => Some(0x82C0),

            _ => None,
        }
    }

    #[rustfmt::skip]
    #[allow(unused_parens)]
    fn geometry(
        &self,
        tr: &SegmentGeometryTransformFn,
        digit_width: f32,
        digit_height: f32,
        segment_thickness: f32,
        segment_spacing: f32,
        digit_median: f32,
    ) -> Vec<Vec<Pos2>> {
        vec![
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
        ]
    }
}

impl SixteenSegment {
    #[rustfmt::skip]
    pub const GLYPHS_HALFWIDTH_NUMBERS: [DisplayFontGlyph; 20] = [
        0x221E, 0x000C, 0x2816, 0x081E, 0x0A0C, 0x0A1A, 0x2A1A, 0x020E,
        0x2A1E, 0x0A1E, 0x22DE, 0x00CC, 0x28D6, 0x08DE, 0x0ACC, 0x0ADA,
        0x2ADA, 0x02CE, 0x2ADE, 0x0ADE,
    ];

    #[rustfmt::skip]
    pub const GLYPHS_FADE_LEFT_RIGHT: [DisplayFontGlyph; 6] = [
        0x0000, 0x00C0, 0xC1E1, 0xE3E1, 0xFFF3, 0xFFFF,
    ];

    #[rustfmt::skip]
    pub const GLYPHS_FADE_RIGHT_LEFT: [DisplayFontGlyph; 6] = [
        0x0000, 0x000C, 0x1C1E, 0x3E1E, 0xFF3F, 0xFFFF,
    ];

    #[rustfmt::skip]
    pub const GLYPHS_FADE_TOP_BOTTOM: [DisplayFontGlyph; 6] = [
        0x0000, 0x0003, 0x0787, 0x8F87, 0xFFCF, 0xFFFF,
    ];

    #[rustfmt::skip]
    pub const GLYPHS_FADE_BOTTOM_TOP: [DisplayFontGlyph; 6] = [
        0x0000, 0x0030, 0x7078, 0xF878, 0xFFFC, 0xFFFF,
    ];

    #[rustfmt::skip]
    pub const GLYPHS_BLOCKS: [DisplayFontGlyph; 16] = [
        0x0000, 0x8381, 0x0E06, 0x8F87, 0xE060, 0xE3E1, 0xEE66, 0xEFE7,
        0x3818, 0xBB99, 0x3E1E, 0xBF9F, 0xF878, 0xFBF9, 0xFE7E, 0xFFFF,
    ];

    #[rustfmt::skip]
    pub const GLYPHS_SPINNER_1: [DisplayFontGlyph; 64] = [
        0x02FF, 0x06FF, 0x0AFF, 0x12FF, 0x22FF, 0x42FF, 0x82FF, 0x03FF,
        0x06FF, 0x04FF, 0x0CFF, 0x14FF, 0x24FF, 0x44FF, 0x84FF, 0x05FF,
        0x0AFF, 0x0CFF, 0x08FF, 0x18FF, 0x28FF, 0x48FF, 0x88FF, 0x09FF,
        0x12FF, 0x14FF, 0x18FF, 0x10FF, 0x30FF, 0x50FF, 0x90FF, 0x11FF,
        0x22FF, 0x24FF, 0x28FF, 0x30FF, 0x20FF, 0x60FF, 0xA0FF, 0x21FF,
        0x42FF, 0x44FF, 0x48FF, 0x50FF, 0x60FF, 0x40FF, 0xC0FF, 0x41FF,
        0x82FF, 0x84FF, 0x88FF, 0x90FF, 0xA0FF, 0xC0FF, 0x80FF, 0x81FF,
        0x03FF, 0x05FF, 0x09FF, 0x11FF, 0x21FF, 0x41FF, 0x81FF, 0x01FF,
    ];

    #[rustfmt::skip]
    pub const GLYPHS_SPINNER_2: [DisplayFontGlyph; 60] = [
        0x00C0, 0x0081, 0x0003, 0x0006, 0x000C, 0x0018, 0x0030, 0x0060,
        0x00C0, 0x0081, 0x0003, 0x0006, 0x000C, 0x0018, 0x0030, 0x0060,
        0x00C0, 0x0081, 0x0003, 0x0006, 0x000C, 0x0018, 0x0030, 0x0060,
        0x00C0, 0x0081, 0x0201, 0x2200, 0x2010, 0x0018,
        0x000C, 0x0006, 0x0003, 0x0081, 0x00C0, 0x0060, 0x0030, 0x0018,
        0x000C, 0x0006, 0x0003, 0x0081, 0x00C0, 0x0060, 0x0030, 0x0018,
        0x000C, 0x0006, 0x0003, 0x0081, 0x00C0, 0x0060, 0x0030, 0x0018,
        0x000C, 0x0006, 0x0202, 0x2200, 0x2020, 0x0060,
    ];

    #[rustfmt::skip]
    pub const GLYPHS_SPINNER_3: [DisplayFontGlyph; 8] = [
        0xE3E1, 0xC7C3, 0x8F87, 0x1F0F, 0x3E1E, 0x7C3C, 0xF878, 0xF1F0,
    ];

    #[rustfmt::skip]
    pub const GLYPHS_SPINNER_4: [DisplayFontGlyph; 8] = [
        0x02FF, 0x04FF, 0x08FF, 0x10FF, 0x20FF, 0x40FF, 0x80FF, 0x01FF,
    ];

    #[rustfmt::skip]
    pub const GLYPHS_SPINNER_5: [DisplayFontGlyph; 8] = [
        0x8281, 0x0503, 0x0A06, 0x140C, 0x2818, 0x5030, 0xA060, 0x41C0,
    ];
}
