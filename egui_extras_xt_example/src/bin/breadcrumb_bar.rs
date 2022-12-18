use std::ffi::OsStr;
use std::path::PathBuf;

use eframe::egui::{self, Label, Sense};
use eframe::emath::vec2;
use egui_extras_xt::ui::path_symbol::PathSymbol;
use itertools::Itertools;

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
                let path_cloned = self.path.clone();
                let path_components = path_cloned.components().collect_vec();

                for (path_component_index, path) in (0..path_components.len())
                    .map(|count| &path_components[..=count])
                    .map(|slice| slice.iter().collect::<PathBuf>())
                    .enumerate()
                {
                    let component_label = format!(
                        "{} {}",
                        path.symbol(),
                        path.file_name()
                            .map(OsStr::to_string_lossy)
                            .unwrap_or_default()
                    );

                    if ui
                        .add(Label::new(component_label).sense(Sense::click()))
                        .clicked()
                    {
                        self.path = path;
                    }

                    if path_component_index < path_components.len() - 1 {
                        ui.label("\u{23F5}");
                    }
                }
            });

            ui.separator();
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
