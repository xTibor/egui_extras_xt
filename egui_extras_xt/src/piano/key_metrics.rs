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

pub const KEY_OCTAVE_WIDTH: isize = 336; // BLACK_COUNT * WHITE_COUNT * EXTRA_RESOLUTION = 12 * 7 * 4
pub const KEY_OCTAVE_HEIGHT: isize = 300;

pub const KEY_WHITE_RATIO: f64 = 6.25;
pub const KEY_WHITE_WIDTH: isize = 48; // KEY_OCTAVE_WIDTH / 7
pub const KEY_WHITE_HEIGHT: isize = 300; // KEY_WHITE_WIDTH * KEY_WHITE_RATIO

pub const KEY_BLACK_RATIO: f64 = 6.75;
pub const KEY_BLACK_WIDTH: isize = 28; // KEY_OCTAVE_WIDTH / 12
pub const KEY_BLACK_HEIGHT: isize = 189; // KEY_BLACK_WIDTH * KEY_BLACK_RATIO

pub const KEY_LARGE_SPACING: isize = 30; // ((KEY_WHITE_WIDTH * 3) - (KEY_BLACK_WIDTH * 3)) / 2
pub const KEY_SMALL_SPACING: isize = 26; // ((KEY_WHITE_WIDTH * 4) - (KEY_BLACK_WIDTH * 5)) / 2

#[derive(PartialEq)]
pub enum KeyColor {
    White,
    Black,
}

pub struct KeyMetrics {
    pub color: KeyColor,
    pub z_index: isize,
    pub bounds: ((isize, isize), (isize, isize)),
    pub geometry_first: &'static [(isize, isize)],
    pub geometry_middle: &'static [(isize, isize)],
    pub geometry_last: &'static [(isize, isize)],
}

