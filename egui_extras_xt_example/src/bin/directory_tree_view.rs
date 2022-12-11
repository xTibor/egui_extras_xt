use std::ffi::OsStr;
use std::path::PathBuf;

use eframe::egui;
use eframe::emath::vec2;
use egui_extras_xt::ui::directory_tree_view::DirectoryTreeViewWidget;

struct DirectoryTreeViewExample {
    root_path: PathBuf,
    selected_path: Option<PathBuf>,
}

impl Default for DirectoryTreeViewExample {
    fn default() -> Self {
        Self {
            root_path: PathBuf::from(".").canonicalize().unwrap(),
            selected_path: None,
        }
    }
}

impl eframe::App for DirectoryTreeViewExample {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if let Some(selected_path) = self.selected_path.as_ref() {
                ui.label(format!(
                    "Selected path: {}",
                    selected_path.as_os_str().to_str().unwrap()
                ));
                ui.separator();
            }

            let directory_response = ui.add(
                DirectoryTreeViewWidget::new(&mut self.selected_path, &self.root_path)
                    .directory_filter(|path| {
                        !path
                            .file_name()
                            .and_then(OsStr::to_str)
                            .unwrap()
                            .starts_with('.')
                    })
                    .file_extensions(&["rs", "toml"])
                    .directory_context_menu(
                        |ui, path| {
                            ui.strong("Directory context menu");
                            ui.label(path.to_str().unwrap());
                        },
                        |_path| true,
                    )
                    .file_context_menu(
                        |ui, path| {
                            ui.strong("File context menu");
                            ui.label(path.to_str().unwrap());
                        },
                        |_path| true,
                    )
                    .directory_hover_ui(
                        |ui, path| {
                            ui.strong("Directory hover ui");
                            ui.label(path.to_str().unwrap());
                        },
                        |_path| true,
                    )
                    .file_hover_ui(
                        |ui, path| {
                            ui.strong("File hover ui");
                            ui.label(path.to_str().unwrap());
                        },
                        |_path| true,
                    )
                    .hide_file_extensions(false)
                    .force_selected_open(false),
            );

            if directory_response.changed() {
                println!("New path selected: {:?}", self.selected_path);
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
        "Directory tree view example",
        options,
        Box::new(|_| Box::<DirectoryTreeViewExample>::default()),
    );
}
