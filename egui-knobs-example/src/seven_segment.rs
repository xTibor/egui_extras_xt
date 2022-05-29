use eframe::egui::{Response, Sense, Ui};
use eframe::emath::{pos2, vec2, Pos2, Rect};
use eframe::epaint::{Color32, Shape, Stroke};

pub enum SevenSegmentPreset {
    DeLoreanRed,
    DeLoreanGreen,
    DeLoreanAmber,
}

const SEVEN_SEGMENT_FONT: [u8; 128] = [
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 00-07: × × × × × × × ×
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 08-0F: × × × × × × × ×
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 10-17: × × × × × × × ×
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 18-1F: × × × × × × × ×
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 20-27: × × × × × × × ×
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 28-2F: × × × × × × × ×
    0x3F, 0x06, 0x5B, 0x4F, 0x66, 0x6D, 0x7D, 0x27, // 30-37: 0 1 2 3 4 5 6 7
    0x7F, 0x6F, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 38-3F: 8 9 × × × × × ×
    0x00, 0x77, 0x7C, 0x39, 0x5E, 0x79, 0x71, 0x00, // 40-47: × A B C D E F ×
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 48-4F: × × × × × × × ×
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 50-57: × × × × × × × ×
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 58-5F: × × × × × × × ×
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 60-67: × × × × × × × ×
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 68-6F: × × × × × × × ×
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 70-77: × × × × × × × ×
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 78-7F: × × × × × × × ×
];