#[rustfmt::skip]
pub const KEY_METRICS: [KeyMetrics; 12] = [
    KeyMetrics {
        color: KeyColor::White,
        z_index: 1,
        bounds: (
            (  0,   0), ( 48, 300),
        ),
        geometry_first: &[
            (  0,   0), ( 30,   0),
            ( 30, 189), ( 48, 189),
            ( 48, 300), (  0, 300),
        ],
        geometry_middle: &[
            (  0,   0), ( 30,   0),
            ( 30, 189), ( 48, 189),
            ( 48, 300), (  0, 300),
        ],
        geometry_last: &[
            (  0,   0), ( 48,   0),
            ( 48, 300), (  0, 300),
        ],
    },
    KeyMetrics {
        color: KeyColor::Black,
        z_index: 0,
        bounds: (
            ( 30,   0), ( 58, 189),
        ),
        geometry_first: &[
            ( 30,   0), ( 58,   0),
            ( 58, 189), ( 30, 189),
        ],
        geometry_middle: &[
            ( 30,   0), ( 58,   0),
            ( 58, 189), ( 30, 189),
        ],
        geometry_last: &[
            ( 30,   0), ( 58,   0),
            ( 58, 189), ( 30, 189),
        ],
    },
    KeyMetrics {
        color: KeyColor::White,
        z_index: 1,
        bounds: (
            ( 48,   0), ( 96, 300),
        ),
        geometry_first: &[
            ( 48,   0), ( 86,   0),
            ( 86, 189), ( 96, 189),
            ( 96, 300), ( 48, 300),
        ],
        geometry_middle: &[
            ( 58,   0), ( 86,   0),
            ( 86, 189), ( 96, 189),
            ( 96, 300), ( 48, 300),
            ( 48, 189), ( 58, 189),
        ],
        geometry_last: &[
            ( 58,   0), ( 96,   0),
            ( 96, 300), ( 48, 300),
            ( 48, 189), ( 58, 189),
        ],
    },
    KeyMetrics {
        color: KeyColor::Black,
        z_index: 0,
        bounds: (
            ( 86,   0), (114, 189),
        ),
        geometry_first: &[
            ( 86,   0), (114,   0),
            (114, 189), ( 86, 189),
        ],
        geometry_middle: &[
            ( 86,   0), (114,   0),
            (114, 189), ( 86, 189),
        ],
        geometry_last: &[
            ( 86,   0), (114,   0),
            (114, 189), ( 86, 189),
        ],
    },
    KeyMetrics {
        color: KeyColor::White,
        z_index: 1,
        bounds: (
            ( 96,   0), (144, 300),
        ),
        geometry_first: &[
            ( 96,   0), (144,   0),
            (144, 300), ( 96, 300),
        ],
        geometry_middle: &[
            (114,   0), (144,   0),
            (144, 300), ( 96, 300),
            ( 96, 189), (114, 189),
        ],
        geometry_last: &[
            (114,   0), (144,   0),
            (144, 300), ( 96, 300),
            ( 96, 189), (114, 189),
        ],
    },
    KeyMetrics {
        color: KeyColor::White,
        z_index: 1,
        bounds: (
            (144,   0), (192, 300),
        ),
        geometry_first: &[
            (144,   0), (170,   0),
            (170, 189), (192, 189),
            (192, 300), (144, 300),
        ],
        geometry_middle: &[
            (144,   0), (170,   0),
            (170, 189), (192, 189),
            (192, 300), (144, 300),
        ],
        geometry_last: &[
            (144,   0), (192,   0),
            (192, 300), (144, 300),
        ],
    },
    KeyMetrics {
        color: KeyColor::Black,
        z_index: 0,
        bounds: (
            (170,   0), (198, 189),
        ),
        geometry_first: &[
            (170,   0), (198,   0),
            (198, 189), (170, 189),
        ],
        geometry_middle: &[
            (170,   0), (198,   0),
            (198, 189), (170, 189),
        ],
        geometry_last: &[
            (170,   0), (198,   0),
            (198, 189), (170, 189),
        ],
    },
    KeyMetrics {
        color: KeyColor::White,
        z_index: 1,
        bounds: (
            (192,   0), (240, 300),
        ),
        geometry_first: &[
            (192,   0), (226,   0),
            (226, 189), (240, 189),
            (240, 300), (192, 300),
        ],
        geometry_middle: &[
            (198,   0), (226,   0),
            (226, 189), (240, 189),
            (240, 300), (192, 300),
            (192, 189), (198, 189),
        ],
        geometry_last: &[
            (198,   0), (240,   0),
            (240, 300), (192, 300),
            (192, 189), (198, 189),
        ],
    },
    KeyMetrics {
        color: KeyColor::Black,
        z_index: 0,
        bounds: (
            (226,   0), (254, 189),
        ),
        geometry_first: &[
            (226,   0), (254,   0),
            (254, 189), (226, 189),
        ],
        geometry_middle: &[
            (226,   0), (254,   0),
            (254, 189), (226, 189),
        ],
        geometry_last: &[
            (226,   0), (254,   0),
            (254, 189), (226, 189),
        ],
    },
    KeyMetrics {
        color: KeyColor::White,
        z_index: 1,
        bounds: (
            (240,   0), (288, 300),
        ),
        geometry_first: &[
            (240,   0), (282,   0),
            (282, 189), (288, 189),
            (288, 300), (240, 300),
        ],
        geometry_middle: &[
            (254,   0), (282,   0),
            (282, 189), (288, 189),
            (288, 300), (240, 300),
            (240, 189), (254, 189),
        ],
        geometry_last: &[
            (254,   0), (288,   0),
            (288, 300), (240, 300),
            (240, 189), (254, 189),
        ],
    },
    KeyMetrics {
        color: KeyColor::Black,
        z_index: 0,
        bounds: (
            (282,   0), (310, 189),
        ),
        geometry_first: &[
            (282,   0), (310,   0),
            (310, 189), (282, 189),
        ],
        geometry_middle: &[
            (282,   0), (310,   0),
            (310, 189), (282, 189),
        ],
        geometry_last: &[
            (282,   0), (310,   0),
            (310, 189), (282, 189),
        ],
    },
    KeyMetrics {
        color: KeyColor::White,
        z_index: 1,
        bounds: (
            (288,   0), (336, 300),
        ),
        geometry_first: &[
            (288,   0), (336,   0),
            (336, 300), (288, 300),
        ],
        geometry_middle: &[
            (310,   0), (336,   0),
            (336, 300), (288, 300),
            (288, 189), (310, 189),
        ],
        geometry_last: &[
            (310,   0), (336,   0),
            (336, 300), (288, 300),
            (288, 189), (310, 189),
        ],
    },
];
