use std::path::PathBuf;

use eframe::egui::{self, Label, Sense};
use eframe::emath::vec2;
use egui_extras_xt::ui::path_symbol::PathSymbol;
use itertools::{Itertools, Position};

struct BreadcrumbBarExample {
    path: PathBuf,
}

impl Default for BreadcrumbBarExample {
    fn default() -> Self {
        Self {
            path: "/home/tibor/git/egui_extras_xt/Cargo.toml".into(),
        }
    }
}

impl eframe::App for BreadcrumbBarExample {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                for component in self.path.components().with_position() {
                    let component_symbol = if !self.path.is_dir()
                        && matches!(component, Position::Last(_) | Position::Only(_))
                    {
                        self.path.symbol()
                    } else {
                        component.into_inner().symbol()
                    };

                    let component_label = format!(
                        "{} {}",
                        component_symbol,
                        component.into_inner().as_os_str().to_string_lossy()
                    );

                    if ui
                        .add(Label::new(component_label).sense(Sense::click()))
                        .clicked()
                    {
                        println!("Clicked");
                    }

                    if matches!(component, Position::First(_) | Position::Middle(_)) {
                        ui.label("\u{23F5}");
                    }
                }
            })
        });
    }
}

fn main() {
    let options = eframe::NativeOptions {
        initial_window_size: Some(vec2(640.0, 480.0)),
        ..Default::default()
    };

    eframe::run_native(
        "Breadcrumb bar example",
        options,
        Box::new(|_| Box::<BreadcrumbBarExample>::default()),
    );
}
