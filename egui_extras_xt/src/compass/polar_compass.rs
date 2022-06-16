use std::f32::consts::TAU;

use egui::{Color32, Response, Ui, Widget};

use crate::common::{normalized_angle_unsigned_excl, Winding, WrapMode};
use crate::compass::{CompassLabels, CompassMarkerShape};

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

pub struct PolarCompassMarker {
    angle: f32,
    distance: f32,
    shape: CompassMarkerShape,
    color: Option<Color32>,
}

impl<'a> PolarCompassMarker {
    pub fn new(angle: f32, distance: f32) -> Self {
        Self {
            angle: normalized_angle_unsigned_excl(angle),
            distance,
            shape: CompassMarkerShape::Square,
            color: None,
        }
    }

    pub fn shape(mut self, shape: CompassMarkerShape) -> Self {
        self.shape = shape;
        self
    }

    pub fn color(mut self, color: Color32) -> Self {
        self.color = Some(color);
        self
    }
}

// ----------------------------------------------------------------------------

#[must_use = "You should put this widget in an ui with `ui.add(widget);`"]
pub struct PolarCompass<'a> {
    get_set_value: GetSetValue<'a>,
    wrap: WrapMode,
    winding: Winding,
    diameter: f32,
    labels: CompassLabels<'a>,
    label_height: f32,
    snap: Option<f32>,
    shift_snap: Option<f32>,
    max_distance: f32,
    markers: &'a [PolarCompassMarker],
}

impl<'a> PolarCompass<'a> {
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
            wrap: WrapMode::Unsigned,
            winding: Winding::Clockwise,
            diameter: 256.0,
            labels: ["N", "E", "S", "W"],
            label_height: 48.0,
            snap: None,
            shift_snap: Some(TAU / 36.0),
            max_distance: 10000.0,
            markers: &[],
        }
    }

    pub fn wrap(mut self, wrap: WrapMode) -> Self {
        self.wrap = wrap;
        self
    }

    pub fn winding(mut self, winding: Winding) -> Self {
        self.winding = winding;
        self
    }

    pub fn diameter(mut self, diameter: impl Into<f32>) -> Self {
        self.diameter = diameter.into();
        self
    }

    pub fn labels(mut self, labels: CompassLabels<'a>) -> Self {
        self.labels = labels;
        self
    }

    pub fn label_height(mut self, label_height: impl Into<f32>) -> Self {
        self.label_height = label_height.into();
        self
    }

    pub fn max_distance(mut self, max_distance: impl Into<f32>) -> Self {
        self.max_distance = max_distance.into();
        self
    }

    pub fn snap(mut self, snap: Option<f32>) -> Self {
        self.snap = snap;
        self
    }

    pub fn shift_snap(mut self, shift_snap: Option<f32>) -> Self {
        self.shift_snap = shift_snap;
        self
    }

    pub fn markers(mut self, markers: &'a [PolarCompassMarker]) -> Self {
        self.markers = markers;
        self
    }
}

impl<'a> Widget for PolarCompass<'a> {
    fn ui(mut self, ui: &mut Ui) -> Response {
        todo!()
    }
}
