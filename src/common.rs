use std::f32::consts::TAU;

use eframe::emath::Rot2;

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum KnobOrientation {
    Right,
    Bottom,
    Left,
    Top,
    Custom(f32),
}

impl KnobOrientation {
    pub fn rot2(&self) -> Rot2 {
        match *self {
            Self::Right => Rot2::from_angle(TAU * 0.00),
            Self::Bottom => Rot2::from_angle(TAU * 0.25),
            Self::Left => Rot2::from_angle(TAU * 0.50),
            Self::Top => Rot2::from_angle(TAU * 0.75),
            Self::Custom(angle) => Rot2::from_angle(angle),
        }
    }
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum KnobDirection {
    Clockwise,
    Counterclockwise,
}

impl KnobDirection {
    pub fn to_float(&self) -> f32 {
        match *self {
            Self::Clockwise => 1.0,
            Self::Counterclockwise => -1.0,
        }
    }
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum KnobMode {
    Signed,
    Unsigned,
    SpinAround,
}

/// Wrap angle to `(0..TAU)` range.
pub fn normalized_angle_unsigned_excl(angle: f32) -> f32 {
    ((angle % TAU) + TAU) % TAU
}

/// Wrap angle to `(0..=TAU)` range.
pub fn normalized_angle_unsigned_incl(angle: f32) -> f32 {
    if angle < 0.0 {
        ((angle % TAU) + TAU) % TAU
    } else if angle > TAU {
        angle % TAU
    } else {
        angle
    }
}
