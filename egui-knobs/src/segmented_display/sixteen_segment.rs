use egui::Pos2;

use crate::segmented_display::{
    SegmentGeometryTransformFn, SegmentedDisplayFont, SegmentedDisplayKind,
};

// ----------------------------------------------------------------------------

#[derive(Copy, Clone, Default)]
pub struct SixteenSegment;

impl SegmentedDisplayKind for SixteenSegment {
    #[rustfmt::skip]
    fn default_font<'a>(&self) -> &'a SegmentedDisplayFont {
        &[
            0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, // 00-07:  ×  ×  ×  ×  ×  ×  ×  ×
            0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, // 08-0F:  ×  ×  ×  ×  ×  ×  ×  ×
            0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, // 10-17:  ×  ×  ×  ×  ×  ×  ×  ×
            0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, // 18-1F:  ×  ×  ×  ×  ×  ×  ×  ×
            0x0000, 0x2200, 0x0280, 0xAA3C, 0xAABB, 0xEE99, 0x9371, 0x0080, // 20-27:     !  "  #  $  %  &  '
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
        ]
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
    pub const FONT_SIXTEENSEGMENT_SYMBOLS: SegmentedDisplayFont = [
        0x221E, 0x000C, 0x2816, 0x081E, 0x0A0C, 0x0A1A, 0x2A1A, 0x020E, // 00-07:  0  1  2  3  4  5  6  7
        0x2A1E, 0x0A1E, 0x22DE, 0x00CC, 0x28D6, 0x08DE, 0x0ACC, 0x0ADA, // 08-0F:  8  9 10 11 12 13 14 15
        0x2ADA, 0x02CE, 0x2ADE, 0x0ADE, 0x0000, 0x0000, 0x0000, 0x0000, // 10-17: 16 17 18 19  ×  ×  ×  ×
        0x02FF, 0x04FF, 0x08FF, 0x10FF, 0x20FF, 0x40FF, 0x80FF, 0x01FF, // 18-1F: COMPASS-COMPASS-COMPASS
        0x0000, 0x00C0, 0xC1E1, 0xE3E1, 0xFFF3, 0xFFFF, 0x0000, 0x0000, // 20-27: FADE-LEFT-TO-RIGHT ×  ×
        0x0000, 0x000C, 0x1C1E, 0x3E1E, 0xFF3F, 0xFFFF, 0x0000, 0x0000, // 28-2F: FADE-RIGHT-TO-LEFT ×  ×
        0x0000, 0x0003, 0x0787, 0x8F87, 0xFFCF, 0xFFFF, 0x0000, 0x0000, // 30-37: FADE-TOP-TO-BOTTOM ×  ×
        0x0000, 0x0030, 0x7078, 0xF878, 0xFFFC, 0xFFFF, 0x0000, 0x0000, // 38-3F: FADE-BOTTOM-TO-TOP ×  ×
        0x02FF, 0x06FF, 0x0AFF, 0x12FF, 0x22FF, 0x42FF, 0x82FF, 0x03FF, // 40-47: CLOCK-CLOCK-CLOCK-CLOCK
        0x06FF, 0x04FF, 0x0CFF, 0x14FF, 0x24FF, 0x44FF, 0x84FF, 0x05FF, // 48-4F: CLOCK-CLOCK-CLOCK-CLOCK
        0x0AFF, 0x0CFF, 0x08FF, 0x18FF, 0x28FF, 0x48FF, 0x88FF, 0x09FF, // 50-57: CLOCK-CLOCK-CLOCK-CLOCK
        0x12FF, 0x14FF, 0x18FF, 0x10FF, 0x30FF, 0x50FF, 0x90FF, 0x11FF, // 58-5F: CLOCK-CLOCK-CLOCK-CLOCK
        0x22FF, 0x24FF, 0x28FF, 0x30FF, 0x20FF, 0x60FF, 0xA0FF, 0x21FF, // 60-67: CLOCK-CLOCK-CLOCK-CLOCK
        0x42FF, 0x44FF, 0x48FF, 0x50FF, 0x60FF, 0x40FF, 0xC0FF, 0x41FF, // 68-6F: CLOCK-CLOCK-CLOCK-CLOCK
        0x82FF, 0x84FF, 0x88FF, 0x90FF, 0xA0FF, 0xC0FF, 0x80FF, 0x81FF, // 70-77: CLOCK-CLOCK-CLOCK-CLOCK
        0x03FF, 0x05FF, 0x09FF, 0x11FF, 0x21FF, 0x41FF, 0x81FF, 0x01FF, // 78-7F: CLOCK-CLOCK-CLOCK-CLOCK
    ];
}
