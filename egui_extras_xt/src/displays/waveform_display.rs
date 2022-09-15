use std::ops::RangeInclusive;

use egui::{
    pos2, remap_clamp, vec2, Align2, FontSelection, Rect, Response, Sense, Shape, Ui, Widget,
};
use itertools::Itertools;

// ----------------------------------------------------------------------------

/// Combined into one function (rather than two) to make it easier
/// for the borrow checker.
type GetSetValue<'a> = Box<dyn 'a + FnMut(Option<bool>) -> bool>;

fn get(get_set_value: &mut GetSetValue<'_>) -> bool {
    (get_set_value)(None)
}

fn set(get_set_value: &mut GetSetValue<'_>, value: bool) {
    (get_set_value)(Some(value));
}

// ----------------------------------------------------------------------------

pub enum SignalEdge {
    FallingEdge,
    RisingEdge,
}

pub enum TriggerMode {
    BufferCenter,
    SignalEdge(SignalEdge),
    SignalEdgeCenter(SignalEdge, SignalEdge),
}

pub enum BufferLayout {
    Planar,
    Interleaved,
}

// ----------------------------------------------------------------------------

pub trait SampleRange<T> {
    const SAMPLE_CENTER: T;
    const DISPLAY_RANGE: RangeInclusive<f32>;
}

impl SampleRange<u8> for u8 {
    const SAMPLE_CENTER: u8 = 128;
    const DISPLAY_RANGE: RangeInclusive<f32> = 0.0..=255.0;
}

impl SampleRange<i8> for i8 {
    const SAMPLE_CENTER: i8 = 0;
    const DISPLAY_RANGE: RangeInclusive<f32> = -128.0..=127.0;
}

impl SampleRange<u16> for u16 {
    const SAMPLE_CENTER: u16 = 32768;
    const DISPLAY_RANGE: RangeInclusive<f32> = 0.0..=65535.0;
}

impl SampleRange<i16> for i16 {
    const SAMPLE_CENTER: i16 = 0;
    const DISPLAY_RANGE: RangeInclusive<f32> = -32768.0..=32767.0;
}

impl SampleRange<f32> for f32 {
    const SAMPLE_CENTER: f32 = 0.0;
    const DISPLAY_RANGE: RangeInclusive<f32> = -1.0..=1.0;
}

impl SampleRange<f64> for f64 {
    const SAMPLE_CENTER: f64 = 0.0;
    const DISPLAY_RANGE: RangeInclusive<f32> = -1.0..=1.0;
}

// ----------------------------------------------------------------------------

#[must_use = "You should put this widget in an ui with `ui.add(widget);`"]
pub struct WaveformDisplayWidget<'a, SampleType>
where
    SampleType: SampleRange<SampleType>,
{
    get_set_value: GetSetValue<'a>,
    buffer: Option<&'a [SampleType]>,
    buffer_layout: BufferLayout,
    channels: usize,
    trigger_mode: TriggerMode,
    window_size: usize,
    width: f32,
    height: f32,
    label: Option<String>,
}

impl<'a, SampleType> WaveformDisplayWidget<'a, SampleType>
where
    SampleType: SampleRange<SampleType>,
{
    pub fn new(value: &'a mut bool) -> Self {
        Self::from_get_set(move |v: Option<bool>| {
            if let Some(v) = v {
                *value = v;
            }
            *value
        })
    }

    pub fn from_get_set(get_set_value: impl 'a + FnMut(Option<bool>) -> bool) -> Self {
        Self {
            get_set_value: Box::new(get_set_value),
            buffer: None,
            buffer_layout: BufferLayout::Interleaved,
            channels: 1,
            trigger_mode: TriggerMode::SignalEdge(SignalEdge::RisingEdge),
            window_size: 128,
            width: 256.0,
            height: 64.0,
            label: None,
        }
    }

    pub fn buffer(mut self, buffer: &'a [SampleType]) -> Self {
        self.buffer = Some(buffer);
        self
    }

    pub fn buffer_layout(mut self, buffer_layout: BufferLayout) -> Self {
        self.buffer_layout = buffer_layout;
        self
    }

    pub fn channels(mut self, channels: usize) -> Self {
        self.channels = channels;
        self
    }

    pub fn trigger_mode(mut self, trigger_mode: TriggerMode) -> Self {
        self.trigger_mode = trigger_mode;
        self
    }

    pub fn window_size(mut self, window_size: usize) -> Self {
        self.window_size = window_size;
        self
    }

    pub fn width(mut self, width: impl Into<f32>) -> Self {
        self.width = width.into();
        self
    }

    pub fn height(mut self, height: impl Into<f32>) -> Self {
        self.height = height.into();
        self
    }

    pub fn label(mut self, label: impl ToString) -> Self {
        self.label = Some(label.to_string());
        self
    }
}

