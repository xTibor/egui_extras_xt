use std::ops::RangeInclusive;

use egui::{self, vec2, Color32, Rect, Response, Sense, Ui, Widget};
use epaint::Stroke;
use itertools::Itertools;

use crate::piano::key_metrics::{
    PianoKeyColor, PianoKeyLogicalBounds, PIANO_KEY_METRICS, PIANO_OCTAVE_HEIGHT,
    PIANO_OCTAVE_WIDTH,
};

// ----------------------------------------------------------------------------

/// Combined into one function (rather than two) to make it easier
/// for the borrow checker.
type GetSetValue<'a> = Box<dyn 'a + FnMut(Option<f32>) -> f32>;

fn get(get_set_value: &mut GetSetValue<'_>) -> f32 {
    (get_set_value)(None)
}

fn set(get_set_value: &mut GetSetValue<'_>, value: f32) {
    (get_set_value)(Some(value));
}

// ----------------------------------------------------------------------------

#[must_use = "You should put this widget in an ui with `ui.add(widget);`"]
pub struct PianoWidget<'a> {
    get_set_value: GetSetValue<'a>,
    note_range: RangeInclusive<isize>,
}

impl<'a> PianoWidget<'a> {
    pub fn new(value: &'a mut f32) -> Self {
        Self::from_get_set(move |v: Option<f32>| {
            if let Some(v) = v {
                *value = v;
            }
            *value
        })
    }

    pub fn from_get_set(get_set_value: impl 'a + FnMut(Option<f32>) -> f32) -> Self {
        Self {
            get_set_value: Box::new(get_set_value),
            note_range: 60..=84,
        }
    }

    pub fn note_range(mut self, note_range: RangeInclusive<isize>) -> Self {
        self.note_range = note_range;
        self
    }
}

impl<'a> Widget for PianoWidget<'a> {
    fn ui(mut self, ui: &mut Ui) -> Response {
        let piano_start_x = {
            let (octave, pitch_class) =
                (self.note_range.start() / 12, self.note_range.start() % 12);
            PIANO_KEY_METRICS[pitch_class as usize].bounds(octave).left
        };

        let piano_end_x = {
            let (octave, pitch_class) = (self.note_range.end() / 12, self.note_range.end() % 12);
            PIANO_KEY_METRICS[pitch_class as usize].bounds(octave).right
        };

        let desired_size = vec2(
            (piano_end_x - piano_start_x) as f32,
            PIANO_OCTAVE_HEIGHT as f32,
        );

        let (rect, mut response) = ui.allocate_exact_size(desired_size, Sense::click_and_drag());

        if ui.is_rect_visible(rect) {
            for note in self.note_range {
                let (octave, pitch_class) = (note / 12, note % 12);

                let metrics = &PIANO_KEY_METRICS[pitch_class as usize];

                let PianoKeyLogicalBounds {
                    left,
                    top,
                    right,
                    bottom,
                } = metrics.bounds(octave);

                let r = Rect::from_min_size(
                    rect.left_top() + vec2(left as f32 - piano_start_x as f32, top as f32),
                    vec2(right as f32 - left as f32, bottom as f32 - top as f32),
                );

                ui.painter().rect(
                    r,
                    0.0,
                    match metrics.color {
                        PianoKeyColor::White => Color32::WHITE,
                        PianoKeyColor::Black => Color32::BLACK,
                    },
                    Stroke::new(2.0, Color32::BLACK),
                )
            }
        }

        response
    }
}
