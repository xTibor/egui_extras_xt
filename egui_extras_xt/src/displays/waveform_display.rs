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
    track_name: Option<String>,
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
            window_size: 128,
            width: 256.0,
            height: 64.0,
            track_name: None,
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

    pub fn track_name(mut self, track_name: impl ToString) -> Self {
        self.track_name = Some(track_name.to_string());
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

                let render_channel =
                    |rect: Rect, channel_buffer: &[SampleType], track_name: &Option<String>| {
                        let header_height = font_id.size;
                        let header_rect = {
                            let mut tmp = rect;
                            tmp.set_height(header_height);
                            tmp
                        };

                        let waveform_rect = {
                            let mut tmp = rect;
                            tmp = tmp.translate(vec2(0.0, header_height));
                            tmp.set_height(rect.height() - header_height);
                            tmp
                        };

                        // Header
                        if let Some(track_name) = track_name {
                            ui.painter().text(
                                header_rect.center(),
                                Align2::CENTER_CENTER,
                                track_name,
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

                        // Waveform

                        let waveform_points = channel_buffer
                            .iter()
                            .enumerate()
                            .map(|(index, &sample)| {
                                pos2(
                                    remap_clamp(
                                        index as f32,
                                        0.0..=(channel_buffer_length as f32 - 1.0),
                                        waveform_rect.x_range(),
                                    ),
                                    remap_clamp(
                                        sample.into(),
                                        SampleType::DISPLAY_RANGE,
                                        (waveform_rect.bottom() - 4.0)
                                            ..=(waveform_rect.top() + 4.0),
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

                        render_channel(channel_rect, channel_buffer.as_slice(), &self.track_name);

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