impl<'a, SampleType> Widget for WaveformDisplayWidget<'a, SampleType>
where
    SampleType: SampleRange<SampleType> + Into<f32> + Copy,
{
    fn ui(mut self, ui: &mut Ui) -> Response {
        let desired_size = vec2(self.width, self.height);
        let (rect, mut response) = ui.allocate_exact_size(desired_size, Sense::click());

        if response.clicked() {
            let value = get(&mut self.get_set_value);
            set(&mut self.get_set_value, !value);
            response.mark_changed();
        }

        if ui.is_rect_visible(rect) {
            let value = get(&mut self.get_set_value);

            let visuals = *ui.style().interact(&response);

            let font_id = FontSelection::Default.resolve(ui.style());

            let foreground_color = if value {
                visuals.text_color()
            } else {
                ui.style().noninteractive().text_color()
            };

            ui.painter().rect(
                rect,
                visuals.rounding,
                ui.style().visuals.extreme_bg_color,
                visuals.fg_stroke,
            );

            if let Some(buffer) = self.buffer {
                assert_eq!(buffer.len() % self.channels, 0);
                let channel_sample_count = buffer.len() / self.channels;

                let render_channel =
                    |rect: Rect, samples: &[SampleType], label: &Option<String>| {
                        let waveform_points = samples
                            .into_iter()
                            .enumerate()
                            .map(|(sample_index, &sample)| {
                                pos2(
                                    remap_clamp(
                                        sample_index as f32,
                                        0.0..=(channel_sample_count as f32 - 1.0),
                                        rect.x_range(),
                                    ),
                                    remap_clamp(
                                        sample.into(),
                                        SampleType::DISPLAY_RANGE,
                                        (rect.bottom() - 4.0)..=(rect.top() + 4.0),
                                    ),
                                )
                            })
                            .collect_vec();

                        ui.painter().line_segment(
                            [rect.left_center(), rect.right_center()],
                            ui.style().visuals.noninteractive().fg_stroke,
                        );

                        ui.painter()
                            .add(Shape::line(waveform_points, visuals.fg_stroke));

                        if let Some(label) = label {
                            ui.painter().text(
                                rect.left_top() + vec2(4.0, 4.0),
                                Align2::LEFT_TOP,
                                label,
                                font_id.clone(),
                                foreground_color,
                            );
                        }

                        if value {
                            ui.painter().text(
                                rect.right_top() + vec2(-4.0, 4.0),
                                Align2::RIGHT_TOP,
                                '\u{1F508}',
                                font_id.clone(),
                                foreground_color,
                            );
                        }
                    };

                if self.channels == 1 {
                    render_channel(rect, buffer, &self.label);
                } else {
                    for channel_id in 0..self.channels {
                        let channel_samples = match self.buffer_layout {
                            BufferLayout::Planar => buffer
                                .iter()
                                .cloned()
                                .skip(channel_id * channel_sample_count)
                                .take(channel_sample_count)
                                .collect_vec(),
                            BufferLayout::Interleaved => buffer
                                .iter()
                                .cloned()
                                .skip(channel_id)
                                .step_by(self.channels)
                                .take(channel_sample_count)
                                .collect_vec(),
                        };
                        assert_eq!(channel_samples.len(), channel_sample_count);

                        let channel_rect = Rect::from_min_size(
                            rect.left_top()
                                + rect.size() / vec2(self.channels as f32, 1.0)
                                    * vec2(channel_id as f32, 0.0),
                            rect.size() / vec2(self.channels as f32, 1.0),
                        );

                        render_channel(channel_rect, channel_samples.as_slice(), &self.label);

                        if channel_id < self.channels - 1 {
                            ui.painter().line_segment(
                                [channel_rect.right_top(), channel_rect.right_bottom()],
                                ui.style().visuals.noninteractive().fg_stroke,
                            );
                        }
                    }
                }
            }
        };

        response
    }
}
