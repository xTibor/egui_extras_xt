mod compass_marker;
mod linear_compass;
mod polar_compass;

pub use compass_marker::{CompassMarker, CompassMarkerShape, DefaultCompassMarkerColor};
pub use linear_compass::LinearCompass;
pub use polar_compass::{PolarCompass, PolarCompassOverflow};

// ----------------------------------------------------------------------------

pub type CompassLabels<'a> = [&'a str; 4];
