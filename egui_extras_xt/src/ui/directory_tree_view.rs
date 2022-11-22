use std::ffi::OsStr;
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
        ) -> Option<Response> {
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
                                    )
                                } else {
                                    render_file(ui, selected_path, &path)
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
        ) -> Option<Response> {
            let file_name = file_path.file_name().unwrap().to_str().unwrap();
            let file_extension = file_path
                .extension()
                .map(OsStr::to_string_lossy)
                .map(String::from);

            let file_symbol = match file_extension.as_deref() {
                // Plain text
                Some(
                    "txt" | "rs" | "log" | "toml" | "md" | "py" | "c" | "h" | "cpp" | "js" | "html"
                    | "json" | "css" | "ini" | "inf" | "glsl",
                ) => '\u{1F5B9}',
                // Rich text
                Some("pdf" | "rtf" | "doc" | "docx") => '\u{1F5BB}',
                // Images
                Some(
                    "png" | "jpg" | "jpeg" | "bmp" | "kra" | "webp" | "gif" | "svg" | "ppm" | "pbm"
                    | "qoi",
                ) => '\u{1F5BC}',
                // Video
                Some("avi" | "mp4" | "webm" | "mkv" | "ogv") => '\u{1F39E}',
                // Audio
                Some(
                    "mp3" | "flac" | "wav" | "m4a" | "ogg" | "opus" | "wma" | "mid" | "xm" | "mod"
                    | "s3m" | "it" | "oga" | "mmp" | "mmpz" | "mscz",
                ) => '\u{266B}',
                // Archives
                Some("zip" | "7z" | "rar" | "tar" | "gz" | "xz" | "cab" | "arj" | "wad") => {
                    '\u{1F4E6}'
                }
                // SoundFont files
                Some("sbk" | "sf2" | "sf3" | "sfark" | "dls") => '\u{1F4E6}',
                // Compact disc images
                Some("iso") => '\u{1F4BF}',
                // Floppy disk images
                Some("dsk" | "d64") => '\u{1F4BE}',
                // Video game ROM images
                Some("gba" | "cia" | "3ds" | "nds" | "nes" | "smc" | "sfc") => '\u{1F3AE}',
                // Video game save files
                Some("sav") => '\u{1F4BE}',
                // Video game patch files
                Some("ips" | "bps" | "ups") => '\u{229E}',
                // Harddisk images
                Some("hdm" | "hdi" | "vdi") => '\u{1F5B4}',
                // Fonts
                Some("ttf" | "otf" | "otb" | "woff" | "woff2") => '\u{1F5DB}',
                // Executables
                Some("appimage" | "sh" | "so" | "exe" | "com" | "dll" | "bat" | "love") => {
                    '\u{2699}'
                }
                // Unknown
                _ => '\u{1F5CB}',
            };

            // egui bug (0.19.0): `selectable_value` returns `changed` responses
            // even when the supplied value has not changed when clicking the
            // button repeatedly.
            Some(ui.selectable_value(
                selected_path,
                Some(file_path.to_path_buf()),
                format!("{file_symbol:} {file_name:}"),
            ))
        }

        ScrollArea::vertical()
            .show(self, |ui| {
                render_directory(
                    ui,
                    selected_path,
                    root,
                    &directory_filter,
                    &file_filter,
                    true,
                )
                .unwrap_or_else(|| ui.scope(|_| {}).response) // Null response
            })
            .inner
    }
}
