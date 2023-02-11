use std::ffi::OsStr;
use std::path::{Path, PathBuf};

use eframe::egui::{self, Ui};
use eframe::emath::vec2;
use egui_extras_xt::filesystem::{BreadcrumbBar, DirectoryTreeViewWidget};

struct FilesystemWidgetsExample {
    root_path: PathBuf,
    selected_path: Option<PathBuf>,
}

impl Default for FilesystemWidgetsExample {
    fn default() -> Self {
        Self {
            root_path: PathBuf::from(".").canonicalize().unwrap(),
            selected_path: None,
        }
    }
}

impl FilesystemWidgetsExample {
    fn directory_filter(path: &Path) -> bool {
        !path
            .file_name()
            .and_then(OsStr::to_str)
            .unwrap()
            .starts_with('.')
    }

    fn file_extensions() -> &'static [&'static str] {
        &["rs", "toml"]
    }

    fn directory_context_menu_contents(ui: &mut Ui, path: &Path) {
        ui.strong("Directory context menu");
        ui.label(path.to_str().unwrap());
    }

    fn directory_context_menu_enabled(_path: &Path) -> bool {
        true
    }

    fn file_context_menu_contents(ui: &mut Ui, path: &Path) {
        ui.strong("File context menu");
        ui.label(path.to_str().unwrap());
    }

    fn file_context_menu_enabled(_path: &Path) -> bool {
        true
    }

    fn directory_hover_ui_contents(ui: &mut Ui, path: &Path) {
        ui.strong("Directory hover ui");
        ui.label(path.to_str().unwrap());
    }

    fn directory_hover_ui_enabled(_path: &Path) -> bool {
        true
    }

    fn file_hover_ui_contents(ui: &mut Ui, path: &Path) {
        ui.strong("File hover ui");
        ui.label(path.to_str().unwrap());
    }

    fn file_hover_ui_enabled(_path: &Path) -> bool {
        true
    }
}

impl eframe::App for FilesystemWidgetsExample {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if let Some(selected_path) = &mut self.selected_path {
                let breadcrum_bar_response = ui.add(
                    BreadcrumbBar::new(selected_path, &self.root_path)
                        .directory_filter(Self::directory_filter)
                        .file_extensions(Self::file_extensions())
                        .directory_context_menu(
                            Self::directory_context_menu_contents,
                            Self::directory_context_menu_enabled,
                        )
                        .file_context_menu(
                            Self::file_context_menu_contents,
                            Self::file_context_menu_enabled,
                        )
                        .directory_hover_ui(
                            Self::directory_hover_ui_contents,
                            Self::directory_hover_ui_enabled,
                        )
                        .file_hover_ui(Self::file_hover_ui_contents, Self::file_hover_ui_enabled)
                        .hide_file_extensions(false),
                );
                ui.separator();

                if breadcrum_bar_response.changed() {
                    println!("New path selected: {:?}", selected_path);
                }
            }

            let directory_tree_response = ui.add(
                DirectoryTreeViewWidget::new(&mut self.selected_path, &self.root_path)
                    .directory_filter(Self::directory_filter)
                    .file_extensions(Self::file_extensions())
                    .directory_selectable(true)
                    .file_selectable(true)
                    .directory_context_menu(
                        Self::directory_context_menu_contents,
                        Self::directory_context_menu_enabled,
                    )
                    .file_context_menu(
                        Self::file_context_menu_contents,
                        Self::file_context_menu_enabled,
                    )
                    .directory_hover_ui(
                        Self::directory_hover_ui_contents,
                        Self::directory_hover_ui_enabled,
                    )
                    .file_hover_ui(Self::file_hover_ui_contents, Self::file_hover_ui_enabled)
                    .hide_file_extensions(false)
                    .force_selected_open(false),
            );

            if directory_tree_response.changed() {
                println!("New path selected: {:?}", self.selected_path);
            }
        });
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        initial_window_size: Some(vec2(320.0, 480.0)),
        ..Default::default()
    };

    eframe::run_native(
        "Filesystem widgets example",
        options,
        Box::new(|_| Box::<FilesystemWidgetsExample>::default()),
    )
}
