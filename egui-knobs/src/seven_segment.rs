use egui::{vec2, Color32, Pos2, Response, Sense, Shape, Stroke, Ui, Widget};

// ----------------------------------------------------------------------------

pub type SevenSegmentFont = [u8; 128];

pub const DEFAULT_FONT: SevenSegmentFont = [
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 00-07: × × × × × × × ×
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 08-0F: × × × × × × × ×
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 10-17: × × × × × × × ×
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 18-1F: × × × × × × × ×
    0x00, 0x00, 0x22, 0x00, 0x00, 0x00, 0x00, 0x02, // 20-27:   × " × × × × '
    0x39, 0x0F, 0x00, 0x00, 0x0C, 0x40, 0x04, 0x52, // 28-2F: ( ) × × , - . /
    0x3F, 0x06, 0x5B, 0x4F, 0x66, 0x6D, 0x7D, 0x27, // 30-37: 0 1 2 3 4 5 6 7
    0x7F, 0x6F, 0x00, 0x00, 0x39, 0x48, 0x0F, 0x53, // 38-3F: 8 9 × × < = > ?
    0x7B, 0x77, 0x7C, 0x39, 0x5E, 0x79, 0x71, 0x3D, // 40-47: @ A B C D E F G
    0x76, 0x30, 0x1E, 0x76, 0x38, 0x2B, 0x37, 0x3F, // 48-4F: H I J K L M N O
    0x73, 0x67, 0x77, 0x6D, 0x07, 0x3E, 0x3E, 0x7E, // 50-57: P Q R S T U V W
    0x76, 0x6E, 0x5B, 0x39, 0x64, 0x0F, 0x23, 0x08, // 58-5F: X Y Z [ \ ] ^ _
    0x20, 0x5F, 0x7C, 0x58, 0x5E, 0x7B, 0x71, 0x6F, // 60-67: ` a b c d e f g
    0x74, 0x10, 0x0E, 0x76, 0x06, 0x55, 0x54, 0x5C, // 68-6F: h i j k l m n o
    0x73, 0x67, 0x50, 0x6D, 0x78, 0x1C, 0x1C, 0x7E, // 70-77: p q r s t u v w
    0x76, 0x6E, 0x5B, 0x46, 0x30, 0x70, 0x40, 0x00, // 78-7F: x y z { | } ~ ×
];

// ----------------------------------------------------------------------------

#[derive(Copy, Clone)]
pub struct SevenSegmentMetrics {
    pub segment_spacing: f32,
    pub segment_thickness: f32,

    pub digit_median: f32,
    pub digit_ratio: f32,
    pub digit_shearing: f32,
    pub digit_spacing: f32,

    pub margin_horizontal: f32,
    pub margin_vertical: f32,

    pub colon_separation: f32,
}

impl Default for SevenSegmentMetrics {
    fn default() -> Self {
        SevenSegmentMetricsPreset::Default.metrics()
    }
}

// ----------------------------------------------------------------------------

#[non_exhaustive]
pub enum SevenSegmentMetricsPreset {
    Default,
    KnightRider,
}

impl SevenSegmentMetricsPreset {
    pub fn metrics(&self) -> SevenSegmentMetrics {
        match *self {
            SevenSegmentMetricsPreset::Default => SevenSegmentMetrics {
                segment_spacing: 0.02,
                segment_thickness: 0.1,
                digit_median: -0.05,
                digit_ratio: 0.5,
                digit_shearing: 0.1,
                digit_spacing: 0.35,
                margin_horizontal: 0.3,
                margin_vertical: 0.1,
                colon_separation: 0.25,
            },
            SevenSegmentMetricsPreset::KnightRider => SevenSegmentMetrics {
                segment_spacing: 0.02,
                segment_thickness: 0.12,
                digit_median: -0.05,
                digit_ratio: 1.0,
                digit_shearing: 0.1,
                digit_spacing: 0.20,
                margin_horizontal: 0.3,
                margin_vertical: 0.1,
                colon_separation: 0.25,
            },
        }
    }
}

// ----------------------------------------------------------------------------

#[derive(Copy, Clone)]
pub struct SevenSegmentStyle {
    pub background_color: Color32,
    pub segment_on_color: Color32,
    pub segment_off_color: Color32,
    pub segment_on_stroke: Stroke,
    pub segment_off_stroke: Stroke,
}

impl Default for SevenSegmentStyle {
    fn default() -> Self {
        SevenSegmentStylePreset::Default.style()
    }
}

// ----------------------------------------------------------------------------

