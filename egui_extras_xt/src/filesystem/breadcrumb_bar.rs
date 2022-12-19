use std::ffi::OsStr;
use std::path::{Path, PathBuf};

use egui::{Label, Sense, Ui};
use itertools::Itertools;

use crate::filesystem::path_symbol::PathSymbol;

// TODO: Turn into proper widget
pub fn breadcrumb_bar(ui: &mut Ui, path: &mut PathBuf, root_directory: &Path) {
    ui.horizontal(|ui| {
        let path_cloned = path.clone();
        let components = path_cloned.components().collect_vec();

        for (path_prefix_index, path_prefix) in (0..components.len())
            .map(|n| components[..=n].iter())
            .map(PathBuf::from_iter)
            .enumerate()
            .filter(|(_, path_prefix)| path_prefix.starts_with(root_directory))
        {
            let component_label = {
                let component_symbol = path_prefix.symbol();
                let component_name = path_prefix
                    .file_name()
                    .map(OsStr::to_string_lossy)
                    .unwrap_or_default();
                format!("{component_symbol} {component_name}")
            };

            let mut response = ui.add(Label::new(component_label).sense(Sense::click()));

            if path_prefix.is_dir() {
                response = response.context_menu(|ui| {
                    let _ = ui.button(format!("Contents of {:?}", &path_prefix));
                });
            }

            if response.clicked() {
                *path = path_prefix.clone();
            }

            if path_prefix_index < components.len() - 1 {
                ui.add(Label::new("\u{23F5}"));
            }
        }
    });
}
