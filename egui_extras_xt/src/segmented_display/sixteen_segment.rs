use egui::Pos2;
use itertools::Itertools;

use crate::segmented_display::{DisplayFontGlyph, DisplayKind, SegmentGeometryTransformFn};

// ----------------------------------------------------------------------------

#[derive(Copy, Clone, Default)]
pub struct SixteenSegment;

impl DisplayKind for SixteenSegment {
    fn glyph(&self, c: char) -> Option<DisplayFontGlyph> {
        #[rustfmt::skip]
        const GLYPH_DATA: &[(char, DisplayFontGlyph)] = &[
            // Basic Latin
            (' ',  0x0000), ('!',  0x2200), ('"',  0x0280), ('#',  0xAA3C),
            ('$',  0xAABB), ('%',  0xEE99), ('&',  0x9379), ('\'', 0x0080),
            ('(',  0x1400), (')',  0x4100), ('*',  0xDD00), ('+',  0xAA00),
            (',',  0xC000), ('-',  0x8800), ('.',  0x0020), ('/',  0x4400),
            ('0',  0x44FF), ('1',  0x040C), ('2',  0x8877), ('3',  0x883F),
            ('4',  0x888C), ('5',  0x88BB), ('6',  0x88FB), ('7',  0x2483),
            ('8',  0x88FF), ('9',  0x88BF), (':',  0x8020), (';',  0xC001),
            ('<',  0x9400), ('=',  0x8830), ('>',  0x4900), ('?',  0x2887),
            ('@',  0x28DF), ('A',  0x88CF), ('B',  0x2A3F), ('C',  0x00F3),
            ('D',  0x223F), ('E',  0x80F3), ('F',  0x80C3), ('G',  0x08FB),
            ('H',  0x88CC), ('I',  0x2233), ('J',  0x007E), ('K',  0x94C0),
            ('L',  0x00F0), ('M',  0x05CC), ('N',  0x11CC), ('O',  0x00FF),
            ('P',  0x88C7), ('Q',  0x10FF), ('R',  0x98C7), ('S',  0x093B),
            ('T',  0x2203), ('U',  0x00FC), ('V',  0x44C0), ('W',  0x50CC),
            ('X',  0x5500), ('Y',  0x2500), ('Z',  0x4433), ('[',  0x2212),
            ('\\', 0x1100), (']',  0x2221), ('^',  0x0404), ('_',  0x0030),
            ('`',  0x0100), ('a',  0xA070), ('b',  0xA0E0), ('c',  0x8060),
            ('d',  0xA260), ('e',  0xC060), ('f',  0xAA02), ('g',  0x1818),
            ('h',  0xA0C0), ('i',  0x0040), ('j',  0x2220), ('k',  0x3A00),
            ('l',  0x00E0), ('m',  0xA848), ('n',  0xA040), ('o',  0xA060),
            ('p',  0x82C1), ('q',  0xA281), ('r',  0x8040), ('s',  0x1810),
            ('t',  0xAA10), ('u',  0x2060), ('v',  0x4040), ('w',  0x5048),
            ('x',  0xD800), ('y',  0x1018), ('z',  0xC020), ('{',  0xA212),
            ('|',  0x2200), ('}',  0x2A21), ('~',  0x0A85),
            // Latin-1 Supplement
            ('??',  0x8060), ('??',  0xA276), ('??',  0xAD00), ('??',  0x11BB),
            ('??',  0x8808), ('??',  0x8281), ('??',  0xAA30), ('??',  0x0C02),
            ('??',  0x0C06), ('??',  0x82C0), ('??',  0xA28F), ('??',  0x5500),
            ('??',  0x0CDB),
            // Greek and Coptic
            ('??',  0x88CF), ('??',  0x8CFB), ('??',  0x00C3), ('??',  0x443C),
            ('??',  0x88F3), ('??',  0x4433), ('??',  0x88CC), ('??',  0x80FF),
            ('??',  0x2233), ('??',  0x94C0), ('??',  0x440C), ('??',  0x05CC),
            ('??',  0x11CC), ('??',  0x8833), ('??',  0x00FF), ('??',  0x00CF),
            ('??',  0x88C7), ('??',  0x4133), ('??',  0x2203), ('??',  0x2500),
            ('??',  0xAA87), ('??',  0x5500), ('??',  0xAA84), ('??',  0xD887),
            ('??',  0xB070), ('??',  0x0CDB), ('??',  0xC280), ('??',  0xA161),
            ('??',  0x80E1), ('??',  0x8161), ('??',  0xA040), ('??',  0xA2E1),
            ('??',  0x0060), ('??',  0x3600), ('??',  0x5100), ('??',  0x82C0),
            ('??',  0x4040), ('??',  0x80F1), ('??',  0xA060), ('??',  0xD800),
            ('??',  0x82C1), ('??',  0x8060), ('??',  0xA860), ('??',  0xA800),
            ('??',  0x2060), ('??',  0xAA86), ('??',  0x5500), ('??',  0xAA84),
            ('??',  0x2078),
            // Currency Symbols
            ('???',  0xEA12),
            // Enclosed Alphanumerics
            ('???',  0x000C), ('???',  0x2816), ('???',  0x081E), ('???',  0x0A0C),
            ('???',  0x0A1A), ('???',  0x2A1A), ('???',  0x020E), ('???',  0x2A1E),
            ('???',  0x0A1E), ('???',  0x22DE), ('???',  0x00CC), ('???',  0x28D6),
            ('???',  0x08DE), ('???',  0x0ACC), ('???',  0x0ADA), ('???',  0x2ADA),
            ('???',  0x02CE), ('???',  0x2ADE), ('???',  0x0ADE), ('???',  0x000C),
            ('???',  0x2816), ('???',  0x081E), ('???',  0x0A0C), ('???',  0x0A1A),
            ('???',  0x2A1A), ('???',  0x020E), ('???',  0x2A1E), ('???',  0x0A1E),
            ('???',  0x22DE), ('???',  0x00CC), ('???',  0x28D6), ('???',  0x08DE),
            ('???',  0x0ACC), ('???',  0x0ADA), ('???',  0x2ADA), ('???',  0x02CE),
            ('???',  0x2ADE), ('???',  0x0ADE), ('???',  0x000C), ('???',  0x2816),
            ('???',  0x081E), ('???',  0x0A0C), ('???',  0x0A1A), ('???',  0x2A1A),
            ('???',  0x020E), ('???',  0x2A1E), ('???',  0x0A1E), ('???',  0x22DE),
            ('???',  0x00CC), ('???',  0x28D6), ('???',  0x08DE), ('???',  0x0ACC),
            ('???',  0x0ADA), ('???',  0x2ADA), ('???',  0x02CE), ('???',  0x2ADE),
            ('???',  0x0ADE), ('???',  0x00CC), ('???',  0x28D6), ('???',  0x08DE),
            ('???',  0x0ACC), ('???',  0x0ADA), ('???',  0x2ADA), ('???',  0x02CE),
            ('???',  0x2ADE), ('???',  0x0ADE), ('???',  0x000C), ('???',  0x2816),
            ('???',  0x081E), ('???',  0x0A0C), ('???',  0x0A1A), ('???',  0x2A1A),
            ('???',  0x020E), ('???',  0x2A1E), ('???',  0x0A1E), ('???',  0x22DE),
            ('???',  0x221E),
            // Dingbats
            ('???',  0x4440), ('???',  0x000C), ('???',  0x2816), ('???',  0x081E),
            ('???',  0x0A0C), ('???',  0x0A1A), ('???',  0x2A1A), ('???',  0x020E),
            ('???',  0x2A1E), ('???',  0x0A1E), ('???',  0x22DE), ('???',  0x000C),
            ('???',  0x2816), ('???',  0x081E), ('???',  0x0A0C), ('???',  0x0A1A),
            ('???',  0x2A1A), ('???',  0x020E), ('???',  0x2A1E), ('???',  0x0A1E),
            ('???',  0x22DE), ('???',  0x000C), ('???',  0x2816), ('???',  0x081E),
            ('???',  0x0A0C), ('???',  0x0A1A), ('???',  0x2A1A), ('???',  0x020E),
            ('???',  0x2A1E), ('???',  0x0A1E), ('???',  0x22DE),
            // CJK Symbols and Punctuation
            ('???',  0xA060), ('???',  0x5540), ('???',  0x00FF), ('???',  0x1400),
            ('???',  0x4100), ('???',  0x00C3), ('???',  0x003C), ('???',  0x28E7),
            ('???',  0x827E), ('???',  0xA803), ('???',  0x2212), ('???',  0x2221),
            ('???',  0x14F3), ('???',  0x413F), ('???',  0x14F3), ('???',  0x413F),
            ('???',  0x22F3), ('???',  0x223F), ('???',  0xA058), ('???',  0x55FF),
            // CJK Unified Ideographs
            ('???',  0x8800), ('???',  0xAA10), ('???',  0x4A0B), ('???',  0x8833),
            ('???',  0x2A30), ('???',  0x3203), ('???',  0xAA87), ('???',  0xAA33),
            ('???',  0xCA08), ('???',  0x0033), ('???',  0xAA3B), ('???',  0x5200),
            ('???',  0xD803), ('???',  0x5200), ('???',  0x420C), ('???',  0xDA00),
            ('???',  0x8ADF), ('???',  0xCA18), ('???',  0xAA00), ('???',  0xAA03),
            ('???',  0x00FF), ('???',  0x8AFF), ('???',  0xAA30), ('???',  0xDA00),
            ('???',  0xDA03), ('???',  0xAC03), ('???',  0x22FC), ('???',  0x22CC),
            ('???',  0x2233), ('???',  0xAA5B), ('???',  0x88FF), ('???',  0xFA00),
            ('???',  0x2A73), ('???',  0xAAFF), ('???',  0x88FF), ('???',  0xF803),
            ('???',  0xFF00), ('???',  0x5287),
            // Halfwidth and Fullwidth Forms
            ('???',  0x2200), ('???',  0x0280), ('???',  0xAA3C), ('???',  0xAABB),
            ('???',  0xEE99), ('???',  0x9379), ('???',  0x0080), ('???',  0x1400),
            ('???',  0x4100), ('???',  0xDD00), ('???',  0xAA00), ('???',  0xC000),
            ('???',  0x8800), ('???',  0x0020), ('???',  0x4400), ('???',  0x44FF),
            ('???',  0x040C), ('???',  0x8877), ('???',  0x883F), ('???',  0x888C),
            ('???',  0x88BB), ('???',  0x88FB), ('???',  0x2483), ('???',  0x88FF),
            ('???',  0x88BF), ('???',  0x8020), ('???',  0xC001), ('???',  0x9400),
            ('???',  0x8830), ('???',  0x4900), ('???',  0x2887), ('???',  0x28DF),
            ('???',  0x88CF), ('???',  0x2A3F), ('???',  0x00F3), ('???',  0x223F),
            ('???',  0x80F3), ('???',  0x80C3), ('???',  0x08FB), ('???',  0x88CC),
            ('???',  0x2233), ('???',  0x007E), ('???',  0x94C0), ('???',  0x00F0),
            ('???',  0x05CC), ('???',  0x11CC), ('???',  0x00FF), ('???',  0x88C7),
            ('???',  0x10FF), ('???',  0x98C7), ('???',  0x093B), ('???',  0x2203),
            ('???',  0x00FC), ('???',  0x44C0), ('???',  0x50CC), ('???',  0x5500),
            ('???',  0x2500), ('???',  0x4433), ('???',  0x2212), ('???',  0x1100),
            ('???',  0x2221), ('???',  0x0404), ('???',  0x0030), ('???',  0x0100),
            ('???',  0xA070), ('???',  0xA0E0), ('???',  0x8060), ('???',  0xA260),
            ('???',  0xC060), ('???',  0xAA02), ('???',  0x1818), ('???',  0xA0C0),
            ('???',  0x0040), ('???',  0x2220), ('???',  0x3A00), ('???',  0x00E0),
            ('???',  0xA848), ('???',  0xA040), ('???',  0xA060), ('???',  0x82C1),
            ('???',  0xA281), ('???',  0x8040), ('???',  0x1810), ('???',  0xAA10),
            ('???',  0x2060), ('???',  0x4040), ('???',  0x5048), ('???',  0xD800),
            ('???',  0x1018), ('???',  0xC020), ('???',  0xA212), ('???',  0x2200),
            ('???',  0x2A21), ('???',  0x0A85), ('???',  0xA060), ('???',  0x00C3),
            ('???',  0x003C), ('???',  0x8060), ('???',  0xA276), ('???',  0x2200),
            ('???',  0xAD00),
            // Enclosed Alphanumeric Supplement
            ('????',  0x221E), ('????',  0x221E), ('????',  0x000C), ('????',  0x2816),
            ('????',  0x081E), ('????',  0x0A0C), ('????',  0x0A1A), ('????',  0x2A1A),
            ('????',  0x020E), ('????',  0x2A1E), ('????',  0x0A1E), ('????',  0x221E),
            ('????',  0x221E),
            // Miscellaneous Symbols and Pictographs
            ('????',  0xDA00), ('????',  0xDA00), ('????',  0xFF55), ('????',  0x8AFC),
            ('????',  0xC887), ('????',  0x05FC), ('????',  0xAC38), ('????',  0x9A07),
            ('????',  0xA160), ('????',  0xCC22), ('????',  0xFF00), ('????',  0x51F0),
            ('????',  0x988F), ('????',  0x0030), ('????',  0x00FF), ('????',  0xAA66),
            ('????',  0x5500), ('????',  0x2A4E), ('????',  0xA2C9), ('????',  0x0700),
            ('????',  0x7000), ('????',  0xC100), ('????',  0x1C00), ('????',  0x7200),
            // Emoticons
            ('????',  0x0578), ('????',  0x04B8), ('????',  0x0578), ('????',  0x04B8),
            ('????',  0x00B4), ('????',  0x00B4), ('????',  0x00B4), ('????',  0x5084),
            ('????',  0x5084), ('????',  0x5084), ('????',  0x5084), ('????',  0x50B4),
            ('????',  0x50B4), ('????',  0x50B4), ('????',  0x50B4), ('????',  0x8AB7),
            ('????',  0x5084),
            // Supplemental Symbols and Pictographs
            ('????',  0x0578), ('????',  0x50B4), ('????',  0x7230),
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