#[non_exhaustive]
pub enum SevenSegmentStylePreset {
    Default,
    Calculator,
    NintendoGameBoy,
    KnightRider,
    BlueNegative,
    Amber,
    DeLoreanRed,
    DeLoreanGreen,
    DeLoreanAmber,
}

impl SevenSegmentStylePreset {
    pub fn style(&self) -> SevenSegmentStyle {
        match *self {
            SevenSegmentStylePreset::Default => SevenSegmentStyle {
                background_color: Color32::from_rgb(0x00, 0x20, 0x00),
                segment_on_color: Color32::from_rgb(0x00, 0xF0, 0x00),
                segment_off_color: Color32::from_rgb(0x00, 0x30, 0x00),
                segment_on_stroke: Stroke::none(),
                segment_off_stroke: Stroke::none(),
            },
            SevenSegmentStylePreset::Calculator => SevenSegmentStyle {
                background_color: Color32::from_rgb(0xC5, 0xCB, 0xB6),
                segment_on_color: Color32::from_rgb(0x00, 0x00, 0x00),
                segment_off_color: Color32::from_rgb(0xB9, 0xBE, 0xAB),
                segment_on_stroke: Stroke::none(),
                segment_off_stroke: Stroke::none(),
            },
            SevenSegmentStylePreset::NintendoGameBoy => SevenSegmentStyle {
                background_color: Color32::from_rgb(0x9B, 0xBC, 0x0F),
                segment_on_color: Color32::from_rgb(0x0F, 0x38, 0x0F),
                segment_off_color: Color32::from_rgb(0x8B, 0xAC, 0x0F),
                segment_on_stroke: Stroke::none(),
                segment_off_stroke: Stroke::none(),
            },
            SevenSegmentStylePreset::KnightRider => SevenSegmentStyle {
                background_color: Color32::from_rgb(0x10, 0x00, 0x00),
                segment_on_color: Color32::from_rgb(0xC8, 0x00, 0x00),
                segment_off_color: Color32::from_rgb(0x20, 0x00, 0x00),
                segment_on_stroke: Stroke::none(),
                segment_off_stroke: Stroke::none(),
            },
            SevenSegmentStylePreset::BlueNegative => SevenSegmentStyle {
                background_color: Color32::from_rgb(0x00, 0x00, 0xFF),
                segment_on_color: Color32::from_rgb(0xE0, 0xFF, 0xFF),
                segment_off_color: Color32::from_rgb(0x28, 0x28, 0xFF),
                segment_on_stroke: Stroke::none(),
                segment_off_stroke: Stroke::none(),
            },
            SevenSegmentStylePreset::Amber => SevenSegmentStyle {
                background_color: Color32::from_rgb(0x1D, 0x12, 0x07),
                segment_on_color: Color32::from_rgb(0xFF, 0x9A, 0x21),
                segment_off_color: Color32::from_rgb(0x33, 0x20, 0x00),
                segment_on_stroke: Stroke::none(),
                segment_off_stroke: Stroke::none(),
            },
            SevenSegmentStylePreset::DeLoreanRed => todo!(),
            SevenSegmentStylePreset::DeLoreanGreen => todo!(),
            SevenSegmentStylePreset::DeLoreanAmber => todo!(),
        }
    }
}

// ----------------------------------------------------------------------------

#[must_use = "You should put this widget in an ui with `ui.add(widget);`"]
pub struct SevenSegmentWidget<'a> {
    display_string: String,
    digit_count: usize,
    digit_height: f32,
    metrics: SevenSegmentMetrics,
    style: SevenSegmentStyle,
    font: &'a SevenSegmentFont,
}

impl<'a> SevenSegmentWidget<'a> {
    pub fn new() -> Self {
        Self {
            display_string: String::new(),
            digit_count: 0,
            digit_height: 48.0,
            metrics: SevenSegmentMetrics::default(),
            style: SevenSegmentStylePreset::Default.style(),
            font: &DEFAULT_FONT,
        }
    }

    pub fn from_string(display_string: &str) -> Self {
        Self::new()
            .display_string(display_string)
            .digit_count(display_string.len())
    }

    pub fn display_string(mut self, display_string: &str) -> Self {
        self.display_string = display_string.to_string();
        self
    }

    pub fn digit_count(mut self, digit_count: usize) -> Self {
        self.digit_count = digit_count;
        self
    }

    pub fn digit_height(mut self, digit_height: impl Into<f32>) -> Self {
        self.digit_height = digit_height.into();
        self
    }

    pub fn style(mut self, style: SevenSegmentStyle) -> Self {
        self.style = style;
        self
    }

    pub fn style_preset(mut self, preset: SevenSegmentStylePreset) -> Self {
        self.style = preset.style();
        self
    }

