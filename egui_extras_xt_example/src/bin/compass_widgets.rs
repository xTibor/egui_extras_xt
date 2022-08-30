use eframe::egui::{self, global_dark_light_mode_switch};
use eframe::emath::vec2;

use egui_extras_xt::compasses::{CompassMarker, CompassMarkerShape, LinearCompass, PolarCompass};

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

struct CompassWidgetsExample {
    heading: f32,
    gps_position: GpsPosition,
    targets: Vec<(GpsPosition, String)>,
}

impl Default for CompassWidgetsExample {
    fn default() -> Self {
        macro_rules! target {
            ($lat:expr, $lon:expr, $name:expr) => {
                (GpsPosition::from_degrees($lat, $lon), String::from($name))
            };
        }

        Self {
            heading: 0.0,
            gps_position: GpsPosition::from_degrees(47.0829, 17.9787),
            #[rustfmt::skip]
            targets: vec![
                target!(47.2307, 16.6212, "Szombathely"   ),
                target!(46.8331, 16.8469, "Zalaegerszeg"  ),
                target!(47.6744, 17.6492, "Győr"          ),
                target!(46.3536, 17.7968, "Kaposvár"      ),
                target!(47.0942, 17.9065, "Veszprém"      ),
                target!(46.0763, 18.2280, "Pécs"          ),
                target!(47.5765, 18.3988, "Tatabánya"     ),
                target!(47.1913, 18.4097, "Székesfehérvár"),
                target!(46.3495, 18.6990, "Szekszárd"     ),
                target!(47.4979, 19.0402, "Budapest"      ),
                target!(46.9080, 19.6931, "Kecskemét"     ),
                target!(48.0999, 19.8049, "Salgótarján"   ),
                target!(46.2507, 20.1516, "Szeged"        ),
                target!(47.1769, 20.1843, "Szolnok"       ),
                target!(47.9026, 20.3771, "Eger"          ),
                target!(48.1032, 20.7779, "Miskolc"       ),
                target!(46.6747, 21.0864, "Békéscsaba"    ),
                target!(47.5314, 21.6242, "Debrecen"      ),
                target!(47.9555, 21.7166, "Nyíregyháza"   ),
            ],
        }
    }
}

impl eframe::App for CompassWidgetsExample {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                global_dark_light_mode_switch(ui);
                ui.heading("Compass widgets example");

                if ui.button("Reset").clicked() {
                    *self = Self::default();
                }
            });

            ui.separator();

            ui.horizontal(|ui| {
                ui.drag_angle(&mut self.gps_position.0);
                ui.drag_angle(&mut self.gps_position.1);
            });

            let markers = self
                .targets
                .iter()
                .map(|(target_gps_position, target_name)| {
                    CompassMarker::new(self.gps_position.bearing_to(target_gps_position))
                        .distance(self.gps_position.distance_to(target_gps_position))
                        .label(target_name)
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
    let options = eframe::NativeOptions {
        initial_window_size: Some(vec2(580.0, 680.0)),
        ..Default::default()
    };

    eframe::run_native(
        "Compass widgets example",
        options,
        Box::new(|_| Box::new(CompassWidgetsExample::default())),
    );
}
