use std::path::{Path, PathBuf};

use egui::util::cache::{ComputerMut, FrameCache};
use egui::{Align, CollapsingHeader, Response, ScrollArea, Ui};
use itertools::Itertools;

use crate::ui::path_symbol::PathSymbol;

// ----------------------------------------------------------------------------

type DirectoryTreeViewCacheKey<'a> = &'a Path;
type DirectoryTreeViewCacheValue = Vec<PathBuf>;

#[derive(Default)]
struct DirectoryTreeViewComputer;

impl<'a> ComputerMut<DirectoryTreeViewCacheKey<'a>, DirectoryTreeViewCacheValue>
    for DirectoryTreeViewComputer
{
    fn compute(&mut self, key: DirectoryTreeViewCacheKey) -> DirectoryTreeViewCacheValue {
        std::fs::read_dir(key)
            .unwrap()
            .filter_map(Result::ok)
            .map(|dir_entry| dir_entry.path())
            .sorted_by_key(|path| {
                (
                    path.is_file(),
                    path.file_name().unwrap().to_string_lossy().to_lowercase(),
                )
            })
            .collect_vec()
    }
}

type DirectoryTreeViewCache<'a> =
    FrameCache<DirectoryTreeViewCacheValue, DirectoryTreeViewComputer>;

// ----------------------------------------------------------------------------

type DirectoryTreeViewFilter = Box<dyn Fn(&Path) -> bool>;

// ----------------------------------------------------------------------------

pub trait DirectoryTreeView {
    fn directory_tree_view(
        &mut self,
        selected_path: &mut Option<PathBuf>,
        root: &Path,
        directory_filter: Option<DirectoryTreeViewFilter>,
        file_filter: Option<DirectoryTreeViewFilter>,
        force_selected_open: bool,
    ) -> Response;
}

impl DirectoryTreeView for Ui {
    fn directory_tree_view(
        &mut self,
        selected_path: &mut Option<PathBuf>,
        root: &Path,
        directory_filter: Option<DirectoryTreeViewFilter>,
        file_filter: Option<DirectoryTreeViewFilter>,
        force_selected_open: bool,
    ) -> Response {
        fn render_directory(
            ui: &mut Ui,
            selected_path: &mut Option<PathBuf>,
            root: &Path,
            directory_filter: &Option<DirectoryTreeViewFilter>,
            file_filter: &Option<DirectoryTreeViewFilter>,
            default_open: bool,
            force_selected_open: bool,
        ) -> Option<Response> {
            let directory_name = root.file_name().unwrap().to_str().unwrap();
            let directory_symbol = root.symbol();

            let open_state = if force_selected_open {
                if let Some(selected_path) = selected_path {
                    Some(selected_path.starts_with(root))
                } else {
                    None
                }
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
                        cache.get(root)
                    };

                    let filtered_directory_listing = cached_directory_listing
                        .iter()
                        .filter(|path| {
                            if path.is_dir() {
                                if let Some(directory_filter) = &directory_filter {
                                    directory_filter(path)
                                } else {
                                    true
                                }
                            } else {
                                if let Some(file_filter) = &file_filter {
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
                            .map(|path| {
                                if path.is_dir() {
                                    render_directory(
                                        ui,
                                        selected_path,
                                        &path,
                                        &directory_filter,
                                        &file_filter,
                                        false,
                                        force_selected_open,
                                    )
                                } else {
                                    render_file(ui, selected_path, &path, force_selected_open)
                                }
                            })
                            .flatten()
                            .reduce(|result, response| result.union(response))
                    } else {
                        ui.weak("Empty directory");
                        None
                    }
                })
                .body_returned
                .unwrap_or(None)
        }

        fn render_file(
            ui: &mut Ui,
            selected_path: &mut Option<PathBuf>,
            file_path: &Path,
            force_selected_open: bool,
        ) -> Option<Response> {
            let file_name = file_path.file_name().unwrap().to_str().unwrap();
            let file_symbol = file_path.symbol();

            // egui bug (0.19.0): https://github.com/emilk/egui/pull/2343
            let response = ui.selectable_value(
                selected_path,
                Some(file_path.to_path_buf()),
                format!("{file_symbol:} {file_name:}"),
            );

            if force_selected_open {
                if let Some(selected_path) = selected_path {
                    if selected_path == file_path {
                        response.scroll_to_me(Some(Align::Center));
                    }
                }
            }

            Some(response)
        }

        ScrollArea::both()
            .auto_shrink([false, false])
            .show(self, |ui| {
                render_directory(
                    ui,
                    selected_path,
                    root,
                    &directory_filter,
                    &file_filter,
                    true,
                    force_selected_open,
                )
                .unwrap_or_else(|| ui.scope(|_| {}).response) // Null response
            })
            .inner
    }
}