pub fn seven_segment(
    ui: &mut Ui,
    display_string: &str,
    digit_count: usize,
    segment_thickness: f32,
    segment_spacing: f32,
    digit_slant: f32,
    digit_height: f32,
    digit_ratio: f32,
) -> Response {
    let digit_width = digit_height * digit_ratio;

    let desired_size = vec2(
        digit_width * digit_count as f32 + 2.0 * digit_slant,
        digit_height,
    );

    let (rect, mut response) = ui.allocate_exact_size(desired_size, Sense::hover());

    ui.painter()
        .rect(rect, 0.0, Color32::BLACK, Stroke::new(2.0, Color32::BLUE));

    let paint_digit = |digit_bits, digit_center: Pos2| {
        let p = |dx, dy| {
            digit_center + vec2(dx, dy) - vec2((dy / (digit_height / 2.0)) * digit_slant, 0.0)
        };

        #[rustfmt::skip]
        #[allow(unused_parens)]
        let segment_points: [Vec<Pos2>; 7] = [
            vec![
                p(-(digit_width / 2.0) + (segment_thickness / 4.0) + segment_spacing, -(digit_height / 2.0) + (segment_thickness / 4.0)                  ),
                p(-(digit_width / 2.0) + (segment_thickness / 2.0) + segment_spacing, -(digit_height / 2.0)                                              ),
                p( (digit_width / 2.0) - (segment_thickness / 2.0) - segment_spacing, -(digit_height / 2.0)                                              ),
                p( (digit_width / 2.0) - (segment_thickness / 4.0) - segment_spacing, -(digit_height / 2.0) + (segment_thickness / 4.0)                  ),
                p( (digit_width / 2.0) - (segment_thickness / 1.0) - segment_spacing, -(digit_height / 2.0) + (segment_thickness / 1.0)                  ),
                p(-(digit_width / 2.0) + (segment_thickness / 1.0) + segment_spacing, -(digit_height / 2.0) + (segment_thickness / 1.0)                  ),
            ],
            vec![
                p( (digit_width / 2.0) - (segment_thickness / 1.0)                  , -(digit_height / 2.0) + (segment_thickness / 1.0) + segment_spacing),
                p( (digit_width / 2.0) - (segment_thickness / 4.0)                  , -(digit_height / 2.0) + (segment_thickness / 4.0) + segment_spacing),
                p( (digit_width / 2.0)                                              , -(digit_height / 2.0) + (segment_thickness / 2.0) + segment_spacing),
                p( (digit_width / 2.0)                                              ,                       - (segment_thickness / 2.0) - segment_spacing),
                p( (digit_width / 2.0) - (segment_thickness / 2.0)                  ,                                                   - segment_spacing),
                p( (digit_width / 2.0) - (segment_thickness / 1.0)                  ,                       - (segment_thickness / 2.0) - segment_spacing),
            ],
            vec![
                p( (digit_width / 2.0) - (segment_thickness / 1.0)                  ,  (digit_height / 2.0) - (segment_thickness / 1.0) - segment_spacing),
                p( (digit_width / 2.0) - (segment_thickness / 4.0)                  ,  (digit_height / 2.0) - (segment_thickness / 4.0) - segment_spacing),
                p( (digit_width / 2.0)                                              ,  (digit_height / 2.0) - (segment_thickness / 2.0) - segment_spacing),
                p( (digit_width / 2.0)                                              ,                         (segment_thickness / 2.0) + segment_spacing),
                p( (digit_width / 2.0) - (segment_thickness / 2.0)                  ,                                                     segment_spacing),
                p( (digit_width / 2.0) - (segment_thickness / 1.0)                  ,                         (segment_thickness / 2.0) + segment_spacing),
            ],
            vec![
                p(-(digit_width / 2.0) + (segment_thickness / 4.0) + segment_spacing,  (digit_height / 2.0) - (segment_thickness / 4.0)                  ),
                p(-(digit_width / 2.0) + (segment_thickness / 2.0) + segment_spacing,  (digit_height / 2.0)                                              ),
                p( (digit_width / 2.0) - (segment_thickness / 2.0) - segment_spacing,  (digit_height / 2.0)                                              ),
                p( (digit_width / 2.0) - (segment_thickness / 4.0) - segment_spacing,  (digit_height / 2.0) - (segment_thickness / 4.0)                  ),
                p( (digit_width / 2.0) - (segment_thickness / 1.0) - segment_spacing,  (digit_height / 2.0) - (segment_thickness / 1.0)                  ),
                p(-(digit_width / 2.0) + (segment_thickness / 1.0) + segment_spacing,  (digit_height / 2.0) - (segment_thickness / 1.0)                  ),
            ],
            vec![
                p(-(digit_width / 2.0) + (segment_thickness / 1.0)                  ,  (digit_height / 2.0) - (segment_thickness / 1.0) - segment_spacing),
                p(-(digit_width / 2.0) + (segment_thickness / 4.0)                  ,  (digit_height / 2.0) - (segment_thickness / 4.0) - segment_spacing),
                p(-(digit_width / 2.0)                                              ,  (digit_height / 2.0) - (segment_thickness / 2.0) - segment_spacing),
                p(-(digit_width / 2.0)                                              ,                         (segment_thickness / 2.0) + segment_spacing),
                p(-(digit_width / 2.0) + (segment_thickness / 2.0)                  ,                                                     segment_spacing),
                p(-(digit_width / 2.0) + (segment_thickness / 1.0)                  ,                         (segment_thickness / 2.0) + segment_spacing),
            ],
            vec![
                p(-(digit_width / 2.0) + (segment_thickness / 1.0)                  , -(digit_height / 2.0) + (segment_thickness / 1.0) + segment_spacing),
                p(-(digit_width / 2.0) + (segment_thickness / 4.0)                  , -(digit_height / 2.0) + (segment_thickness / 4.0) + segment_spacing),
                p(-(digit_width / 2.0)                                              , -(digit_height / 2.0) + (segment_thickness / 2.0) + segment_spacing),
                p(-(digit_width / 2.0)                                              ,                       - (segment_thickness / 2.0) - segment_spacing),
                p(-(digit_width / 2.0) + (segment_thickness / 2.0)                  ,                                                   - segment_spacing),
                p(-(digit_width / 2.0) + (segment_thickness / 1.0)                  ,                       - (segment_thickness / 2.0) - segment_spacing),
            ],
            vec![
                p(-(digit_width / 2.0) + (segment_thickness / 2.0) + segment_spacing,                                                                 0.0),
                p(-(digit_width / 2.0) + (segment_thickness / 1.0) + segment_spacing,                       - (segment_thickness / 2.0)                  ),
                p( (digit_width / 2.0) - (segment_thickness / 1.0) - segment_spacing,                       - (segment_thickness / 2.0)                  ),
                p( (digit_width / 2.0) - (segment_thickness / 2.0) - segment_spacing,                                                                 0.0),
                p( (digit_width / 2.0) - (segment_thickness / 1.0) - segment_spacing,                         (segment_thickness / 2.0)                  ),
                p(-(digit_width / 2.0) + (segment_thickness / 1.0) + segment_spacing,                         (segment_thickness / 2.0)                  ),
            ],
        ];

        for (segment_index, segment_points) in segment_points.iter().enumerate() {
            let segment_on = ((digit_bits >> segment_index) & 0x01) != 0x00;

            ui.painter().add(Shape::convex_polygon(
                segment_points.to_vec(),
                if segment_on {
                    Color32::GREEN
                } else {
                    Color32::DARK_GREEN
                },
                Stroke::new(2.0, Color32::DARK_GREEN),
            ));
        }
    };

    for (digit_index, digit_char) in display_string.chars().enumerate() {
        let digit_bits = if digit_char.is_ascii() {
            SEVEN_SEGMENT_FONT[digit_char as usize]
        } else {
            0x00
        };

        let digit_center = rect.left_center()
            + vec2(
                digit_slant + (digit_width * digit_index as f32) + (digit_width / 2.0),
                0.0,
            );

        paint_digit(digit_bits, digit_center);
    }

    response
}
