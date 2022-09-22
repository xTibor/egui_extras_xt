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

fn set(get_set_value: &mut GetSetValue<'_>, track_enabled: bool) {
    (get_set_value)(Some(track_enabled));
}

// ----------------------------------------------------------------------------

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum SignalEdge {
    FallingEdge,
    RisingEdge,
}

impl SignalEdge {
    fn from_samples<SampleType>(sample_a: &SampleType, sample_b: &SampleType) -> Option<SignalEdge>
    where
        SampleType: SampleRange<SampleType> + PartialOrd,
    {
        match (*sample_a >= SampleType::ZERO, *sample_b >= SampleType::ZERO) {
            (false, true) => Some(SignalEdge::RisingEdge),
            (true, false) => Some(SignalEdge::FallingEdge),
            _ => None,
        }
    }
}

// ----------------------------------------------------------------------------

#[derive(Debug)]
pub enum TriggerMode {
    BufferCenter,
    SignalEdge(SignalEdge),
    SignalEdgeCenter(SignalEdge, SignalEdge),
}

#[derive(Debug)]
pub enum BufferLayout {
    Planar,
    Interleaved,
}

// ----------------------------------------------------------------------------

pub trait SampleRange<T> {
    const ZERO: T;
    const DISPLAY_RANGE: RangeInclusive<f32>;
}

impl SampleRange<u8> for u8 {
    const ZERO: u8 = 128;
    const DISPLAY_RANGE: RangeInclusive<f32> = 0.0..=255.0;
}

impl SampleRange<i8> for i8 {
    const ZERO: i8 = 0;
    const DISPLAY_RANGE: RangeInclusive<f32> = -128.0..=127.0;
}

impl SampleRange<u16> for u16 {
    const ZERO: u16 = 32768;
    const DISPLAY_RANGE: RangeInclusive<f32> = 0.0..=65535.0;
}

impl SampleRange<i16> for i16 {
    const ZERO: i16 = 0;
    const DISPLAY_RANGE: RangeInclusive<f32> = -32768.0..=32767.0;
}

impl SampleRange<u32> for u32 {
    const ZERO: u32 = 2147483648;
    const DISPLAY_RANGE: RangeInclusive<f32> = 0.0..=4294967295.0;
}

impl SampleRange<i32> for i32 {
    const ZERO: i32 = 0;
    const DISPLAY_RANGE: RangeInclusive<f32> = -2147483648.0..=2147483647.0;
}

impl SampleRange<f32> for f32 {
    const ZERO: f32 = 0.0;
    const DISPLAY_RANGE: RangeInclusive<f32> = -1.0..=1.0;
}

impl SampleRange<f64> for f64 {
    const ZERO: f64 = 0.0;
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
    window_size: Option<usize>,
    width: f32,
    height: f32,
    track_name: Option<String>,
    channel_names: Option<Vec<String>>,
    show_header: bool,
}

