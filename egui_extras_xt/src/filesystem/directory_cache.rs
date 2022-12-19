use std::io;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use egui::util::cache::{ComputerMut, FrameCache};
use itertools::Itertools;

// ----------------------------------------------------------------------------

type DirectoryCacheKey<'a> = &'a Path;
type DirectoryCacheValue = Arc<io::Result<Vec<PathBuf>>>;

pub type DirectoryCache<'a> = FrameCache<DirectoryCacheValue, DirectoryCacheComputer>;

// ----------------------------------------------------------------------------

#[derive(Default)]
pub struct DirectoryCacheComputer;

impl<'a> ComputerMut<DirectoryCacheKey<'a>, DirectoryCacheValue> for DirectoryCacheComputer {
    fn compute(&mut self, key: DirectoryCacheKey) -> DirectoryCacheValue {
        Arc::new(std::fs::read_dir(key).map(|read_dir| {
            read_dir
                .filter_map(Result::ok)
                .map(|dir_entry| dir_entry.path())
                .sorted_by_key(|path| {
                    (
                        !path.is_dir(),
                        path.file_name().unwrap().to_string_lossy().to_lowercase(),
                    )
                })
                .collect_vec()
        }))
    }
}
