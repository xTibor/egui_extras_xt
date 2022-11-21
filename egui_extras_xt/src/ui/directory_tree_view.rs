use std::path::{Path, PathBuf};

use egui::util::cache::{ComputerMut, FrameCache};
use egui::{CollapsingHeader, Response, Ui};
use itertools::Itertools;

// ----------------------------------------------------------------------------

type DirectoryViewCacheKey<'a> = &'a Path;
type DirectoryViewCacheValue = Vec<PathBuf>;

#[derive(Default)]
struct DirectoryViewComputer;

impl<'a> ComputerMut<DirectoryViewCacheKey<'a>, DirectoryViewCacheValue> for DirectoryViewComputer {
    fn compute(&mut self, key: DirectoryViewCacheKey) -> DirectoryViewCacheValue {
        println!("Computed: {:?}", key);
        std::fs::read_dir(key)
            .unwrap()
            .filter_map(Result::ok)
            .map(|dir_entry| dir_entry.path())
            .collect_vec()
    }
}

type DirectoryViewCache<'a> = FrameCache<DirectoryViewCacheValue, DirectoryViewComputer>;

// ----------------------------------------------------------------------------

pub trait DirectoryTreeView {
    fn directory_tree_view(&mut self, path: &mut Option<PathBuf>, root: &Path) -> Response;
}

impl DirectoryTreeView for Ui {
    fn directory_tree_view(
        &mut self,
        selected_path: &mut Option<PathBuf>,
        root: &Path,
    ) -> Response {
        fn render_directory(
            ui: &mut Ui,
            selected_path: &mut Option<PathBuf>,
            root: &Path,
            default_open: bool,
        ) {
            let directory_name = root.file_name().unwrap().to_str().unwrap();

            CollapsingHeader::new(format!("\u{1F5C0} {directory_name:}"))
                .default_open(default_open)
                .show(ui, |ui| {
                    let cached_directory_listing = {
                        let mut memory = ui.memory();
                        let cache = memory.caches.cache::<DirectoryViewCache<'_>>();
                        cache.get(root)
                    };

                    if !cached_directory_listing.is_empty() {
                        for path in cached_directory_listing {
                            if path.is_dir() {
                                render_directory(ui, selected_path, &path, false);
                            } else {
                                let file_name = path.file_name().unwrap().to_str().unwrap();
                                ui.selectable_value(
                                    selected_path,
                                    Some(path.clone()),
                                    format!("\u{1F5CB} {file_name:}"),
                                );
                            }
                        }
                    } else {
                        ui.weak("Empty directory");
                    }

                    //println!("show");
                });
        };

        render_directory(self, selected_path, root, true);

        self.scope(|ui| {}).response
    }
}
