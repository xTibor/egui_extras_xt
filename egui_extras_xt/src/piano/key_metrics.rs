/*
    0    30    58    86    114   144  170   198   226   254   282   310  336
    | +30 | +28 | +28 | +28 | +30 |+26 | +28 | +28 | +28 | +28 | +28 |+26 |
  0-┌─────┬─────┬─────┬─────┬─────┬────┬─────┬─────┬─────┬─────┬─────┬────┐-0
    │     │     │     │     │     │    │     │     │     │     │     │    │
    │     │     │     │     │     │    │     │     │     │     │     │    │
    │     │     │     │     │     │    │     │     │     │     │     │    │
    │     │  ×  │     │  ×  │     │    │  ×  │     │  ×  │     │  ×  │    │
189-│     └───┬─┘     └─┬───┘     │    └────┬┘     └──┬──┘     └┬────┘    │-189
    │         │         │         │         │         │         │         │
    │    ×    │    ×    │    ×    │    ×    │    ×    │    ×    │    ×    │
    │         │         │         │         │         │         │         │
300-└─────────┴─────────┴─────────┴─────────┴─────────┴─────────┴─────────┘-300
    |   +48   |   +48   |   +48   |   +48   |   +48   |   +48   |   +48   |
    0        48        96        144       192       240       288       336
*/

pub const PIANO_OCTAVE_WIDTH: isize = 336;
pub const PIANO_OCTAVE_HEIGHT: isize = 300;

pub struct PianoKeyLogicalPos(pub isize, pub isize);

pub struct PianoKeyLogicalBounds(pub isize, pub isize, pub isize, pub isize); // (left, top, right, bottom)

#[derive(PartialEq)]
pub enum PianoKeyColor {
    White,
    Black,
}

pub struct PianoKeyMetrics {
    pub color: PianoKeyColor,
    pub z_index: isize,
    pub bounds: PianoKeyLogicalBounds,
    pub geometry_first: &'static [PianoKeyLogicalPos],
    pub geometry_middle: &'static [PianoKeyLogicalPos],
    pub geometry_last: &'static [PianoKeyLogicalPos],
}

