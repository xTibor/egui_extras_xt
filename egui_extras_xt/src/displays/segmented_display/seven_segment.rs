use egui::{pos2, Pos2};
use itertools::Itertools;

use crate::displays::segmented_display::{DisplayGlyph, DisplayImpl};

// ----------------------------------------------------------------------------

#[derive(Clone, Copy, Default)]
pub struct SevenSegment;

impl DisplayImpl for SevenSegment {
    fn segment_count(&self) -> usize {
        7
    }

    fn glyph(&self, c: char) -> Option<DisplayGlyph> {
        #[rustfmt::skip]
        const GLYPH_DATA: &[(char, DisplayGlyph)] = &[
            // Basic Latin
            (' ',  0x0000), ('!',  0x0030), ('"',  0x0022), ('#',  0x0000),
            ('$',  0x0000), ('%',  0x0000), ('&',  0x0000), ('\'', 0x0002),
            ('(',  0x0039), (')',  0x000F), ('*',  0x0000), ('+',  0x0000),
            (',',  0x000C), ('-',  0x0040), ('.',  0x0004), ('/',  0x0052),
            ('0',  0x003F), ('1',  0x0006), ('2',  0x005B), ('3',  0x004F),
            ('4',  0x0066), ('5',  0x006D), ('6',  0x007D), ('7',  0x0027),
            ('8',  0x007F), ('9',  0x006F), (':',  0x0048), (';',  0x0048),
            ('<',  0x0039), ('=',  0x0048), ('>',  0x000F), ('?',  0x0053),
            ('@',  0x007B), ('A',  0x0077), ('B',  0x007C), ('C',  0x0039),
            ('D',  0x005E), ('E',  0x0079), ('F',  0x0071), ('G',  0x003D),
            ('H',  0x0076), ('I',  0x0030), ('J',  0x001E), ('K',  0x0076),
            ('L',  0x0038), ('M',  0x002B), ('N',  0x0037), ('O',  0x003F),
            ('P',  0x0073), ('Q',  0x0067), ('R',  0x0077), ('S',  0x006D),
            ('T',  0x0007), ('U',  0x003E), ('V',  0x003E), ('W',  0x007E),
            ('X',  0x0076), ('Y',  0x006E), ('Z',  0x005B), ('[',  0x0039),
            ('\\', 0x0064), (']',  0x000F), ('^',  0x0023), ('_',  0x0008),
            ('`',  0x0020), ('a',  0x005F), ('b',  0x007C), ('c',  0x0058),
            ('d',  0x005E), ('e',  0x007B), ('f',  0x0071), ('g',  0x006F),
            ('h',  0x0074), ('i',  0x0010), ('j',  0x000E), ('k',  0x0076),
            ('l',  0x0006), ('m',  0x0055), ('n',  0x0054), ('o',  0x005C),
            ('p',  0x0073), ('q',  0x0067), ('r',  0x0050), ('s',  0x006D),
            ('t',  0x0078), ('u',  0x001C), ('v',  0x001C), ('w',  0x007E),
            ('x',  0x0076), ('y',  0x006E), ('z',  0x005B), ('{',  0x0046),
            ('|',  0x0030), ('}',  0x0070), ('~',  0x0040),
        ];

        assert!(GLYPH_DATA
            .iter()
            .map(|(k, _)| k)
            .tuple_windows()
            .all(|(k1, k2)| k1 < k2)); // is_sorted()

        GLYPH_DATA
            .binary_search_by_key(&c, |(k, _)| *k)
            .ok()
            .map(|index| GLYPH_DATA[index].1)
    }

