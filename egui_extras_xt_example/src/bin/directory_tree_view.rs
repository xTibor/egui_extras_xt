use std::ffi::OsStr;
use std::path::{Path, PathBuf};

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

            ui.directory_tree_view(
                &mut self.selected_path,
                &self.root_path,
                Some(Box::new(|path| path.ends_with(".rs"))),
            );
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
