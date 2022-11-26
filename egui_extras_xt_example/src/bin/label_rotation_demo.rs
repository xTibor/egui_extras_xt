use std::f64::consts::TAU;

use eframe::egui::{CentralPanel, Context, Direction, Layout, TextStyle};
use eframe::emath::vec2;
use eframe::epaint::color::Hsva;

use egui_extras_xt::ui::rotated_label::RotatedLabel;

struct LabelRotationDemo {
    angle: f32,
    hue: f32,
}

impl Default for LabelRotationDemo {
    fn default() -> Self {
        Self {
            angle: 0.0,
            hue: 0.0,
        }
    }
}

impl eframe::App for LabelRotationDemo {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ui.with_layout(Layout::centered_and_justified(Direction::TopDown), |ui| {
                ui.style_mut().override_text_style = Some(TextStyle::Heading);
                ui.style_mut().visuals.override_text_color =
                    Some(Hsva::new(self.hue, 1.0, 1.0, 1.0).into());

                ui.add(
                    RotatedLabel::new(
                        "You spin me right 'round, baby, right 'round\n\n\
                        Like a record, baby, right 'round, 'round, 'round",
                    )
                    .angle(self.angle),
                );
            });
        });

        self.angle = (ctx.input().time * TAU / 9.0) as f32;
        self.hue = ((ctx.input().time / 3.0) % 1.0) as f32;
        ctx.request_repaint();
    }
}

fn main() {
    let options = eframe::NativeOptions {
        initial_window_size: Some(vec2(500.0, 500.0)),
        ..Default::default()
    };

    eframe::run_native(
        "You spin me round (like a record)",
        options,
        Box::new(|_| Box::<LabelRotationDemo>::default()),
    );
}
