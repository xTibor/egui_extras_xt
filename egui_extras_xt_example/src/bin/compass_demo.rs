use std::f32::consts::TAU;

use eframe::egui::{self, global_dark_light_mode_switch, DragValue, Response};
use eframe::epaint::color::Hsva;
use eframe::epaint::Color32;
use egui_extras_xt::{CompassMarker, LinearCompass, PolarCompass};

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
        ("Szombathely",    GpsPosition::from_degrees(47.230795, 16.621286)),
        ("Zalaegerszeg",   GpsPosition::from_degrees(46.833170, 16.846919)),
        ("Győr",           GpsPosition::from_degrees(47.674452, 17.649232)),
        ("Kaposvár",       GpsPosition::from_degrees(46.353691, 17.796872)),
        ("Veszprém",       GpsPosition::from_degrees(47.094285, 17.906540)),
        ("Pécs",           GpsPosition::from_degrees(46.076389, 18.228098)),
        ("Tatabánya",      GpsPosition::from_degrees(47.576595, 18.398840)),
        ("Székesfehérvár", GpsPosition::from_degrees(47.191392, 18.409764)),
        ("Szekszárd",      GpsPosition::from_degrees(46.349588, 18.699044)),
        ("Budapest",       GpsPosition::from_degrees(47.497870, 19.040246)),
        ("Kecskemét",      GpsPosition::from_degrees(46.908010, 19.693137)),
        ("Salgótarján",    GpsPosition::from_degrees(48.099903, 19.804960)),
        ("Szeged",         GpsPosition::from_degrees(46.250742, 20.151697)),
        ("Szolnok",        GpsPosition::from_degrees(47.176968, 20.184351)),
        ("Eger",           GpsPosition::from_degrees(47.902627, 20.377112)),
        ("Miskolc",        GpsPosition::from_degrees(48.103211, 20.777915)),
        ("Békéscsaba",     GpsPosition::from_degrees(46.674736, 21.086400)),
        ("Debrecen",       GpsPosition::from_degrees(47.531371, 21.624222)),
        ("Nyíregyháza",    GpsPosition::from_degrees(47.955528, 21.716693)),
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
            gps_position: GpsPosition::from_degrees(47.082944, 17.978775),
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
                let mut drag_angle_slow = |radians: &mut f32| -> Response {
                    let mut degrees = radians.to_degrees();
                    let mut response = ui.add(DragValue::new(&mut degrees).speed(0.01).suffix("°"));

                    if degrees != radians.to_degrees() {
                        *radians = degrees.to_radians();
                        response.changed = true;
                    }

                    response
                };

                drag_angle_slow(&mut self.gps_position.0);
                drag_angle_slow(&mut self.gps_position.1);
            });

            let markers = TARGETS
                .iter()
                .map(|(name, target_gps_position)| {
                    CompassMarker::new(self.gps_position.bearing_to(target_gps_position))
                        .distance(self.gps_position.distance_to(target_gps_position))
                        .label(name)
                        // TODO: PolarCompass.default_color(DefaultColor::HueByBearing)
                        .color(Color32::from(Hsva::new(
                            self.gps_position.bearing_to(target_gps_position) / TAU,
                            1.0,
                            1.0,
                            1.0,
                        )))
                })
                .collect::<Vec<CompassMarker>>();

            ui.add(
                PolarCompass::new(&mut self.heading)
                    .interactive(true)
                    .markers(&markers)
                    .diameter(512.0)
                    .show_marker_labels(true)
                    .show_marker_lines(true)
                    .max_distance(1000.0),
            );

            ui.add(
                LinearCompass::new(&mut self.heading)
                    .interactive(true)
                    .width(512.0 + 24.0 * 2.0)
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
