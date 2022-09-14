use egui::{vec2, Align2, FontSelection, Response, Sense, Ui, Widget};

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

#[must_use = "You should put this widget in an ui with `ui.add(widget);`"]
pub struct WaveformDisplayWidget<'a, Sample> {
    get_set_value: GetSetValue<'a>,
    buffer: Option<&'a [Sample]>,
    buffer_layout: BufferLayout,
    channels: usize,
    trigger_mode: TriggerMode,
    width: f32,
    height: f32,
    label: Option<String>,
}

impl<'a, Sample> WaveformDisplayWidget<'a, Sample> {
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
            width: 256.0,
            height: 64.0,
            label: None,
        }
    }

    pub fn buffer(mut self, buffer: &'a [Sample]) -> Self {
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

impl<'a, Sample> Widget for WaveformDisplayWidget<'a, Sample> {
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

            if let Some(label) = self.label {
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
                    font_id,
                    foreground_color,
                );
            }

            ui.painter().line_segment(
                [rect.left_center(), rect.right_center()],
                ui.style().visuals.noninteractive().fg_stroke,
            )
        };

        response
    }
}
