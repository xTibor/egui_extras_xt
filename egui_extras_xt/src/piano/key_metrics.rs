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

pub struct PianoKeyLogicalBounds {
    pub top: isize,
    pub bottom: isize,
    pub left: isize,
    pub right: isize,
}

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
        bounds: PianoKeyLogicalBounds {
            top: 0, bottom: 300, left: 0, right: 48,
        },
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
        bounds: PianoKeyLogicalBounds {
            top: 0, bottom: 189, left: 30, right: 58,
        },
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
        bounds: PianoKeyLogicalBounds {
            top: 0, bottom: 300, left: 48, right: 96,
        },
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
        bounds: PianoKeyLogicalBounds {
            top: 0, bottom: 189, left: 86, right: 114,
        },
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
        bounds: PianoKeyLogicalBounds {
            top: 0, bottom: 300, left: 96, right: 144,
        },
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
        bounds: PianoKeyLogicalBounds {
            top: 0, bottom: 300, left: 144, right: 192,
        },
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
        bounds: PianoKeyLogicalBounds {
            top: 0, bottom: 189, left: 170, right: 198,
        },
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
        bounds: PianoKeyLogicalBounds {
            top: 0, bottom: 300, left: 192, right: 240,
        },
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
        bounds: PianoKeyLogicalBounds {
            top: 0, bottom: 189, left: 226, right: 254,
        },
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
        bounds: PianoKeyLogicalBounds {
            top: 0, bottom: 300, left: 240, right: 288,
        },
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
        bounds: PianoKeyLogicalBounds {
            top: 0, bottom: 189, left: 282, right: 310,
        },
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
        bounds: PianoKeyLogicalBounds {
            top: 0, bottom: 300, left: 288, right: 336,
        },
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
