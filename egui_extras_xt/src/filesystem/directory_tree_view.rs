use std::borrow::Borrow;
use std::ffi::OsStr;
use std::path::{Path, PathBuf};

use egui::collapsing_header::{paint_default_icon, CollapsingState};
use egui::{
    Align, InnerResponse, Label, Layout, Response, ScrollArea, Sense, TextWrapMode, Ui, Widget,
};
use itertools::Itertools;

use crate::filesystem::directory_cache::DirectoryCache;
use crate::filesystem::path_symbol::PathSymbol;
use crate::filesystem::{DirectoryContextMenu, DirectoryFilter, DirectoryHoverUi};

// ----------------------------------------------------------------------------

#[must_use = "You should put this widget in an ui with `ui.add(widget);`"]
pub struct DirectoryTreeViewWidget<'a> {
    selected_path: &'a mut Option<PathBuf>,
    root_directory: &'a Path,
    force_selected_open: bool,
    hide_file_extensions: bool,

    file_selectable: bool,
    file_filter: Option<DirectoryFilter<'a>>,
    file_context_menu: Option<DirectoryContextMenu<'a>>,
    file_hover_ui: Option<DirectoryHoverUi<'a>>,

    directory_selectable: bool,
    directory_filter: Option<DirectoryFilter<'a>>,
    directory_context_menu: Option<DirectoryContextMenu<'a>>,
    directory_hover_ui: Option<DirectoryHoverUi<'a>>,
}

impl<'a> DirectoryTreeViewWidget<'a> {
    pub fn new(selected_path: &'a mut Option<PathBuf>, root_directory: &'a Path) -> Self {
        Self {
            selected_path,
            root_directory,
            force_selected_open: false,
            hide_file_extensions: false,

            file_selectable: true,
            file_filter: None,
            file_context_menu: None,
            file_hover_ui: None,

            directory_selectable: false,
            directory_filter: None,
            directory_context_menu: None,
            directory_hover_ui: None,
        }
    }