impl<'a, SampleType> WaveformDisplayWidget<'a, SampleType>
where
    SampleType: SampleRange<SampleType>,
{
    pub fn new(track_enabled: &'a mut bool) -> Self {
        Self::from_get_set(move |v: Option<bool>| {
            if let Some(v) = v {
                *track_enabled = v;
            }
            *track_enabled
        })
    }

    pub fn from_get_set(get_set_value: impl 'a + FnMut(Option<bool>) -> bool) -> Self {
        Self {
            get_set_value: Box::new(get_set_value),
            buffer: None,
            buffer_layout: BufferLayout::Interleaved,
            channels: 1,
            trigger_mode: TriggerMode::SignalEdge(SignalEdge::RisingEdge),
            window_size: None,
            width: 256.0,
            height: 64.0,
            track_name: None,
            channel_names: None,
            show_header: true,
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
        self.window_size = Some(window_size);
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

    pub fn track_name(mut self, track_name: impl ToString) -> Self {
        self.track_name = Some(track_name.to_string());
        self
    }

    pub fn channel_names(mut self, channel_names: &[impl ToString]) -> Self {
        self.channel_names = Some(
            channel_names
                .iter()
                .map(|channel_name| channel_name.to_string())
                .collect_vec(),
        );
        self
    }

    pub fn show_header(mut self, show_header: bool) -> Self {
        self.show_header = show_header;
        self
    }
}

impl<'a, SampleType> Widget for WaveformDisplayWidget<'a, SampleType>
where
    SampleType: SampleRange<SampleType> + Into<f32> + Copy + PartialOrd,
{
    fn ui(mut self, ui: &mut Ui) -> Response {
        let desired_size = vec2(self.width, self.height);
        let (rect, mut response) = ui.allocate_exact_size(desired_size, Sense::click());

        if response.clicked() {
            let track_enabled = get(&mut self.get_set_value);
            set(&mut self.get_set_value, !track_enabled);
            response.mark_changed();
        }

        if ui.is_rect_visible(rect) {
            let track_enabled = get(&mut self.get_set_value);

            let visuals = *ui.style().interact(&response);

            let font_id = FontSelection::Default.resolve(ui.style());

            let foreground_color = if track_enabled {
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
                let channel_buffer_length = buffer.len() / self.channels;

                if let Some(ref channel_names) = self.channel_names {
                    assert_eq!(channel_names.len(), self.channels);
                }

                let render_channel =
                    |rect: Rect, channel_buffer: &[SampleType], channel_name: &Option<String>| {
                        let header_height = font_id.size;
                        let waveform_vertical_margin = 4.0;

                        // Header
                        if self.show_header {
                            let header_rect = {
                                let mut tmp = rect;
                                tmp.set_height(header_height);
                                tmp
                            };

                            if let Some(channel_name) = channel_name {
                                ui.painter().text(
                                    header_rect.center(),
                                    Align2::CENTER_CENTER,
                                    channel_name,
                                    font_id.clone(),
                                    foreground_color,
                                );
                            }

                            if track_enabled {
                                ui.painter().text(
                                    header_rect.right_center(),
                                    Align2::RIGHT_CENTER,
                                    '\u{1F508}',
                                    font_id.clone(),
                                    foreground_color,
                                );
                            }
                        }

                        // Waveform
                        {
                            let waveform_rect = if self.show_header {
                                let mut tmp = rect;
                                tmp = tmp.translate(vec2(0.0, header_height));
                                tmp.set_height(rect.height() - header_height);
                                tmp.shrink2(vec2(0.0, waveform_vertical_margin))
                            } else {
                                rect.shrink2(vec2(0.0, waveform_vertical_margin))
                            };

                            let window_size = self.window_size.unwrap_or(channel_buffer_length / 2); // Default window size
                            assert!(window_size <= channel_buffer_length);

                            let window_center_valid_range =
                                (window_size / 2)..=(channel_buffer_length - (window_size / 2));

                            let (left_target, right_target) =
                                (SignalEdge::RisingEdge, SignalEdge::RisingEdge);

                            let left_signal_edge_delta = channel_buffer
                                [..=(channel_buffer_length / 2)]
                                .iter()
                                .rev()
                                .tuple_windows()
                                .map(|(a, b)| (b, a))
                                .enumerate()
                                .map(|(delta, (a, b))| (delta, SignalEdge::from_samples(a, b)))
                                .find(|(_, signal_edge)| *signal_edge == Some(left_target))
                                .map(|(delta, _)| delta + 1)
                                .unwrap_or(channel_buffer_length / 2);

                            let right_signal_edge_delta = channel_buffer
                                [(channel_buffer_length / 2)..]
                                .iter()
                                .tuple_windows()
                                .enumerate()
                                .map(|(delta, (a, b))| (delta, SignalEdge::from_samples(a, b)))
                                .find(|(_, signal_edge)| *signal_edge == Some(right_target))
                                .map(|(delta, _)| delta + 0)
                                .unwrap_or(channel_buffer_length / 2);

                            let mut window_center =
                                if right_signal_edge_delta < left_signal_edge_delta {
                                    (channel_buffer_length / 2) + right_signal_edge_delta
                                } else {
                                    (channel_buffer_length / 2) - left_signal_edge_delta
                                };

                            if !window_center_valid_range.contains(&window_center) {
                                window_center = channel_buffer_length / 2;
                            }

                            let waveform_points = channel_buffer[(window_center - window_size / 2)
                                ..(window_center + window_size / 2)]
                                .iter()
                                .enumerate()
                                .map(|(index, &sample)| {
                                    pos2(
                                        remap_clamp(
                                            index as f32,
                                            0.0..=(window_size as f32 - 1.0),
                                            waveform_rect.x_range(),
                                        ),
                                        remap_clamp(
                                            sample.into(),
                                            SampleType::DISPLAY_RANGE,
                                            waveform_rect.bottom_up_range(),
                                        ),
                                    )
                                })
                                .collect_vec();

                            ui.painter().line_segment(
                                [waveform_rect.left_center(), waveform_rect.right_center()],
                                ui.style().visuals.noninteractive().fg_stroke,
                            );

                            ui.painter()
                                .add(Shape::line(waveform_points, visuals.fg_stroke));
                        }
                    };

                if self.channels == 1 {
                    render_channel(rect, buffer, &self.track_name);
                } else {
                    for channel_id in 0..self.channels {
                        let channel_buffer = match self.buffer_layout {
                            BufferLayout::Planar => buffer
                                .iter()
                                .cloned()
                                .skip(channel_id * channel_buffer_length)
                                .take(channel_buffer_length)
                                .collect_vec(),
                            BufferLayout::Interleaved => buffer
                                .iter()
                                .cloned()
                                .skip(channel_id)
                                .step_by(self.channels)
                                .take(channel_buffer_length)
                                .collect_vec(),
                        };
                        assert_eq!(channel_buffer.len(), channel_buffer_length);

                        let channel_rect = Rect::from_min_size(
                            rect.left_top()
                                + rect.size() / vec2(self.channels as f32, 1.0)
                                    * vec2(channel_id as f32, 0.0),
                            rect.size() / vec2(self.channels as f32, 1.0),
                        );

                        let channel_name: Option<String> =
                            if let Some(ref channel_names) = self.channel_names {
                                channel_names.get(channel_id).cloned()
                            } else {
                                self.track_name.clone()
                            };

                        render_channel(channel_rect, channel_buffer.as_slice(), &channel_name);

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
