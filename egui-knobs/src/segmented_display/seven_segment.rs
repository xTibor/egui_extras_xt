use egui::Pos2;

use crate::segmented_display::{
    SegmentGeometryTransformFn, SegmentedDisplayFont, SegmentedDisplayKind,
};

// ----------------------------------------------------------------------------

#[derive(Copy, Clone, Default)]
pub struct SevenSegment;

impl SegmentedDisplayKind for SevenSegment {
    #[rustfmt::skip]
    fn default_font<'a>(&self) -> &'a SegmentedDisplayFont {
        &[
            0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, // 00-07:  ×  ×  ×  ×  ×  ×  ×  ×
            0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, // 08-0F:  ×  ×  ×  ×  ×  ×  ×  ×
            0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, // 10-17:  ×  ×  ×  ×  ×  ×  ×  ×
            0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, // 18-1F:  ×  ×  ×  ×  ×  ×  ×  ×
            0x0000, 0x0000, 0x0022, 0x0000, 0x0000, 0x0000, 0x0000, 0x0002, // 20-27:     ×  "  ×  ×  ×  ×  '
            0x0039, 0x000F, 0x0000, 0x0000, 0x000C, 0x0040, 0x0004, 0x0052, // 28-2F:  (  )  ×  ×  ,  -  .  /
            0x003F, 0x0006, 0x005B, 0x004F, 0x0066, 0x006D, 0x007D, 0x0027, // 30-37:  0  1  2  3  4  5  6  7
            0x007F, 0x006F, 0x0000, 0x0000, 0x0039, 0x0048, 0x000F, 0x0053, // 38-3F:  8  9  ×  ×  <  =  >  ?
            0x007B, 0x0077, 0x007C, 0x0039, 0x005E, 0x0079, 0x0071, 0x003D, // 40-47:  @  A  B  C  D  E  F  G
            0x0076, 0x0030, 0x001E, 0x0076, 0x0038, 0x002B, 0x0037, 0x003F, // 48-4F:  H  I  J  K  L  M  N  O
            0x0073, 0x0067, 0x0077, 0x006D, 0x0007, 0x003E, 0x003E, 0x007E, // 50-57:  P  Q  R  S  T  U  V  W
            0x0076, 0x006E, 0x005B, 0x0039, 0x0064, 0x000F, 0x0023, 0x0008, // 58-5F:  X  Y  Z  [  \  ]  ^  _
            0x0020, 0x005F, 0x007C, 0x0058, 0x005E, 0x007B, 0x0071, 0x006F, // 60-67:  `  a  b  c  d  e  f  g
            0x0074, 0x0010, 0x000E, 0x0076, 0x0006, 0x0055, 0x0054, 0x005C, // 68-6F:  h  i  j  k  l  m  n  o
            0x0073, 0x0067, 0x0050, 0x006D, 0x0078, 0x001C, 0x001C, 0x007E, // 70-77:  p  q  r  s  t  u  v  w
            0x0076, 0x006E, 0x005B, 0x0046, 0x0030, 0x0070, 0x0040, 0x0000, // 78-7F:  x  y  z  {  |  }  ~  ×
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
        ]
    }
}
