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
                let components = path_cloned.components().collect_vec();

                for (path_prefix_index, path_prefix) in (0..components.len())
                    .map(|n| components[..=n].iter())
                    .map(PathBuf::from_iter)
                    .enumerate()
                {
                    let component_label = {
                        let component_symbol = path_prefix.symbol();
                        let component_name = path_prefix
                            .file_name()
                            .map(OsStr::to_string_lossy)
                            .unwrap_or_default();
                        format!("{component_symbol} {component_name}")
                    };

                    if ui
                        .add(Label::new(component_label).sense(Sense::click()))
                        .clicked()
                    {
                        self.path = path_prefix.clone();
                    }

                    if path_prefix_index < components.len() - 1 {
                        if ui
                            .add(Label::new("\u{23F5}").sense(Sense::click()))
                            .clicked()
                        {
                            println!("List contents of {:?}", &path_prefix)
                        }
                    }
                }
            });

            ui.separator();

            if ui.button("\u{1F504} Reset").clicked() {
                *self = Self::default();
            }
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
