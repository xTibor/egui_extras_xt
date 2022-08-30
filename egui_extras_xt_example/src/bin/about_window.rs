use eframe::egui;
use eframe::emath::vec2;

use egui_extras_xt::show_about_window;

#[derive(Default)]
struct AboutWindowExample {
    about_window_open: bool,
}

impl eframe::App for AboutWindowExample {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.toggle_value(&mut self.about_window_open, "About");
        });

        show_about_window!(ctx, &mut self.about_window_open);
    }
}

fn main() {
    let options = eframe::NativeOptions {
        initial_window_size: Some(vec2(640.0, 480.0)),
        ..Default::default()
    };

    eframe::run_native(
        "About Window Example",
        options,
        Box::new(|_| Box::new(AboutWindowExample::default())),
    );
}
