use egui::{Hyperlink, Response, Ui, Widget};

pub trait HyperlinkWithIcon {
    fn hyperlink_with_icon(&mut self, url: impl ToString) -> Response;
    fn hyperlink_with_icon_to(&mut self, label: impl ToString, url: impl ToString) -> Response;
}

#[rustfmt::skip]
fn hyperlink_icon(url: &str) -> char {
    for &(u, icon) in &[
        // Warnings
        ("ftp:",       '\u{26A0}' ),
        ("http:",      '\u{26A0}' ),
        ("telnet:",    '\u{26A0}' ),

        // URI schemes
        ("appstream:", '\u{1F4E6}'),
        ("apt:",       '\u{1F4E6}'),
        ("fax:",       '\u{1F4E0}'),
        ("fb:",        '\u{E604}' ),
        ("file:",      '\u{1F5C1}'),
        ("flatpak:",   '\u{1F4E6}'),
        ("gemini:",    '\u{264A}' ),
        ("geo:",       '\u{1F5FA}'),
        ("git:",       '\u{E625}' ),
        ("info:",      '\u{2139}' ),
        ("ipp:",       '\u{1F5B6}'),
        ("ipps:",      '\u{1F5B6}'),
        ("irc:",       '\u{1F4AC}'),
        ("irc6:",      '\u{1F4AC}'),
        ("ircs:",      '\u{1F4AC}'),
        ("itms-apps:", '\u{F8FF}' ),
        ("ldap:",      '\u{1F4D5}'),
        ("ldaps:",     '\u{1F4D5}'),
        ("mailto:",    '\u{1F4E7}'),
        ("maps:",      '\u{1F5FA}'),
        ("market:",    '\u{E618}' ),
        ("message:",   '\u{1F4E7}'),
        ("ms-",        '\u{E61F}' ), // Not a typo.
        ("nfs:",       '\u{1F5C1}'),
        ("pkg:",       '\u{1F4E6}'),
        ("rpm:",       '\u{1F4E6}'),
        ("sftp:",      '\u{1F5C1}'),
        ("sip:",       '\u{1F4DE}'),
        ("sips:",      '\u{1F4DE}'),
        ("skype:",     '\u{E613}' ),
        ("smb:",       '\u{1F5C1}'),
        ("sms:",       '\u{2709}' ),
        ("snap:",      '\u{1F4E6}'),
        ("ssh:",       '\u{1F5A5}'),
        ("steam:",     '\u{E623}' ),
        ("tel:",       '\u{1F4DE}'),

        // Websites
        ("https://apps.apple.com/",             '\u{F8FF}' ),
        ("https://crates.io/",                  '\u{1F4E6}'),
        ("https://docs.rs/",                    '\u{1F4DA}'),
        ("https://drive.google.com/",           '\u{E62F}' ),
        ("https://play.google.com/store/apps/", '\u{E618}' ),
        ("https://soundcloud.com/",             '\u{E627}' ),
        ("https://stackoverflow.com/",          '\u{E601}' ),
        ("https://steamcommunity.com/",         '\u{E623}' ),
        ("https://store.steampowered.com/",     '\u{E623}' ),
        ("https://twitter.com/",                '\u{E603}' ),
        ("https://vimeo.com/",                  '\u{E602}' ),
        ("https://www.dropbox.com/",            '\u{E610}' ),
        ("https://www.facebook.com/",           '\u{E604}' ),
        ("https://www.instagram.com/",          '\u{E60F}' ),
        ("https://www.paypal.com/",             '\u{E616}' ),
        ("https://www.youtube.com/",            '\u{E636}' ),
        ("https://youtu.be/",                   '\u{E636}' ),

        // Generic git rules
        ("https://git.",                        '\u{E625}' ),
        ("https://cgit.",                       '\u{E625}' ),
        ("https://gitlab.",                     '\u{E625}' ),

        // Non-exhaustive list of some git instances not covered by the generic rules
        ("https://bitbucket.org/",              '\u{E625}' ),
        ("https://code.qt.io/",                 '\u{E625}' ),
        ("https://code.videolan.org/",          '\u{E625}' ),
        ("https://framagit.org/",               '\u{E625}' ),
        ("https://gitee.com/",                  '\u{E625}' ),
        ("https://github.com/",                 '\u{E624}' ),
        ("https://invent.kde.org/",             '\u{E625}' ),
        ("https://salsa.debian.org/",           '\u{E625}' ),

        // Discord and friends have no symbols in the default emoji font.
    ] {
        if url.starts_with(u) {
            return icon;
        }
    }

    if url.starts_with("https://") {
        for &(u, icon) in &[
            (".github.io/",  '\u{E624}'),
            (".gitlab.io/",  '\u{E625}'),
            (".reddit.com/", '\u{E628}'),
        ] {
            if url.contains(u) {
                return icon;
            }
        }

        // Generic web link
        return '\u{1F30D}';
    }

    // Unknown link type
    '\u{2BA9}'
}

impl HyperlinkWithIcon for Ui {
    fn hyperlink_with_icon(&mut self, url: impl ToString) -> Response {
        Hyperlink::from_label_and_url(
            format!("{} {}", hyperlink_icon(&url.to_string()), url.to_string()),
            url,
        )
        .ui(self)
    }

    fn hyperlink_with_icon_to(&mut self, label: impl ToString, url: impl ToString) -> Response {
        Hyperlink::from_label_and_url(
            format!("{} {}", hyperlink_icon(&url.to_string()), label.to_string()),
            url,
        )
        .ui(self)
    }
}