    pub fn metrics(mut self, metrics: SevenSegmentMetrics) -> Self {
        self.metrics = metrics;
        self
    }

    pub fn metrics_preset(mut self, preset: SevenSegmentMetricsPreset) -> Self {
        self.metrics = preset.metrics();
        self
    }

    pub fn font(mut self, font: &'a SevenSegmentFont) -> Self {
        self.font = font;
        self
    }
}

impl<'a> Widget for SevenSegmentWidget<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        let digit_height = self.digit_height;
        let digit_width = digit_height * self.metrics.digit_ratio;

        // Turn relative metrics to absolute metrics
        let segment_thickness = self.metrics.segment_thickness * digit_height;
        let segment_spacing = self.metrics.segment_spacing * digit_height;
        let digit_shearing = self.metrics.digit_shearing * digit_width;
        let digit_spacing = self.metrics.digit_spacing * digit_width;
        let margin_horizontal = self.metrics.margin_horizontal * digit_width;
        let margin_vertical = self.metrics.margin_vertical * digit_height;
        let digit_median = self.metrics.digit_median * (digit_height / 2.0);
        let colon_separation = self.metrics.colon_separation * (digit_height / 2.0);

        let desired_size = vec2(
            (digit_width * self.digit_count as f32)
                + (digit_spacing * (self.digit_count - 1) as f32)
                + (2.0 * margin_horizontal)
                + (2.0 * digit_shearing.abs()),
            digit_height + (2.0 * margin_vertical),
        );

        let (rect, response) = ui.allocate_exact_size(desired_size, Sense::hover());

        ui.painter()
            .rect(rect, 0.0, self.style.background_color, Stroke::none());

        let paint_digit = |digit_bits: u8, digit_center: Pos2| {
            let p = |dx, dy| {
                digit_center + vec2(dx, dy)
                    - vec2((dy / (digit_height / 2.0)) * digit_shearing, 0.0)
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

            #[rustfmt::skip]
            let apostrophe_points: Vec<Pos2> = vec![
                p(-(digit_width / 2.0) - (digit_spacing / 2.0) - (segment_thickness / 2.0), -(digit_height / 2.0)                            ),
                p(-(digit_width / 2.0) - (digit_spacing / 2.0) + (segment_thickness / 2.0), -(digit_height / 2.0)                            ),
                p(-(digit_width / 2.0) - (digit_spacing / 2.0) - (segment_thickness / 2.0), -(digit_height / 2.0) + (segment_thickness * 2.0)),
            ];

            #[rustfmt::skip]
            let (colon_top_pos, colon_bottom_pos, dot_pos) = (
                p(-(digit_width / 2.0) - (digit_spacing / 2.0), digit_median - colon_separation),
                p(-(digit_width / 2.0) - (digit_spacing / 2.0), digit_median + colon_separation),
                p( (digit_width / 2.0) + (digit_spacing / 2.0), (digit_height / 2.0) - (segment_thickness / 2.0))
            );

            for (segment_index, segment_points) in segment_points.iter().enumerate() {
                let segment_on = ((digit_bits >> segment_index) & 0x01) != 0x00;

                let (fill, stroke) = if segment_on {
                    (self.style.segment_on_color, self.style.segment_on_stroke)
                } else {
                    (self.style.segment_off_color, self.style.segment_off_stroke)
                };

                ui.painter()
                    .add(Shape::convex_polygon(segment_points.to_vec(), fill, stroke));
            }

            ui.painter().circle(
                colon_top_pos,
                segment_thickness / 2.0,
                self.style.segment_off_color,
                self.style.segment_off_stroke,
            );

            ui.painter().circle(
                colon_bottom_pos,
                segment_thickness / 2.0,
                self.style.segment_off_color,
                self.style.segment_off_stroke,
            );

            ui.painter().circle(
                dot_pos,
                segment_thickness / 2.0,
                self.style.segment_off_color,
                self.style.segment_off_stroke,
            );

            ui.painter().add(Shape::convex_polygon(
                apostrophe_points.to_vec(),
                self.style.segment_off_color,
                self.style.segment_off_stroke,
            ));
        };

        for (digit_index, digit_char) in self.display_string.chars().enumerate() {
            let digit_bits = if digit_char.is_ascii() {
                self.font[digit_char as usize]
            } else {
                0x00
            };

            let digit_center = rect.left_center()
                + vec2(
                    margin_horizontal
                        + digit_shearing.abs()
                        + ((digit_width + digit_spacing) * digit_index as f32)
                        + (digit_width / 2.0),
                    0.0,
                );

            paint_digit(digit_bits, digit_center);
        }

        response
    }
}
