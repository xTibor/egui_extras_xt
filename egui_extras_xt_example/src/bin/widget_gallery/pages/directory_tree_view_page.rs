use std::path::PathBuf;

use eframe::egui::{Grid, Ui};
use egui_extras_xt::filesystem::directory_tree_view::DirectoryTreeViewWidget;
use egui_extras_xt::ui::optional_value_widget::OptionalValueWidget;

use crate::pages::PageImpl;

use crate::pages::ui::pathbuf_ui;

pub struct DirectoryTreeViewPage {
    root_path: PathBuf,
    selected_path: Option<PathBuf>,
    force_selected_open: bool,
    hide_file_extensions: bool,
    file_selectable: bool,
    directory_selectable: bool,
}

impl Default for DirectoryTreeViewPage {
    fn default() -> DirectoryTreeViewPage {
        DirectoryTreeViewPage {
            root_path: PathBuf::from("/"),
            selected_path: None,
            force_selected_open: false,
            hide_file_extensions: false,
            file_selectable: true,
            directory_selectable: false,
        }
    }
}

impl PageImpl for DirectoryTreeViewPage {
    fn ui(&mut self, ui: &mut Ui) {
        ui.add_sized(
            [300.0, 300.0],
            DirectoryTreeViewWidget::new(&mut self.selected_path, &self.root_path)
                .force_selected_open(self.force_selected_open)
                .hide_file_extensions(self.hide_file_extensions)
                .file_selectable(self.file_selectable)
                .directory_selectable(self.directory_selectable),
        );
        ui.separator();

        Grid::new("directory_tree_view_properties")
            .num_columns(2)
            .spacing([20.0, 10.0])
            .striped(true)
            .show(ui, |ui| {
                ui.label("Root path");
                pathbuf_ui(ui, &mut self.root_path);
                ui.end_row();

                ui.label("Selected path");
                ui.optional_value_widget(&mut self.selected_path, pathbuf_ui);
                ui.end_row();

                ui.label("Force selected path open");
                ui.checkbox(&mut self.force_selected_open, "");
                ui.end_row();

                ui.label("Hide file extensions");
                ui.checkbox(&mut self.hide_file_extensions, "");
                ui.end_row();

                ui.label("Selectable files");
                ui.checkbox(&mut self.file_selectable, "");
                ui.end_row();

                ui.label("Selectable directories");
                ui.checkbox(&mut self.directory_selectable, "");
                ui.end_row();
            });
    }
}