    #[rustfmt::skip]
    #[allow(unused_parens)]
    fn geometry(
        &self,
        digit_width: f32,
        digit_height: f32,
        segment_thickness: f32,
        segment_spacing: f32,
        digit_median: f32,
    ) -> Vec<Vec<Pos2>> {
        vec![
            vec![
                pos2(-(digit_width / 2.0) + (segment_thickness / 4.0) + segment_spacing, -(digit_height / 2.0) + (segment_thickness / 4.0)                                 ),
                pos2(-(digit_width / 2.0) + (segment_thickness / 2.0) + segment_spacing, -(digit_height / 2.0)                                                             ),
                pos2( (digit_width / 2.0) - (segment_thickness / 2.0) - segment_spacing, -(digit_height / 2.0)                                                             ),
                pos2( (digit_width / 2.0) - (segment_thickness / 4.0) - segment_spacing, -(digit_height / 2.0) + (segment_thickness / 4.0)                                 ),
                pos2( (digit_width / 2.0) - (segment_thickness / 1.0) - segment_spacing, -(digit_height / 2.0) + (segment_thickness / 1.0)                                 ),
                pos2(-(digit_width / 2.0) + (segment_thickness / 1.0) + segment_spacing, -(digit_height / 2.0) + (segment_thickness / 1.0)                                 ),
            ],
            vec![
                pos2( (digit_width / 2.0) - (segment_thickness / 1.0)                  , -(digit_height / 2.0) + (segment_thickness / 1.0) + segment_spacing               ),
                pos2( (digit_width / 2.0) - (segment_thickness / 4.0)                  , -(digit_height / 2.0) + (segment_thickness / 4.0) + segment_spacing               ),
                pos2( (digit_width / 2.0)                                              , -(digit_height / 2.0) + (segment_thickness / 2.0) + segment_spacing               ),
                pos2( (digit_width / 2.0)                                              ,                       - (segment_thickness / 2.0) - segment_spacing + digit_median),
                pos2( (digit_width / 2.0) - (segment_thickness / 2.0)                  ,                                                   - segment_spacing + digit_median),
                pos2( (digit_width / 2.0) - (segment_thickness / 1.0)                  ,                       - (segment_thickness / 2.0) - segment_spacing + digit_median),
            ],
            vec![
                pos2( (digit_width / 2.0) - (segment_thickness / 1.0)                  ,  (digit_height / 2.0) - (segment_thickness / 1.0) - segment_spacing               ),
                pos2( (digit_width / 2.0) - (segment_thickness / 4.0)                  ,  (digit_height / 2.0) - (segment_thickness / 4.0) - segment_spacing               ),
                pos2( (digit_width / 2.0)                                              ,  (digit_height / 2.0) - (segment_thickness / 2.0) - segment_spacing               ),
                pos2( (digit_width / 2.0)                                              ,                         (segment_thickness / 2.0) + segment_spacing + digit_median),
                pos2( (digit_width / 2.0) - (segment_thickness / 2.0)                  ,                                                     segment_spacing + digit_median),
                pos2( (digit_width / 2.0) - (segment_thickness / 1.0)                  ,                         (segment_thickness / 2.0) + segment_spacing + digit_median),
            ],
            vec![
                pos2(-(digit_width / 2.0) + (segment_thickness / 4.0) + segment_spacing,  (digit_height / 2.0) - (segment_thickness / 4.0)                                 ),
                pos2(-(digit_width / 2.0) + (segment_thickness / 2.0) + segment_spacing,  (digit_height / 2.0)                                                             ),
                pos2( (digit_width / 2.0) - (segment_thickness / 2.0) - segment_spacing,  (digit_height / 2.0)                                                             ),
                pos2( (digit_width / 2.0) - (segment_thickness / 4.0) - segment_spacing,  (digit_height / 2.0) - (segment_thickness / 4.0)                                 ),
                pos2( (digit_width / 2.0) - (segment_thickness / 1.0) - segment_spacing,  (digit_height / 2.0) - (segment_thickness / 1.0)                                 ),
                pos2(-(digit_width / 2.0) + (segment_thickness / 1.0) + segment_spacing,  (digit_height / 2.0) - (segment_thickness / 1.0)                                 ),
            ],
            vec![
                pos2(-(digit_width / 2.0) + (segment_thickness / 1.0)                  ,  (digit_height / 2.0) - (segment_thickness / 1.0) - segment_spacing               ),
                pos2(-(digit_width / 2.0) + (segment_thickness / 4.0)                  ,  (digit_height / 2.0) - (segment_thickness / 4.0) - segment_spacing               ),
                pos2(-(digit_width / 2.0)                                              ,  (digit_height / 2.0) - (segment_thickness / 2.0) - segment_spacing               ),
                pos2(-(digit_width / 2.0)                                              ,                         (segment_thickness / 2.0) + segment_spacing + digit_median),
                pos2(-(digit_width / 2.0) + (segment_thickness / 2.0)                  ,                                                     segment_spacing + digit_median),
                pos2(-(digit_width / 2.0) + (segment_thickness / 1.0)                  ,                         (segment_thickness / 2.0) + segment_spacing + digit_median),
            ],
            vec![
                pos2(-(digit_width / 2.0) + (segment_thickness / 1.0)                  , -(digit_height / 2.0) + (segment_thickness / 1.0) + segment_spacing               ),
                pos2(-(digit_width / 2.0) + (segment_thickness / 4.0)                  , -(digit_height / 2.0) + (segment_thickness / 4.0) + segment_spacing               ),
                pos2(-(digit_width / 2.0)                                              , -(digit_height / 2.0) + (segment_thickness / 2.0) + segment_spacing               ),
                pos2(-(digit_width / 2.0)                                              ,                       - (segment_thickness / 2.0) - segment_spacing + digit_median),
                pos2(-(digit_width / 2.0) + (segment_thickness / 2.0)                  ,                                                   - segment_spacing + digit_median),
                pos2(-(digit_width / 2.0) + (segment_thickness / 1.0)                  ,                       - (segment_thickness / 2.0) - segment_spacing + digit_median),
            ],
            vec![
                pos2(-(digit_width / 2.0) + (segment_thickness / 2.0) + segment_spacing,                                                                       digit_median),
                pos2(-(digit_width / 2.0) + (segment_thickness / 1.0) + segment_spacing,                       - (segment_thickness / 2.0)                   + digit_median),
                pos2( (digit_width / 2.0) - (segment_thickness / 1.0) - segment_spacing,                       - (segment_thickness / 2.0)                   + digit_median),
                pos2( (digit_width / 2.0) - (segment_thickness / 2.0) - segment_spacing,                                                                       digit_median),
                pos2( (digit_width / 2.0) - (segment_thickness / 1.0) - segment_spacing,                         (segment_thickness / 2.0)                   + digit_median),
                pos2(-(digit_width / 2.0) + (segment_thickness / 1.0) + segment_spacing,                         (segment_thickness / 2.0)                   + digit_median),
            ],
        ]
    }
}
