use std::borrow::Borrow;
use std::ffi::OsStr;
use std::io;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use egui::util::cache::{ComputerMut, FrameCache};
use egui::{Align, CollapsingHeader, Response, ScrollArea, Ui, Widget};
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

// ----------------------------------------------------------------------------

#[must_use = "You should put this widget in an ui with `ui.add(widget);`"]
pub struct DirectoryTreeViewWidget<'a> {
    selected_path: &'a mut Option<PathBuf>,
    root_directory: &'a Path,
    directory_filter: Option<DirectoryTreeFilter<'a>>,
    file_filter: Option<DirectoryTreeFilter<'a>>,
    force_selected_open: bool,
    hide_file_extensions: bool,
}

impl<'a> DirectoryTreeViewWidget<'a> {
    pub fn new(selected_path: &'a mut Option<PathBuf>, root_directory: &'a Path) -> Self {
        Self {
            selected_path,
            root_directory,
            directory_filter: None,
            file_filter: None,
            force_selected_open: false,
            hide_file_extensions: false,
        }
    }

    pub fn directory_filter(mut self, directory_filter: impl Fn(&Path) -> bool + 'a) -> Self {
        self.directory_filter = Some(Box::new(directory_filter));
        self
    }

    pub fn file_filter(mut self, file_filter: impl Fn(&Path) -> bool + 'a) -> Self {
        self.file_filter = Some(Box::new(file_filter));
        self
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
        let directory_name = if directory_path.parent().is_none() {
            "Root directory"
        } else {
            directory_path.file_name().and_then(OsStr::to_str).unwrap()
        };

        let directory_symbol = directory_path.symbol();

        let open_state = if self.force_selected_open {
            self.selected_path
                .as_mut()
                .map(|selected_path| selected_path.starts_with(directory_path))
        } else {
            None
        };

        CollapsingHeader::new(format!("{directory_symbol:} {directory_name:}"))
            .open(open_state)
            .default_open(default_open)
            .show(ui, |ui| {
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
            })
            .body_returned
            .unwrap_or(None)
    }

    #[allow(clippy::unnecessary_wraps)] // Necessary wrap, false warning
    fn show_file(&mut self, ui: &mut Ui, file_path: &Path) -> Option<Response> {
        let file_name = if self.hide_file_extensions {
            file_path.file_stem().and_then(OsStr::to_str).unwrap()
        } else {
            file_path.file_name().and_then(OsStr::to_str).unwrap()
        };

        let file_symbol = file_path.symbol();

        // egui bug (0.19.0): https://github.com/emilk/egui/pull/2343
        let response = ui.selectable_value(
            self.selected_path,
            Some(file_path.to_path_buf()),
            format!("{file_symbol:} {file_name:}"),
        );

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
