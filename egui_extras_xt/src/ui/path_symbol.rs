use std::ffi::OsStr;
use std::path::Path;

#[cfg(unix)]
use std::os::unix::prelude::PermissionsExt;

pub trait PathSymbol {
    fn symbol(&self) -> char;
}

impl PathSymbol for Path {
    fn symbol(&self) -> char {
        if self.is_symlink() {
            '\u{2BA9}'
        } else if self.is_dir() {
            '\u{1F5C0}'
        } else {
            // Executables
            #[cfg(unix)]
            if let Ok(metadata) = self.metadata() {
                if metadata.permissions().mode() & 0o111 != 0 {
                    return '\u{2699}';
                }
            }

            let file_extension = self
                .extension()
                .map(OsStr::to_string_lossy)
                .map(String::from);

            match file_extension.as_deref() {
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
                Some("sav" | "save") => '\u{1F4BE}',
                // Video game patch files
                Some("ips" | "bps" | "ups") => '\u{229E}',
                // Harddisk images
                Some("hdm" | "hdi" | "vdi") => '\u{1F5B4}',
                // Fonts
                Some("ttf" | "otf" | "otb" | "woff" | "woff2") => '\u{1F5DB}',
                // Executables (fallback)
                Some("appimage" | "sh" | "so" | "exe" | "com" | "dll" | "bat" | "love") => {
                    '\u{2699}'
                }
                // Unknown
                _ => '\u{1F5CB}',
            }
        }
    }
}
