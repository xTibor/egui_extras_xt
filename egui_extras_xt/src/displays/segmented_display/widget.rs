use egui::{pos2, vec2, Pos2, Response, Sense, Shape, Stroke, Ui, Widget};
use itertools::Itertools;

use crate::displays::segmented_display::{
    DisplayDigit, DisplayKind, DisplayMetrics, DisplayMetricsPreset,
};
use crate::displays::{DisplayStyle, DisplayStylePreset};

#[must_use = "You should put this widget in an ui with `ui.add(widget);`"]
pub struct SegmentedDisplayWidget {
    display_kind: DisplayKind,
    digits: Vec<DisplayDigit>,
    digit_height: f32,
    metrics: DisplayMetrics,
    style: DisplayStyle,
    show_dots: bool,
    show_colons: bool,
    show_apostrophes: bool,
}

impl SegmentedDisplayWidget {
    pub fn new(display_kind: DisplayKind) -> Self {
        Self {
            display_kind,
            digits: Vec::new(),
            digit_height: 80.0,
            metrics: DisplayMetrics::default(),
            style: DisplayStylePreset::Default.style(),
            show_dots: true,
            show_colons: true,
            show_apostrophes: true,
        }
    }

    pub fn seven_segment<T: AsRef<str>>(value: T) -> Self {
        Self::new(DisplayKind::SevenSegment).push_string(value.as_ref())
    }

    pub fn nine_segment<T: AsRef<str>>(value: T) -> Self {
        Self::new(DisplayKind::NineSegment).push_string(value.as_ref())
    }

    pub fn sixteen_segment<T: AsRef<str>>(value: T) -> Self {
        Self::new(DisplayKind::SixteenSegment).push_string(value.as_ref())
    }

    pub fn push_string<T: AsRef<str>>(mut self, value: T) -> Self {
        let display_impl = self.display_kind.display_impl();

        self.digits.extend(
            [None]
                .into_iter()
                .chain(value.as_ref().chars().map(Some))
                .chain([None])
                .tuple_windows()
                .filter_map(|(prev, curr, next)| match curr {
                    Some('.') if self.show_dots => None,
                    Some(':') if self.show_colons => None,
                    Some('\'') if self.show_apostrophes => None,
                    Some(c) if display_impl.glyph(c).is_some() => Some(DisplayDigit {
                        glyph: display_impl.glyph(c).unwrap(),
                        dot: (next == Some('.')) && self.show_dots,
                        colon: (prev == Some(':')) && self.show_colons,
                        apostrophe: (prev == Some('\'')) && self.show_apostrophes,
                    }),
                    _ => None,
                }),
        );
        self
    }

    pub fn push_digit(mut self, digit: DisplayDigit) -> Self {
        self.digits.push(digit);
        self
    }

    pub fn digit_height(mut self, digit_height: impl Into<f32>) -> Self {
        self.digit_height = digit_height.into();
        self
    }

    pub fn style(mut self, style: DisplayStyle) -> Self {
        self.style = style;
        self
    }

    pub fn style_preset(mut self, preset: DisplayStylePreset) -> Self {
        self.style = preset.style();
        self
    }

    pub fn metrics(mut self, metrics: DisplayMetrics) -> Self {
        self.metrics = metrics;
        self
    }

    pub fn metrics_preset(mut self, preset: DisplayMetricsPreset) -> Self {
        self.metrics = preset.metrics();
        self
    }

    pub fn show_dots(mut self, show_dots: bool) -> Self {
        self.show_dots = show_dots;
        self
    }

    pub fn show_colons(mut self, show_colons: bool) -> Self {
        self.show_colons = show_colons;
        self
    }

    pub fn show_apostrophes(mut self, show_apostrophes: bool) -> Self {
        self.show_apostrophes = show_apostrophes;
        self
    }
}

impl Widget for SegmentedDisplayWidget {
    fn ui(self, ui: &mut Ui) -> Response {
        let display_impl = self.display_kind.display_impl();

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
            (digit_width * self.digits.len() as f32)
                + (digit_spacing * (self.digits.len().saturating_sub(1)) as f32)
                + (2.0 * margin_horizontal)
                + (2.0 * digit_shearing.abs()),
            digit_height + (2.0 * margin_vertical),
        );

