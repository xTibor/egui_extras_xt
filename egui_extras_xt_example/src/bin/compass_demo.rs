use eframe::egui::{self, global_dark_light_mode_switch};
use egui_extras_xt::{CompassMarker, CompassMarkerShape, LinearCompass, PolarCompass};

use lazy_static::lazy_static;

struct GpsPosition(f32, f32);

impl GpsPosition {
    fn from_degrees(lat: f32, lon: f32) -> Self {
        Self(lat.to_radians(), lon.to_radians())
    }

    fn bearing_to(&self, target: &GpsPosition) -> f32 {
        let y = (target.1 - self.1).sin() * target.0.cos();
        let x = self.0.cos() * target.0.sin()
            - self.0.sin() * target.0.cos() * (target.1 - self.1).cos();
        y.atan2(x)
    }

    fn distance_to(&self, target: &GpsPosition) -> f32 {
        let a = ((target.0 - self.0) / 2.0).sin().powf(2.0)
            + self.0.cos() * target.0.cos() * ((target.1 - self.1) / 2.0).sin().powf(2.0);
        a.sqrt().atan2((1.0 - a).sqrt()) * 12734.0
    }
}

lazy_static! {
    #[rustfmt::skip]
    static ref TARGETS: [(&'static str, GpsPosition); 19] = [
        ("Szombathely",    GpsPosition::from_degrees(47.2307, 16.6212)),
        ("Zalaegerszeg",   GpsPosition::from_degrees(46.8331, 16.8469)),
        ("Győr",           GpsPosition::from_degrees(47.6744, 17.6492)),
        ("Kaposvár",       GpsPosition::from_degrees(46.3536, 17.7968)),
        ("Veszprém",       GpsPosition::from_degrees(47.0942, 17.9065)),
        ("Pécs",           GpsPosition::from_degrees(46.0763, 18.2280)),
        ("Tatabánya",      GpsPosition::from_degrees(47.5765, 18.3988)),
        ("Székesfehérvár", GpsPosition::from_degrees(47.1913, 18.4097)),
        ("Szekszárd",      GpsPosition::from_degrees(46.3495, 18.6990)),
        ("Budapest",       GpsPosition::from_degrees(47.4979, 19.0402)),
        ("Kecskemét",      GpsPosition::from_degrees(46.9080, 19.6931)),
        ("Salgótarján",    GpsPosition::from_degrees(48.0999, 19.8049)),
        ("Szeged",         GpsPosition::from_degrees(46.2507, 20.1516)),
        ("Szolnok",        GpsPosition::from_degrees(47.1769, 20.1843)),
        ("Eger",           GpsPosition::from_degrees(47.9026, 20.3771)),
        ("Miskolc",        GpsPosition::from_degrees(48.1032, 20.7779)),
        ("Békéscsaba",     GpsPosition::from_degrees(46.6747, 21.0864)),
        ("Debrecen",       GpsPosition::from_degrees(47.5314, 21.6242)),
        ("Nyíregyháza",    GpsPosition::from_degrees(47.9555, 21.7166)),
    ];
}

struct CompassExampleApp {
    heading: f32,
    gps_position: GpsPosition,
}

impl Default for CompassExampleApp {
    fn default() -> Self {
        Self {
            heading: 0.0,
            gps_position: GpsPosition::from_degrees(47.0829, 17.9787),
        }
    }
}

impl eframe::App for CompassExampleApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                global_dark_light_mode_switch(ui);
                ui.heading("Compass widgets");

                if ui.button("Reset").clicked() {
                    *self = Self::default();
                }
            });

            ui.separator();

            ui.horizontal(|ui| {
                ui.drag_angle(&mut self.gps_position.0);
                ui.drag_angle(&mut self.gps_position.1);
            });

            let markers = TARGETS
                .iter()
                .map(|(name, target_gps_position)| {
                    CompassMarker::new(self.gps_position.bearing_to(target_gps_position))
                        .distance(self.gps_position.distance_to(target_gps_position))
                        .label(name)
                })
                .collect::<Vec<CompassMarker>>();

            ui.add(
                PolarCompass::new(&mut self.heading)
                    .interactive(true)
                    .markers(&markers)
                    .diameter(512.0)
                    .show_marker_labels(true)
                    .show_marker_lines(true)
                    .default_marker_shape(CompassMarkerShape::Star(5, 0.5))
                    .max_distance(1000.0),
            );

            ui.add(
                LinearCompass::new(&mut self.heading)
                    .interactive(true)
                    .width(512.0 + 24.0 * 2.0)
                    .default_marker_shape(CompassMarkerShape::Star(5, 0.5))
                    .markers(&markers),
            );
        });
    }
}

fn main() {
    let options = eframe::NativeOptions::default();

    eframe::run_native(
        "Compass widgets",
        options,
        Box::new(|_cc| Box::new(CompassExampleApp::default())),
    );
}
