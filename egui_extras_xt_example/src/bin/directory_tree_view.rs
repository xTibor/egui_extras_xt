use std::ffi::OsStr;
use std::path::PathBuf;

use eframe::egui;
use eframe::emath::vec2;
use egui_extras_xt::ui::directory_tree_view::DirectoryTreeView;

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

            if DirectoryTreeView::new(&mut self.selected_path, &self.root_path)
                .directory_filter(Box::new(|path| {
                    !path.file_name().unwrap().to_str().unwrap().starts_with('.')
                }))
                .file_filter(Box::new(|path| path.extension() == Some(OsStr::new("rs"))))
                .force_selected_open(false)
                .show(ui)
                .changed()
            {
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
        Box::new(|_| Box::new(DirectoryTreeViewExample::default())),
    );
}