        let (rect, response) = ui.allocate_exact_size(desired_size, Sense::click());

        let mut child_ui = ui.child_ui(rect, *ui.layout());
        child_ui.set_clip_rect(child_ui.clip_rect().intersect(rect));

        if child_ui.is_rect_visible(rect) {
            // Draw the widget background without clipping
            ui.painter().rect(
                rect,
                ui.style().visuals.noninteractive().rounding,
                self.style.background_color,
                Stroke::NONE,
            );

            let segment_geometry = display_impl.geometry(
                digit_width,
                digit_height,
                segment_thickness,
                segment_spacing,
                digit_median,
            );
            assert_eq!(segment_geometry.len(), display_impl.segment_count());

            #[rustfmt::skip]
            let apostrophe_points: Vec<Pos2> = vec![
                pos2(-(digit_width / 2.0) - (digit_spacing / 2.0) - (segment_thickness / 2.0), -(digit_height / 2.0)                            ),
                pos2(-(digit_width / 2.0) - (digit_spacing / 2.0) + (segment_thickness / 2.0), -(digit_height / 2.0)                            ),
                pos2(-(digit_width / 2.0) - (digit_spacing / 2.0) - (segment_thickness / 2.0), -(digit_height / 2.0) + (segment_thickness * 2.0)),
            ];

            #[rustfmt::skip]
            let (colon_top_pos, colon_bottom_pos, dot_pos) = (
                pos2(-(digit_width / 2.0) - (digit_spacing / 2.0), digit_median - colon_separation),
                pos2(-(digit_width / 2.0) - (digit_spacing / 2.0), digit_median + colon_separation),
                pos2( (digit_width / 2.0) + (digit_spacing / 2.0), (digit_height / 2.0) - (segment_thickness / 2.0))
            );

            let paint_digit = |digit: &DisplayDigit, digit_center: Pos2| {
                let transform = |&Pos2 { x, y }| {
                    digit_center + vec2(x, y)
                        - vec2((y / (digit_height / 2.0)) * digit_shearing, 0.0)
                };

                for (segment_index, segment_points) in segment_geometry.iter().enumerate() {
                    let segment_active = ((digit.glyph >> segment_index) & 0x01) != 0x00;

                    // TODO: concave_polygon
                    // https://github.com/emilk/egui/issues/513
                    child_ui.painter().add(Shape::convex_polygon(
                        segment_points.iter().map(transform).collect_vec(),
                        self.style.foreground_color(segment_active),
                        self.style.foreground_stroke(segment_active),
                    ));
                }

                if self.show_dots {
                    child_ui.painter().circle(
                        transform(&dot_pos),
                        segment_thickness / 2.0,
                        self.style.foreground_color(digit.dot),
                        self.style.foreground_stroke(digit.dot),
                    );
                }

                if self.show_colons {
                    child_ui.painter().circle(
                        transform(&colon_top_pos),
                        segment_thickness / 2.0,
                        self.style.foreground_color(digit.colon),
                        self.style.foreground_stroke(digit.colon),
                    );

                    child_ui.painter().circle(
                        transform(&colon_bottom_pos),
                        segment_thickness / 2.0,
                        self.style.foreground_color(digit.colon),
                        self.style.foreground_stroke(digit.colon),
                    );
                }

                if self.show_apostrophes {
                    child_ui.painter().add(Shape::convex_polygon(
                        apostrophe_points.iter().map(transform).collect_vec(),
                        self.style.foreground_color(digit.apostrophe),
                        self.style.foreground_stroke(digit.apostrophe),
                    ));
                }
            };

            for (digit_index, digit) in self.digits.iter().enumerate() {
                let digit_center = rect.left_center()
                    + vec2(
                        margin_horizontal
                            + digit_shearing.abs()
                            + ((digit_width + digit_spacing) * digit_index as f32)
                            + (digit_width / 2.0),
                        0.0,
                    );

                paint_digit(digit, digit_center);
            }
        }

        response
    }
}
