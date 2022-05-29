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
    0x00, 0x00, 0x22, 0x00, 0x00, 0x00, 0x00, 0x02, // 20-27:   × " × × × × '
    0x39, 0x0F, 0x00, 0x00, 0x0C, 0x40, 0x04, 0x52, // 28-2F: ( ) × × , - . /
    0x3F, 0x06, 0x5B, 0x4F, 0x66, 0x6D, 0x7D, 0x27, // 30-37: 0 1 2 3 4 5 6 7
    0x7F, 0x6F, 0x00, 0x00, 0x39, 0x48, 0x0F, 0x53, // 38-3F: 8 9 × × < = > ?
    0x7B, 0x77, 0x7C, 0x39, 0x5E, 0x79, 0x71, 0x3D, // 40-47: @ A B C D E F G
    0x76, 0x30, 0x1E, 0x76, 0x38, 0x00, 0x37, 0x3F, // 48-4F: H I J K L × N O
    0x73, 0x67, 0x77, 0x6D, 0x07, 0x3E, 0x3E, 0x00, // 50-57: P Q R S T U V ×
    0x76, 0x6E, 0x5B, 0x39, 0x64, 0x0F, 0x23, 0x08, // 58-5F: X Y Z [ \ ] ^ _
    0x20, 0x00, 0x7C, 0x58, 0x5E, 0x00, 0x71, 0x00, // 60-67: ` × b c d × f ×
    0x74, 0x10, 0x0E, 0x76, 0x06, 0x00, 0x54, 0x5C, // 68-6F: h i j k l × n o
    0x73, 0x67, 0x50, 0x6D, 0x78, 0x1C, 0x1C, 0x00, // 70-77: p q r s t u v ×
    0x00, 0x00, 0x00, 0x39, 0x30, 0x0F, 0x40, 0x00, // 78-7F: × × × { | } ~ ×
];

