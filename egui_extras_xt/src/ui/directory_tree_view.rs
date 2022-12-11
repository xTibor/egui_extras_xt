use std::borrow::Borrow;
use std::ffi::OsStr;
use std::io;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use egui::collapsing_header::CollapsingState;
use egui::util::cache::{ComputerMut, FrameCache};
use egui::{Align, InnerResponse, Response, ScrollArea, Ui, Widget};
use itertools::Itertools;

use crate::ui::path_symbol::PathSymbol;

// ----------------------------------------------------------------------------

type DirectoryTreeViewCacheKey<'a> = &'a Path;
type DirectoryTreeViewCacheValue = Arc<io::Result<Vec<PathBuf>>>;

#[derive(Default)]
struct DirectoryTreeViewComputer;

impl<'a> ComputerMut<DirectoryTreeViewCacheKey<'a>, DirectoryTreeViewCacheValue>
    for DirectoryTreeViewComputer
{
    fn compute(&mut self, key: DirectoryTreeViewCacheKey) -> DirectoryTreeViewCacheValue {
        Arc::new(std::fs::read_dir(key).map(|read_dir| {
            read_dir
                .filter_map(Result::ok)
                .map(|dir_entry| dir_entry.path())
                .sorted_by_key(|path| {
                    (
                        path.is_file(),
                        path.file_name().unwrap().to_string_lossy().to_lowercase(),
                    )
                })
                .collect_vec()
        }))
    }
}

type DirectoryTreeViewCache<'a> =
    FrameCache<DirectoryTreeViewCacheValue, DirectoryTreeViewComputer>;

// ----------------------------------------------------------------------------

pub trait DirectoryTreeView {
    fn directory_tree_view(
        &mut self,
        selected_path: &mut Option<PathBuf>,
        root_directory: &Path,
    ) -> Response;
}

impl DirectoryTreeView for Ui {
    fn directory_tree_view(
        &mut self,
        selected_path: &mut Option<PathBuf>,
        root_directory: &Path,
    ) -> Response {
        self.add(DirectoryTreeViewWidget::new(selected_path, root_directory))
    }
}

// ----------------------------------------------------------------------------

type DirectoryTreeFilter<'a> = Box<dyn Fn(&Path) -> bool + 'a>;

type DirectoryTreeContextMenu<'a> = (
    Box<dyn Fn(&mut Ui, &Path) + 'a>,
    Box<dyn Fn(&Path) -> bool + 'a>,
);

type DirectoryTreeHoverUi<'a> = (
    Box<dyn Fn(&mut Ui, &Path) + 'a>,
    Box<dyn Fn(&Path) -> bool + 'a>,
);

// ----------------------------------------------------------------------------

#[must_use = "You should put this widget in an ui with `ui.add(widget);`"]
pub struct DirectoryTreeViewWidget<'a> {
    selected_path: &'a mut Option<PathBuf>,
    root_directory: &'a Path,
    force_selected_open: bool,
    hide_file_extensions: bool,

    file_selectable: bool,
    file_filter: Option<DirectoryTreeFilter<'a>>,
    file_context_menu: Option<DirectoryTreeContextMenu<'a>>,
    file_hover_ui: Option<DirectoryTreeHoverUi<'a>>,

    directory_selectable: bool,
    directory_filter: Option<DirectoryTreeFilter<'a>>,
    directory_context_menu: Option<DirectoryTreeContextMenu<'a>>,
    directory_hover_ui: Option<DirectoryTreeHoverUi<'a>>,
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
                self.show_directory(ui, self.root_directory, true)
                    .unwrap_or_else(|| ui.scope(|_| {}).response) // Null response
            })
            .inner
    }
}

impl<'a> DirectoryTreeViewWidget<'a> {
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

        // TODO: Reimplement force_selected_open
        let open_state = if self.force_selected_open {
            self.selected_path
                .as_mut()
                .map(|selected_path| selected_path.starts_with(directory_path))
        } else {
            None
        };

        let collapsing_state_id = ui.make_persistent_id(directory_path);

        let (_button_response, header_response, body_response) =
            CollapsingState::load_with_default_open(ui.ctx(), collapsing_state_id, default_open)
                .show_header(ui, |ui| {
                    let mut response = if self.directory_selectable {
                        ui.selectable_value(
                            self.selected_path,
                            Some(directory_path.to_path_buf()),
                            directory_label,
                        )
                    } else {
                        // TODO: Toggle CollapsingState on label click
                        ui.label(directory_label)
                    };

                    if let Some((add_contents_fn, enabled_fn)) = &self.directory_context_menu {
                        if enabled_fn(directory_path) {
                            response =
                                response.context_menu(|ui| add_contents_fn(ui, directory_path));
                        }
                    }

                    if let Some((add_contents_fn, enabled_fn)) = &self.directory_hover_ui {
                        if enabled_fn(directory_path) {
                            response =
                                response.on_hover_ui(|ui| add_contents_fn(ui, directory_path));
                        }
                    }

                    response
                })
                .body(|ui| {
                    let cached_directory_listing = {
                        let mut memory = ui.memory();
                        let cache = memory.caches.cache::<DirectoryTreeViewCache<'_>>();
                        cache.get(directory_path)
                    };

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
            Some(header_response.inner.union(body_response))
        } else {
            Some(header_response.inner)
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

        let mut response = if self.file_selectable {
            ui.selectable_value(
                self.selected_path,
                Some(file_path.to_path_buf()),
                file_label,
            )
        } else {
            ui.label(file_label)
        };

        if let Some((add_contents_fn, enabled_fn)) = &self.file_context_menu {
            if enabled_fn(file_path) {
                response = response.context_menu(|ui| add_contents_fn(ui, file_path));
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
                    response.scroll_to_me(Some(Align::Center));
                }
            }
        }

        Some(response)
    }
}