#[rustfmt::skip]
pub const PIANO_KEY_METRICS: [PianoKeyMetrics; 12] = [
    PianoKeyMetrics {
        color: PianoKeyColor::White,
        z_index: 1,
        bounds: PianoKeyLogicalBounds(0, 0, 48, 300),
        geometry_first: &[
            PianoKeyLogicalPos(  0,   0),
            PianoKeyLogicalPos( 30,   0),
            PianoKeyLogicalPos( 30, 189),
            PianoKeyLogicalPos( 48, 189),
            PianoKeyLogicalPos( 48, 300),
            PianoKeyLogicalPos(  0, 300),
        ],
        geometry_middle: &[
            PianoKeyLogicalPos(  0,   0),
            PianoKeyLogicalPos( 30,   0),
            PianoKeyLogicalPos( 30, 189),
            PianoKeyLogicalPos( 48, 189),
            PianoKeyLogicalPos( 48, 300),
            PianoKeyLogicalPos(  0, 300),
        ],
        geometry_last: &[
            PianoKeyLogicalPos(  0,   0),
            PianoKeyLogicalPos( 48,   0),
            PianoKeyLogicalPos( 48, 300),
            PianoKeyLogicalPos(  0, 300),
        ],
    },
    PianoKeyMetrics {
        color: PianoKeyColor::Black,
        z_index: 0,
        bounds: PianoKeyLogicalBounds(30, 0, 58, 189),
        geometry_first: &[
            PianoKeyLogicalPos( 30,   0),
            PianoKeyLogicalPos( 58,   0),
            PianoKeyLogicalPos( 58, 189),
            PianoKeyLogicalPos( 30, 189),
        ],
        geometry_middle: &[
            PianoKeyLogicalPos( 30,   0),
            PianoKeyLogicalPos( 58,   0),
            PianoKeyLogicalPos( 58, 189),
            PianoKeyLogicalPos( 30, 189),
        ],
        geometry_last: &[
            PianoKeyLogicalPos( 30,   0),
            PianoKeyLogicalPos( 58,   0),
            PianoKeyLogicalPos( 58, 189),
            PianoKeyLogicalPos( 30, 189),
        ],
    },
    PianoKeyMetrics {
        color: PianoKeyColor::White,
        z_index: 1,
        bounds: PianoKeyLogicalBounds(48, 0, 96, 300),
        geometry_first: &[
            PianoKeyLogicalPos( 48,   0),
            PianoKeyLogicalPos( 86,   0),
            PianoKeyLogicalPos( 86, 189),
            PianoKeyLogicalPos( 96, 189),
            PianoKeyLogicalPos( 96, 300),
            PianoKeyLogicalPos( 48, 300),
        ],
        geometry_middle: &[
            PianoKeyLogicalPos( 58,   0),
            PianoKeyLogicalPos( 86,   0),
            PianoKeyLogicalPos( 86, 189),
            PianoKeyLogicalPos( 96, 189),
            PianoKeyLogicalPos( 96, 300),
            PianoKeyLogicalPos( 48, 300),
            PianoKeyLogicalPos( 48, 189),
            PianoKeyLogicalPos( 58, 189),
        ],
        geometry_last: &[
            PianoKeyLogicalPos( 58,   0),
            PianoKeyLogicalPos( 96,   0),
            PianoKeyLogicalPos( 96, 300),
            PianoKeyLogicalPos( 48, 300),
            PianoKeyLogicalPos( 48, 189),
            PianoKeyLogicalPos( 58, 189),
        ],
    },
    PianoKeyMetrics {
        color: PianoKeyColor::Black,
        z_index: 0,
        bounds: PianoKeyLogicalBounds(86, 0, 114, 189),
        geometry_first: &[
            PianoKeyLogicalPos( 86,   0),
            PianoKeyLogicalPos(114,   0),
            PianoKeyLogicalPos(114, 189),
            PianoKeyLogicalPos( 86, 189),
        ],
        geometry_middle: &[
            PianoKeyLogicalPos( 86,   0),
            PianoKeyLogicalPos(114,   0),
            PianoKeyLogicalPos(114, 189),
            PianoKeyLogicalPos( 86, 189),
        ],
        geometry_last: &[
            PianoKeyLogicalPos( 86,   0),
            PianoKeyLogicalPos(114,   0),
            PianoKeyLogicalPos(114, 189),
            PianoKeyLogicalPos( 86, 189),
        ],
    },
    PianoKeyMetrics {
        color: PianoKeyColor::White,
        z_index: 1,
        bounds: PianoKeyLogicalBounds(96, 0, 144, 300),
        geometry_first: &[
            PianoKeyLogicalPos( 96,   0),
            PianoKeyLogicalPos(144,   0),
            PianoKeyLogicalPos(144, 300),
            PianoKeyLogicalPos( 96, 300),
        ],
        geometry_middle: &[
            PianoKeyLogicalPos(114,   0),
            PianoKeyLogicalPos(144,   0),
            PianoKeyLogicalPos(144, 300),
            PianoKeyLogicalPos( 96, 300),
            PianoKeyLogicalPos( 96, 189),
            PianoKeyLogicalPos(114, 189),
        ],
        geometry_last: &[
            PianoKeyLogicalPos(114,   0),
            PianoKeyLogicalPos(144,   0),
            PianoKeyLogicalPos(144, 300),
            PianoKeyLogicalPos( 96, 300),
            PianoKeyLogicalPos( 96, 189),
            PianoKeyLogicalPos(114, 189),
        ],
    },
    PianoKeyMetrics {
        color: PianoKeyColor::White,
        z_index: 1,
        bounds: PianoKeyLogicalBounds(144, 0, 192, 300),
        geometry_first: &[
            PianoKeyLogicalPos(144,   0),
            PianoKeyLogicalPos(170,   0),
            PianoKeyLogicalPos(170, 189),
            PianoKeyLogicalPos(192, 189),
            PianoKeyLogicalPos(192, 300),
            PianoKeyLogicalPos(144, 300),
        ],
        geometry_middle: &[
            PianoKeyLogicalPos(144,   0),
            PianoKeyLogicalPos(170,   0),
            PianoKeyLogicalPos(170, 189),
            PianoKeyLogicalPos(192, 189),
            PianoKeyLogicalPos(192, 300),
            PianoKeyLogicalPos(144, 300),
        ],
        geometry_last: &[
            PianoKeyLogicalPos(144,   0),
            PianoKeyLogicalPos(192,   0),
            PianoKeyLogicalPos(192, 300),
            PianoKeyLogicalPos(144, 300),
        ],
    },
    PianoKeyMetrics {
        color: PianoKeyColor::Black,
        z_index: 0,
        bounds: PianoKeyLogicalBounds(170, 0, 198, 189),
        geometry_first: &[
            PianoKeyLogicalPos(170,   0),
            PianoKeyLogicalPos(198,   0),
            PianoKeyLogicalPos(198, 189),
            PianoKeyLogicalPos(170, 189),
        ],
        geometry_middle: &[
            PianoKeyLogicalPos(170,   0),
            PianoKeyLogicalPos(198,   0),
            PianoKeyLogicalPos(198, 189),
            PianoKeyLogicalPos(170, 189),
        ],
        geometry_last: &[
            PianoKeyLogicalPos(170,   0),
            PianoKeyLogicalPos(198,   0),
            PianoKeyLogicalPos(198, 189),
            PianoKeyLogicalPos(170, 189),
        ],
    },
    PianoKeyMetrics {
        color: PianoKeyColor::White,
        z_index: 1,
        bounds: PianoKeyLogicalBounds(192, 0, 240, 300),
        geometry_first: &[
            PianoKeyLogicalPos(192,   0),
            PianoKeyLogicalPos(226,   0),
            PianoKeyLogicalPos(226, 189),
            PianoKeyLogicalPos(240, 189),
            PianoKeyLogicalPos(240, 300),
            PianoKeyLogicalPos(192, 300),
        ],
        geometry_middle: &[
            PianoKeyLogicalPos(198,   0),
            PianoKeyLogicalPos(226,   0),
            PianoKeyLogicalPos(226, 189),
            PianoKeyLogicalPos(240, 189),
            PianoKeyLogicalPos(240, 300),
            PianoKeyLogicalPos(192, 300),
            PianoKeyLogicalPos(192, 189),
            PianoKeyLogicalPos(198, 189),
        ],
        geometry_last: &[
            PianoKeyLogicalPos(198,   0),
            PianoKeyLogicalPos(240,   0),
            PianoKeyLogicalPos(240, 300),
            PianoKeyLogicalPos(192, 300),
            PianoKeyLogicalPos(192, 189),
            PianoKeyLogicalPos(198, 189),
        ],
    },
    PianoKeyMetrics {
        color: PianoKeyColor::Black,
        z_index: 0,
        bounds: PianoKeyLogicalBounds(226, 0, 254, 189),
        geometry_first: &[
            PianoKeyLogicalPos(226,   0),
            PianoKeyLogicalPos(254,   0),
            PianoKeyLogicalPos(254, 189),
            PianoKeyLogicalPos(226, 189),
        ],
        geometry_middle: &[
            PianoKeyLogicalPos(226,   0),
            PianoKeyLogicalPos(254,   0),
            PianoKeyLogicalPos(254, 189),
            PianoKeyLogicalPos(226, 189),
        ],
        geometry_last: &[
            PianoKeyLogicalPos(226,   0),
            PianoKeyLogicalPos(254,   0),
            PianoKeyLogicalPos(254, 189),
            PianoKeyLogicalPos(226, 189),
        ],
    },
    PianoKeyMetrics {
        color: PianoKeyColor::White,
        z_index: 1,
        bounds: PianoKeyLogicalBounds(240, 0, 288, 300),
        geometry_first: &[
            PianoKeyLogicalPos(240,   0),
            PianoKeyLogicalPos(282,   0),
            PianoKeyLogicalPos(282, 189),
            PianoKeyLogicalPos(288, 189),
            PianoKeyLogicalPos(288, 300),
            PianoKeyLogicalPos(240, 300),
        ],
        geometry_middle: &[
            PianoKeyLogicalPos(254,   0),
            PianoKeyLogicalPos(282,   0),
            PianoKeyLogicalPos(282, 189),
            PianoKeyLogicalPos(288, 189),
            PianoKeyLogicalPos(288, 300),
            PianoKeyLogicalPos(240, 300),
            PianoKeyLogicalPos(240, 189),
            PianoKeyLogicalPos(254, 189),
        ],
        geometry_last: &[
            PianoKeyLogicalPos(254,   0),
            PianoKeyLogicalPos(288,   0),
            PianoKeyLogicalPos(288, 300),
            PianoKeyLogicalPos(240, 300),
            PianoKeyLogicalPos(240, 189),
            PianoKeyLogicalPos(254, 189),
        ],
    },
    PianoKeyMetrics {
        color: PianoKeyColor::Black,
        z_index: 0,
        bounds: PianoKeyLogicalBounds(282, 0, 310, 189),
        geometry_first: &[
            PianoKeyLogicalPos(282,   0),
            PianoKeyLogicalPos(310,   0),
            PianoKeyLogicalPos(310, 189),
            PianoKeyLogicalPos(282, 189),
        ],
        geometry_middle: &[
            PianoKeyLogicalPos(282,   0),
            PianoKeyLogicalPos(310,   0),
            PianoKeyLogicalPos(310, 189),
            PianoKeyLogicalPos(282, 189),
        ],
        geometry_last: &[
            PianoKeyLogicalPos(282,   0),
            PianoKeyLogicalPos(310,   0),
            PianoKeyLogicalPos(310, 189),
            PianoKeyLogicalPos(282, 189),
        ],
    },
    PianoKeyMetrics {
        color: PianoKeyColor::White,
        z_index: 1,
        bounds: PianoKeyLogicalBounds(288, 0, 336, 300),
        geometry_first: &[
            PianoKeyLogicalPos(288,   0),
            PianoKeyLogicalPos(336,   0),
            PianoKeyLogicalPos(336, 300),
            PianoKeyLogicalPos(288, 300),
        ],
        geometry_middle: &[
            PianoKeyLogicalPos(310,   0),
            PianoKeyLogicalPos(336,   0),
            PianoKeyLogicalPos(336, 300),
            PianoKeyLogicalPos(288, 300),
            PianoKeyLogicalPos(288, 189),
            PianoKeyLogicalPos(310, 189),
        ],
        geometry_last: &[
            PianoKeyLogicalPos(310,   0),
            PianoKeyLogicalPos(336,   0),
            PianoKeyLogicalPos(336, 300),
            PianoKeyLogicalPos(288, 300),
            PianoKeyLogicalPos(288, 189),
            PianoKeyLogicalPos(310, 189),
        ],
    },
];