pub fn seven_segment(
    ui: &mut Ui,
    display_string: &str,
    digit_count: usize,
    segment_thickness: f32,
    segment_spacing: f32,
    digit_shearing: f32,
    digit_height: f32,
    digit_ratio: f32,
    digit_spacing: f32,
    digit_median: f32,
    margin_horizontal: f32,
    margin_vertical: f32,
) -> Response {
    let digit_width = digit_height * digit_ratio;

    // Turn relative metrics to absolute metrics
    let segment_thickness = segment_thickness * digit_height;
    let segment_spacing = segment_spacing * digit_height;
    let digit_shearing = digit_shearing * digit_width;
    let digit_spacing = digit_spacing * digit_width;
    let margin_horizontal = margin_horizontal * digit_width;
    let margin_vertical = margin_vertical * digit_height;
    let digit_median = digit_median * (digit_height / 2.0);

    let desired_size = vec2(
        (digit_width * digit_count as f32)
            + (digit_spacing * (digit_count - 1) as f32)
            + (2.0 * margin_horizontal)
            + (2.0 * digit_shearing),
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
            digit_center + vec2(dx, dy) - vec2((dy / (digit_height / 2.0)) * digit_shearing, 0.0)
        };

        #[rustfmt::skip]
        #[allow(unused_parens)]
        let segment_points: [Vec<Pos2>; 7] = [
            vec![
                p(-(digit_width / 2.0) + (segment_thickness / 4.0) + segment_spacing, -(digit_height / 2.0) + (segment_thickness / 4.0)                                 ),
                p(-(digit_width / 2.0) + (segment_thickness / 2.0) + segment_spacing, -(digit_height / 2.0)                                                             ),
                p( (digit_width / 2.0) - (segment_thickness / 2.0) - segment_spacing, -(digit_height / 2.0)                                                             ),
                p( (digit_width / 2.0) - (segment_thickness / 4.0) - segment_spacing, -(digit_height / 2.0) + (segment_thickness / 4.0)                                 ),
                p( (digit_width / 2.0) - (segment_thickness / 1.0) - segment_spacing, -(digit_height / 2.0) + (segment_thickness / 1.0)                                 ),
                p(-(digit_width / 2.0) + (segment_thickness / 1.0) + segment_spacing, -(digit_height / 2.0) + (segment_thickness / 1.0)                                 ),
            ],
            vec![
                p( (digit_width / 2.0) - (segment_thickness / 1.0)                  , -(digit_height / 2.0) + (segment_thickness / 1.0) + segment_spacing               ),
                p( (digit_width / 2.0) - (segment_thickness / 4.0)                  , -(digit_height / 2.0) + (segment_thickness / 4.0) + segment_spacing               ),
                p( (digit_width / 2.0)                                              , -(digit_height / 2.0) + (segment_thickness / 2.0) + segment_spacing               ),
                p( (digit_width / 2.0)                                              ,                       - (segment_thickness / 2.0) - segment_spacing + digit_median),
                p( (digit_width / 2.0) - (segment_thickness / 2.0)                  ,                                                   - segment_spacing + digit_median),
                p( (digit_width / 2.0) - (segment_thickness / 1.0)                  ,                       - (segment_thickness / 2.0) - segment_spacing + digit_median),
            ],
            vec![
                p( (digit_width / 2.0) - (segment_thickness / 1.0)                  ,  (digit_height / 2.0) - (segment_thickness / 1.0) - segment_spacing               ),
                p( (digit_width / 2.0) - (segment_thickness / 4.0)                  ,  (digit_height / 2.0) - (segment_thickness / 4.0) - segment_spacing               ),
                p( (digit_width / 2.0)                                              ,  (digit_height / 2.0) - (segment_thickness / 2.0) - segment_spacing               ),
                p( (digit_width / 2.0)                                              ,                         (segment_thickness / 2.0) + segment_spacing + digit_median),
                p( (digit_width / 2.0) - (segment_thickness / 2.0)                  ,                                                     segment_spacing + digit_median),
                p( (digit_width / 2.0) - (segment_thickness / 1.0)                  ,                         (segment_thickness / 2.0) + segment_spacing + digit_median),
            ],
            vec![
                p(-(digit_width / 2.0) + (segment_thickness / 4.0) + segment_spacing,  (digit_height / 2.0) - (segment_thickness / 4.0)                                 ),
                p(-(digit_width / 2.0) + (segment_thickness / 2.0) + segment_spacing,  (digit_height / 2.0)                                                             ),
                p( (digit_width / 2.0) - (segment_thickness / 2.0) - segment_spacing,  (digit_height / 2.0)                                                             ),
                p( (digit_width / 2.0) - (segment_thickness / 4.0) - segment_spacing,  (digit_height / 2.0) - (segment_thickness / 4.0)                                 ),
                p( (digit_width / 2.0) - (segment_thickness / 1.0) - segment_spacing,  (digit_height / 2.0) - (segment_thickness / 1.0)                                 ),
                p(-(digit_width / 2.0) + (segment_thickness / 1.0) + segment_spacing,  (digit_height / 2.0) - (segment_thickness / 1.0)                                 ),
            ],
            vec![
                p(-(digit_width / 2.0) + (segment_thickness / 1.0)                  ,  (digit_height / 2.0) - (segment_thickness / 1.0) - segment_spacing               ),
                p(-(digit_width / 2.0) + (segment_thickness / 4.0)                  ,  (digit_height / 2.0) - (segment_thickness / 4.0) - segment_spacing               ),
                p(-(digit_width / 2.0)                                              ,  (digit_height / 2.0) - (segment_thickness / 2.0) - segment_spacing               ),
                p(-(digit_width / 2.0)                                              ,                         (segment_thickness / 2.0) + segment_spacing + digit_median),
                p(-(digit_width / 2.0) + (segment_thickness / 2.0)                  ,                                                     segment_spacing + digit_median),
                p(-(digit_width / 2.0) + (segment_thickness / 1.0)                  ,                         (segment_thickness / 2.0) + segment_spacing + digit_median),
            ],
            vec![
                p(-(digit_width / 2.0) + (segment_thickness / 1.0)                  , -(digit_height / 2.0) + (segment_thickness / 1.0) + segment_spacing               ),
                p(-(digit_width / 2.0) + (segment_thickness / 4.0)                  , -(digit_height / 2.0) + (segment_thickness / 4.0) + segment_spacing               ),
                p(-(digit_width / 2.0)                                              , -(digit_height / 2.0) + (segment_thickness / 2.0) + segment_spacing               ),
                p(-(digit_width / 2.0)                                              ,                       - (segment_thickness / 2.0) - segment_spacing + digit_median),
                p(-(digit_width / 2.0) + (segment_thickness / 2.0)                  ,                                                   - segment_spacing + digit_median),
                p(-(digit_width / 2.0) + (segment_thickness / 1.0)                  ,                       - (segment_thickness / 2.0) - segment_spacing + digit_median),
            ],
            vec![
                p(-(digit_width / 2.0) + (segment_thickness / 2.0) + segment_spacing,                                                                       digit_median),
                p(-(digit_width / 2.0) + (segment_thickness / 1.0) + segment_spacing,                       - (segment_thickness / 2.0)                   + digit_median),
                p( (digit_width / 2.0) - (segment_thickness / 1.0) - segment_spacing,                       - (segment_thickness / 2.0)                   + digit_median),
                p( (digit_width / 2.0) - (segment_thickness / 2.0) - segment_spacing,                                                                       digit_median),
                p( (digit_width / 2.0) - (segment_thickness / 1.0) - segment_spacing,                         (segment_thickness / 2.0)                   + digit_median),
                p(-(digit_width / 2.0) + (segment_thickness / 1.0) + segment_spacing,                         (segment_thickness / 2.0)                   + digit_median),
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
                    + digit_shearing
                    + ((digit_width + digit_spacing) * digit_index as f32)
                    + (digit_width / 2.0),
                0.0,
            );

        paint_digit(digit_bits, digit_center);
    }

    response
}
