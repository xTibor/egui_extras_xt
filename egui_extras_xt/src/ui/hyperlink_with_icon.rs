use egui::{Hyperlink, Response, Ui, Widget};

pub trait HyperlinkWithIcon {
    fn hyperlink_with_icon(&mut self, url: impl ToString) -> Response;
    fn hyperlink_with_icon_to(&mut self, label: impl ToString, url: impl ToString) -> Response;
}

fn hyperlink_icon(url: &str) -> char {
    for &(u, icon) in &[
        // Warnings
        ("ftp:", '\u{26A0}'),
        ("http:", '\u{26A0}'),
        // URI schemes
        ("fb:", '\u{E604}'),
        ("file:", '\u{1F5C1}'),
        ("geo:", '\u{1F5FA}'),
        ("git:", '\u{E625}'),
        ("irc:", '\u{1F4AC}'),
        ("irc6:", '\u{1F4AC}'),
        ("ircs:", '\u{1F4AC}'),
        ("itms-apps:", '\u{F8FF}'),
        ("mailto:", '\u{1F4E7}'),
        ("maps:", '\u{1F5FA}'),
        ("market:", '\u{E618}'),
        ("message:", '\u{1F4E7}'),
        ("ms-", '\u{E61F}'), // Not a typo.
        ("nfs:", '\u{1F5C1}'),
        ("sftp:", '\u{1F5C1}'),
        ("skype:", '\u{E613}'),
        ("sms:", '\u{2709}'),
        ("ssh:", '\u{1F5A5}'),
        ("steam:", '\u{E623}'),
        ("tel:", '\u{1F4DE}'),
        // Websites
        ("https://apps.apple.com/", '\u{F8FF}'),
        ("https://crates.io/", '\u{1F4E6}'),
        ("https://drive.google.com/", '\u{E62F}'),
        ("https://github.com/", '\u{E624}'),
        ("https://gitlab.com/", '\u{E625}'),
        ("https://play.google.com/store/apps/", '\u{E618}'),
        ("https://stackoverflow.com/", '\u{E601}'),
        ("https://store.steampowered.com/", '\u{E623}'),
        ("https://twitter.com/", '\u{E603}'),
        ("https://www.dropbox.com/", '\u{E610}'),
        ("https://www.facebook.com/", '\u{E604}'),
        ("https://www.instagram.com/", '\u{E60F}'),
        ("https://www.paypal.com/", '\u{E616}'),
        ("https://www.youtube.com/", '\u{E636}'),
        // Discord and friends have no symbols in the default font.
    ] {
        if url.starts_with(u) {
            return icon;
        }
    }

    if url.starts_with("https://") {
        for &(u, icon) in &[(".github.io/", '\u{E624}'), (".reddit.com/", '\u{E628}')] {
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