    pub fn force_selected_open(mut self, force_selected_open: bool) -> Self {
        self.force_selected_open = force_selected_open;
        self
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

    pub fn file_selectable(mut self, file_selectable: bool) -> Self {
        self.file_selectable = file_selectable;
        self
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

    pub fn directory_selectable(mut self, directory_selectable: bool) -> Self {
        self.directory_selectable = directory_selectable;
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

impl<'a> Widget for DirectoryTreeViewWidget<'a> {
    fn ui(mut self, ui: &mut Ui) -> Response {
        ScrollArea::both()
            .auto_shrink([false, false])
            .show(ui, |ui| {
                ui.style_mut().wrap_mode = Some(TextWrapMode::Extend);
                ui.with_layout(Layout::top_down_justified(Align::LEFT), |ui| {
                    self.show_directory(ui, self.root_directory, true)
                        .unwrap_or_else(|| ui.scope(|_| {}).response) // Null response
                })
                .inner
            })
            .inner
    }
}

impl<'a> DirectoryTreeViewWidget<'a> {
    #[allow(clippy::collapsible_if)]
    fn show_directory(
        &mut self,
        ui: &mut Ui,
        directory_path: &Path,
        default_open: bool,
    ) -> Option<Response> {
        let directory_label = {
            let directory_name = if directory_path.parent().is_none() {
                "Root directory"
            } else {
                directory_path.file_name().and_then(OsStr::to_str).unwrap()
            };
            let directory_symbol = directory_path.symbol();

            format!("{directory_symbol:} {directory_name:}")
        };

        let mut collapsing_state = CollapsingState::load_with_default_open(
            ui.ctx(),
            ui.make_persistent_id(directory_path),
            default_open,
        );

        if let Some(selected_path) = self.selected_path {
            if self.force_selected_open {
                collapsing_state.set_open(selected_path.starts_with(directory_path));
            }
        }

        let mut header_response = ui
            .horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = 0.0;
                collapsing_state.show_toggle_button(ui, paint_default_icon);

                ui.with_layout(Layout::top_down_justified(Align::LEFT), |ui| {
                    if self.directory_selectable {
                        let response = ui.selectable_value(
                            self.selected_path,
                            Some(directory_path.to_path_buf()),
                            directory_label,
                        );

                        if response.double_clicked() {
                            collapsing_state.toggle(ui);
                        }

                        response
                    } else {
                        // TODO: Same font and metrics as selectable_value
                        let response = ui.add(
                            Label::new(directory_label)
                                .selectable(false)
                                .sense(Sense::click()),
                        );

                        if response.clicked() {
                            collapsing_state.toggle(ui);
                        }

                        response
                    }
                })
                .inner
            })
            .inner;

        if let Some(selected_path) = self.selected_path {
            if self.force_selected_open {
                if selected_path == directory_path {
                    // TODO: egui 0.20 does not allow specifying separate scrolling alignment per axis.
                    // Alignment should be (Horizontal::Left, Vertical::Center) here.
                    header_response.scroll_to_me(Some(Align::Center));
                }
            }
        }

        if let Some((add_contents_fn, enabled_fn)) = &self.directory_context_menu {
            if enabled_fn(directory_path) {
                header_response
                    .context_menu(|ui| add_contents_fn(ui, directory_path))
                    .map(|resp| {
                        header_response = resp.response;
                    });
            }
        }

        if let Some((add_contents_fn, enabled_fn)) = &self.directory_hover_ui {
            if enabled_fn(directory_path) {
                header_response =
                    header_response.on_hover_ui(|ui| add_contents_fn(ui, directory_path));
            }
        }

        let body_response = collapsing_state.show_body_indented(&header_response, ui, |ui| {
            let cached_directory_listing = ui.memory_mut(|mem| {
                let cache = mem.caches.cache::<DirectoryCache<'_>>();
                cache.get(directory_path)
            });

            match cached_directory_listing.borrow() {
                Ok(cached_directory_listing) => {
                    let filtered_directory_listing = cached_directory_listing
                        .iter()
                        .filter(|path| {
                            #[allow(clippy::collapsible_else_if)]
                            if path.is_dir() {
                                if let Some(directory_filter) = &self.directory_filter {
                                    directory_filter(path)
                                } else {
                                    true
                                }
                            } else {
                                if let Some(file_filter) = &self.file_filter {
                                    file_filter(path)
                                } else {
                                    true
                                }
                            }
                        })
                        .collect_vec();

                    if !filtered_directory_listing.is_empty() {
                        filtered_directory_listing
                            .iter()
                            .filter_map(|path| {
                                if path.is_dir() {
                                    self.show_directory(ui, path, false)
                                } else {
                                    self.show_file(ui, path)
                                }
                            })
                            .reduce(|result, response| result.union(response))
                    } else {
                        ui.weak("Empty directory");
                        None
                    }
                }

                Err(err) => {
                    ui.weak(format!("\u{1F525} {err}"));
                    None
                }
            }
        });

        if let Some(InnerResponse {
            inner: Some(body_response),
            ..
        }) = body_response
        {
            Some(header_response.union(body_response))
        } else {
            Some(header_response)
        }
    }

    #[allow(clippy::unnecessary_wraps)] // Necessary wrap, false warning
    fn show_file(&mut self, ui: &mut Ui, file_path: &Path) -> Option<Response> {
        let file_label = {
            let file_name = if self.hide_file_extensions {
                file_path.file_stem().and_then(OsStr::to_str).unwrap()
            } else {
                file_path.file_name().and_then(OsStr::to_str).unwrap()
            };
            let file_symbol = file_path.symbol();

            format!("{file_symbol:} {file_name:}")
        };

        let mut response = ui
            .with_layout(Layout::top_down_justified(Align::LEFT), |ui| {
                if self.file_selectable {
                    ui.selectable_value(
                        self.selected_path,
                        Some(file_path.to_path_buf()),
                        file_label,
                    )
                } else {
                    // TODO: Same font and metrics as selectable_value
                    ui.add(Label::new(file_label))
                }
            })
            .inner;

        if let Some((add_contents_fn, enabled_fn)) = &self.file_context_menu {
            if enabled_fn(file_path) {
                response
                    .context_menu(|ui| add_contents_fn(ui, file_path))
                    .map(|resp| {
                        response = resp.response;
                    });
            }
        }

        if let Some((add_contents_fn, enabled_fn)) = &self.file_hover_ui {
            if enabled_fn(file_path) {
                response = response.on_hover_ui(|ui| add_contents_fn(ui, file_path));
            }
        }

        if self.force_selected_open {
            if let Some(selected_path) = self.selected_path {
                if selected_path == file_path {
                    // TODO: egui 0.20 does not allow specifying separate scrolling alignment per axis.
                    // Alignment should be (Horizontal::Left, Vertical::Center) here.
                    response.scroll_to_me(Some(Align::Center));
                }
            }
        }

        Some(response)
    }
}
