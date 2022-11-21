use std::path::{Path, PathBuf};

use egui::util::cache::{ComputerMut, FrameCache};
use egui::{CollapsingHeader, Response, ScrollArea, Ui};
use itertools::Itertools;

// ----------------------------------------------------------------------------

type DirectoryTreeViewCacheKey<'a> = &'a Path;
type DirectoryTreeViewCacheValue = Vec<PathBuf>;

#[derive(Default)]
struct DirectoryTreeViewComputer;

impl<'a> ComputerMut<DirectoryTreeViewCacheKey<'a>, DirectoryTreeViewCacheValue>
    for DirectoryTreeViewComputer
{
    fn compute(&mut self, key: DirectoryTreeViewCacheKey) -> DirectoryTreeViewCacheValue {
        println!("Computed: {:?}", key);
        std::fs::read_dir(key)
            .unwrap()
            .filter_map(Result::ok)
            // TODO: filter here
            .map(|dir_entry| dir_entry.path())
            .sorted_by_key(|path| path.is_file()) //(path.is_dir(), &path))
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
    ) -> Response;
}

impl DirectoryTreeView for Ui {
    fn directory_tree_view(
        &mut self,
        selected_path: &mut Option<PathBuf>,
        root: &Path,
        directory_filter: Option<DirectoryTreeViewFilter>,
        file_filter: Option<DirectoryTreeViewFilter>,
    ) -> Response {
        fn render_directory(
            ui: &mut Ui,
            selected_path: &mut Option<PathBuf>,
            root: &Path,
            directory_filter: &Option<DirectoryTreeViewFilter>,
            file_filter: &Option<DirectoryTreeViewFilter>,
            default_open: bool,
        ) {
            let directory_name = root.file_name().unwrap().to_str().unwrap();

            CollapsingHeader::new(format!("\u{1F5C0} {directory_name:}"))
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
                        for path in filtered_directory_listing {
                            if path.is_dir() {
                                render_directory(
                                    ui,
                                    selected_path,
                                    &path,
                                    &directory_filter,
                                    &file_filter,
                                    false,
                                );
                            } else {
                                render_file(ui, selected_path, &path);
                            }
                        }
                    } else {
                        ui.weak("Empty directory");
                    }
                });
        }

        fn render_file(ui: &mut Ui, selected_path: &mut Option<PathBuf>, file_path: &Path) {
            let file_name = file_path.file_name().unwrap().to_str().unwrap();

            ui.selectable_value(
                selected_path,
                Some(file_path.to_path_buf()),
                format!("\u{1F5CB} {file_name:}"),
            );
        }

        ScrollArea::vertical().show(self, |ui| {
            render_directory(
                ui,
                selected_path,
                root,
                &directory_filter,
                &file_filter,
                true,
            );
        });

        self.scope(|ui| {}).response
    }
}
