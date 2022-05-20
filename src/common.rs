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
