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
    digit_spacing: f32,
    margin_horizontal: f32,
    margin_vertical: f32,
) -> Response {
    let digit_width = digit_height * digit_ratio;

    // Turn relative metrics to absolute metrics
    let segment_thickness = segment_thickness * digit_height;
    let segment_spacing = segment_spacing * digit_height;
    let digit_slant = digit_slant * digit_width;
    let digit_spacing = digit_spacing * digit_width;
    let margin_horizontal = margin_horizontal * digit_width;
    let margin_vertical = margin_vertical * digit_height;

    let desired_size = vec2(
        (digit_width * digit_count as f32)
            + (digit_spacing * (digit_count - 1) as f32)
            + (2.0 * margin_horizontal)
            + (2.0 * digit_slant),
        digit_height + (2.0 * margin_vertical),
    );

    let (rect, mut response) = ui.allocate_exact_size(desired_size, Sense::hover());

    //ui.set_clip_rect(rect);

    ui.painter().rect(
        rect,
        0.0,
        Color32::from_rgb(0x0, 0x20, 0x00),
        Stroke::none(),
    );

    let paint_digit = |digit_bits: u8, digit_center: Pos2| {
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

            let (fill, stroke) = if segment_on {
                (
                    Color32::from_rgb(0x00, 0xF0, 0x00),
                    Stroke::new(2.0, Color32::from_rgb(0x00, 0xFF, 0x00)),
                )
            } else {
                (
                    Color32::from_rgb(0x00, 0x30, 0x00),
                    Stroke::new(2.0, Color32::from_rgb(0x00, 0x28, 0x00)),
                )
            };

            ui.painter()
                .add(Shape::convex_polygon(segment_points.to_vec(), fill, stroke));
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
                margin_horizontal
                    + digit_slant
                    + ((digit_width + digit_spacing) * digit_index as f32)
                    + (digit_width / 2.0),
                0.0,
            );

        paint_digit(digit_bits, digit_center);
    }

    response
}
