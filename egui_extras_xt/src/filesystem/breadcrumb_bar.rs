use std::ffi::OsStr;
use std::path::{Path, PathBuf};

use egui::{Label, Response, Sense, Ui, Widget};
use itertools::Itertools;

use crate::filesystem::path_symbol::PathSymbol;
use crate::filesystem::{DirectoryContextMenu, DirectoryFilter, DirectoryHoverUi};

// ----------------------------------------------------------------------------

#[must_use = "You should put this widget in an ui with `ui.add(widget);`"]
pub struct BreadcrumbBar<'a> {
    selected_path: &'a mut PathBuf,
    root_directory: &'a Path,
    hide_file_extensions: bool,

    file_filter: Option<DirectoryFilter<'a>>,
    file_context_menu: Option<DirectoryContextMenu<'a>>,
    file_hover_ui: Option<DirectoryHoverUi<'a>>,

    directory_filter: Option<DirectoryFilter<'a>>,
    directory_context_menu: Option<DirectoryContextMenu<'a>>,
    directory_hover_ui: Option<DirectoryHoverUi<'a>>,
}

impl<'a> BreadcrumbBar<'a> {
    pub fn new(selected_path: &'a mut PathBuf, root_directory: &'a Path) -> Self {
        Self {
            selected_path,
            root_directory,
            hide_file_extensions: false,

            file_filter: None,
            file_context_menu: None,
            file_hover_ui: None,

            directory_filter: None,
            directory_context_menu: None,
            directory_hover_ui: None,
        }
    }

    pub fn hide_file_extensions(mut self, hide_file_extensions: bool) -> Self {
        self.hide_file_extensions = hide_file_extensions;
        self
    }

    pub fn file_extensions(self, file_extensions: &'a [&'a str]) -> Self {
        self.file_filter(|path| {
            if let Some(file_extension) = path
                .extension()
                .and_then(OsStr::to_str)
                .map(str::to_lowercase)
            {
                file_extensions.contains(&file_extension.as_str())
            } else {
                false
            }
        })
    }

    pub fn file_filter(mut self, filter: impl Fn(&Path) -> bool + 'a) -> Self {
        self.file_filter = Some(Box::new(filter));
        self
    }

    pub fn file_context_menu(
        mut self,
        add_contents: impl Fn(&mut Ui, &Path) + 'a,
        enabled: impl Fn(&Path) -> bool + 'a,
    ) -> Self {
        self.file_context_menu = Some((Box::new(add_contents), Box::new(enabled)));
        self
    }

    pub fn file_hover_ui(
        mut self,
        add_contents: impl Fn(&mut Ui, &Path) + 'a,
        enabled: impl Fn(&Path) -> bool + 'a,
    ) -> Self {
        self.file_hover_ui = Some((Box::new(add_contents), Box::new(enabled)));
        self
    }

    pub fn directory_filter(mut self, filter: impl Fn(&Path) -> bool + 'a) -> Self {
        self.directory_filter = Some(Box::new(filter));
        self
    }

    pub fn directory_context_menu(
        mut self,
        add_contents: impl Fn(&mut Ui, &Path) + 'a,
        enabled: impl Fn(&Path) -> bool + 'a,
    ) -> Self {
        self.directory_context_menu = Some((Box::new(add_contents), Box::new(enabled)));
        self
    }

    pub fn directory_hover_ui(
        mut self,
        add_contents: impl Fn(&mut Ui, &Path) + 'a,
        enabled: impl Fn(&Path) -> bool + 'a,
    ) -> Self {
        self.directory_hover_ui = Some((Box::new(add_contents), Box::new(enabled)));
        self
    }
}

// ----------------------------------------------------------------------------

impl<'a> Widget for BreadcrumbBar<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        ui.horizontal(|ui| {
            let path_cloned = self.selected_path.clone();
            let components = path_cloned.components().collect_vec();

            for (path_prefix_index, path_prefix) in (0..components.len())
                .map(|n| components[..=n].iter())
                .map(PathBuf::from_iter)
                .enumerate()
                .filter(|(_, path_prefix)| path_prefix.starts_with(self.root_directory))
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
                    if let Some((hover_ui_contents, hover_ui_enabled)) = &self.directory_hover_ui {
                        if hover_ui_enabled(&path_prefix) {
                            response =
                                response.on_hover_ui(|ui| hover_ui_contents(ui, &path_prefix));
                        }
                    }

                    if let Some((context_menu_contents, context_menu_enabled)) =
                        &self.directory_context_menu
                    {
                        response = response.context_menu(|ui| {
                            if context_menu_enabled(&path_prefix) {
                                context_menu_contents(ui, &path_prefix);
                                ui.separator();
                            }

                            if ui
                                .button(format!("Contents of {:?}", path_prefix))
                                .clicked()
                            {
                                ui.close_menu();
                            }
                        });
                    }
                } else {
                    if let Some((hover_ui_contents, hover_ui_enabled)) = &self.file_hover_ui {
                        if hover_ui_enabled(&path_prefix) {
                            response =
                                response.on_hover_ui(|ui| hover_ui_contents(ui, &path_prefix));
                        }
                    }

                    if let Some((context_menu_contents, context_menu_enabled)) =
                        &self.file_context_menu
                    {
                        if context_menu_enabled(&path_prefix) {
                            response =
                                response.context_menu(|ui| context_menu_contents(ui, &path_prefix));
                        }
                    }
                }

                if response.clicked() {
                    *self.selected_path = path_prefix.clone();
                    response.mark_changed();
                }

                if path_prefix_index < components.len() - 1 {
                    ui.add(Label::new("\u{23F5}"));
                }
            }
        });

        // TODO: response
        ui.scope(|_| {}).response
    }
}
