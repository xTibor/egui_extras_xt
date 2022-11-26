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
                .map(|s| s.to_lowercase());

            #[allow(clippy::match_same_arms)]
            match file_extension.as_deref() {
                // Plain text
                Some(
                    "asm" | "c" | "conf" | "cpp" | "css" | "glsl" | "h" | "htm" | "html" | "inc"
                    | "inf" | "ini" | "js" | "json" | "log" | "lua" | "md" | "pas" | "pp" | "py"
                    | "rs" | "s" | "toml" | "txt" | "xml" | "yml",
                ) => '\u{1F5B9}',
                // Rich text
                Some("doc" | "docx" | "pdf" | "rtf") => '\u{1F5BB}',
                // Images
                Some(
                    "bmp" | "gif" | "jpe" | "jpeg" | "jpg" | "jxl" | "kra" | "pam" | "pbm" | "pgm"
                    | "png" | "pnm" | "ppm" | "qoi" | "svg" | "svgz" | "webp",
                ) => '\u{1F5BC}',
                // Video
                Some("avi" | "mkv" | "mp4" | "ogv" | "webm") => '\u{1F39E}',
                // Audio
                Some(
                    "flac" | "it" | "m4a" | "mid" | "mmp" | "mmpz" | "mod" | "mp3" | "mscz" | "oga"
                    | "ogg" | "opus" | "s3m" | "sid" | "sng" | "wav" | "wma" | "xm",
                ) => '\u{266B}',
                // Archives
                Some("7z" | "arj" | "cab" | "gz" | "rar" | "tar" | "wad" | "xz" | "zip") => {
                    '\u{1F4E6}'
                }
                // SoundFont files
                Some("sbk" | "sf2" | "sf3" | "sfark" | "dls") => '\u{1F4E6}',
                // Compact disc images
                Some("iso") => '\u{1F4BF}',
                // Floppy disk images
                Some("d64" | "dsk") => '\u{1F4BE}',
                // Video game ROM images
                Some("3ds" | "cia" | "gba" | "nds" | "nes" | "sfc" | "smc") => '\u{1F3AE}',
                // Video game save files
                Some("sav" | "save") => '\u{1F4BE}',
                // Video game patch files
                Some("bps" | "ips" | "ups") => '\u{229E}',
                // Harddisk images
                Some("hdi" | "hdm" | "vdi") => '\u{1F5B4}',
                // Fonts
                Some("otb" | "otf" | "ttf" | "woff" | "woff2") => '\u{1F5DB}',
                // Binaries
                Some(
                    "appimage" | "bat" | "com" | "dll" | "exe" | "love" | "o" | "ppu" | "ps1"
                    | "sh" | "so",
                ) => '\u{2699}',
                // Unknown
                _ => '\u{1F5CB}',
            }
        }
    }
}
