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
            ('¢',  0x8060),
            ('£',  0xA276),
            ('¥',  0xAD00),
            ('§',  0x11BB),
            ('±',  0xAA30),
            ('²',  0x0C02),
            ('³',  0x0C06),
            ('µ',  0x82C0),
            ('¶',  0xA28F),
            // Greek and Coptic
            ('Α',  0x88CF), ('Β',  0x8CFB), ('Γ',  0x00C3), ('Δ',  0x443C),
            ('Ε',  0x88F3), ('Ζ',  0x4433), ('Η',  0x88CC), ('Θ',  0x08FF),
            ('Ι',  0x2233), ('Κ',  0x94C0), ('Λ',  0x440C), ('Μ',  0x05CC),
            ('Ν',  0x11CC), ('Ξ',  0x8833), ('Ο',  0x00FF), ('Π',  0x00CF),
            ('Ρ',  0x88C7), ('Σ',  0x4133), ('Τ',  0x2203), ('Υ',  0x2500),
            ('Φ',  0xAA87), ('Χ',  0x5500), ('Ψ',  0xAA84), ('Ω',  0xD887),
            ('α',  0xB070), ('β',  0x0000), ('γ',  0xC280), ('δ',  0xA0E1),
            ('ε',  0x80E1), ('ζ',  0x0000), ('η',  0xA040), ('θ',  0xA2E1),
            ('ι',  0x0040), ('κ',  0x3600), ('λ',  0x5100), ('μ',  0x82C0),
            ('ν',  0x4040), ('ξ',  0x0000), ('ο',  0xA060), ('π',  0xD800),
            ('ρ',  0x82C1), ('ς',  0x8060), ('σ',  0xA860), ('τ',  0xA800),
            ('υ',  0x2060), ('φ',  0xAA86), ('χ',  0x5500), ('ψ',  0xAA84),
            ('ω',  0x2078),
            // Currency Symbols
            ('€',  0xEA12),
            // Dingbats
            ('✔',  0x4440),
            // CJK Symbols and Punctuation
            ('。',  0xA060),
            ('〆',  0x5540),
            ('〇',  0x00FF),
            ('〈',  0x1400),
            ('〉',  0x4100),
            ('「',  0x00C3),
            ('」',  0x003C),
            ('『',  0x28E7),
            ('』',  0x827E),
            ('〒',  0xA803),
            ('〔',  0x2212),
            ('〕',  0x2221),
            ('〖',  0x14F3),
            ('〗',  0x413F),
            ('〘',  0x14F3),
            ('〙',  0x413F),
            ('〚',  0x22F3),
            ('〛',  0x223F),
            ('〜',  0xA058),
            ('〿',  0x55FF),
            // CJK Unified Ideographs
            ('一',  0x8800),
            ('三',  0x8833),
            ('上',  0x2A30),
            ('下',  0x3203),
            ('中',  0xAA87),
            ('主',  0xAA33),
            ('九',  0xCA08),
            ('二',  0x0033),
            ('五',  0xAA3B),
            ('人',  0x5200),
            ('元',  0xD803),
            ('入',  0x5200),
            ('六',  0xDA00),
            ('円',  0x8ADF),
            ('力',  0xCA18),
            ('十',  0xAA00),
            ('口',  0x00FF),
            ('土',  0xAA30),
            ('大',  0xDA00),
            ('天',  0xDA03),
            ('子',  0xAC03),
            ('山',  0x22FC),
            ('川',  0x22CC),
            ('工',  0x2233),
            ('市',  0xAA5B),
            ('日',  0x88FF),
            ('木',  0xFA00),
            ('正',  0x2A73),
            ('田',  0xAAFF),
            ('示',  0xF803),
            ('米',  0xFF00),
            // Miscellaneous Symbols and Pictographs
            ('🍀',  0xFF55),
            ('🎂',  0x8AFC),
            ('🎈',  0xC887),
            ('🐱',  0x05FC),
            ('👍',  0xAC38),
            ('👎',  0x9A07),
            ('🔫',  0x988F),
            // Emoticons
            ('😂',  0x0578),
            ('😉',  0x04B8),
            ('😊',  0x0578),
            ('😏',  0x04B8),
            ('😐',  0x00B4),
            ('😑',  0x00B4),
            ('😒',  0x00B4),
            ('😕',  0x5084),
            ('😞',  0x5084),
            ('😢',  0x5084),
            ('😥',  0x5084),
            ('😩',  0x50B4),
            ('😫',  0x50B4),
            ('😭',  0x50B4),
            ('😲',  0x50B4),
            ('😳',  0x8AB7),
            ('🙁',  0x5084),
            // Supplemental Symbols and Pictographs
            ('🤣',  0x0578),
            ('🥵',  0x50B4),
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
