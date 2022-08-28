use egui::{Context, Window};

use crate::ui::hyperlink_with_icon::HyperlinkWithIcon;

// ----------------------------------------------------------------------------

pub struct PackageInfo {
    pub name: &'static str,
    pub version: &'static str,
    pub authors: &'static str,
    pub description: Option<&'static str>,
    pub homepage: Option<&'static str>,
    pub repository: Option<&'static str>,
    pub license: Option<&'static str>,
    pub license_file: Option<&'static str>,
}

#[macro_export]
macro_rules! package_info {
    () => {{
        macro_rules! option_env_some {
            ( $x:expr ) => {
                match option_env!($x) {
                    Some("") => None,
                    opt => opt,
                }
            };
        }

        $crate::ui::about_window::PackageInfo {
            name: env!("CARGO_PKG_NAME"),
            version: env!("CARGO_PKG_VERSION"),
            authors: env!("CARGO_PKG_AUTHORS"),
            description: option_env_some!("CARGO_PKG_DESCRIPTION"),
            homepage: option_env_some!("CARGO_PKG_HOMEPAGE"),
            repository: option_env_some!("CARGO_PKG_REPOSITORY"),
            license: option_env_some!("CARGO_PKG_LICENSE"),
            license_file: option_env_some!("CARGO_PKG_LICENSE_FILE"),
        }
    }};
}

impl PackageInfo {
    fn authors(&self) -> impl Iterator<Item = (&'static str, Option<&'static str>)> {
        self.authors.split(':').map(|author_line| {
            let author_parts = author_line
                .split(|c| ['<', '>'].contains(&c))
                .map(str::trim)
                .collect::<Vec<_>>();
            (author_parts[0], author_parts.get(1).cloned())
        })
    }
}

// ----------------------------------------------------------------------------

#[macro_export]
macro_rules! show_about_window {
    ($ctx:expr, $open:expr) => {{
        $crate::ui::about_window::show_about_window_impl($ctx, $open, $crate::package_info!());
    }};
}

pub fn show_about_window_impl(ctx: &Context, open: &mut bool, package_info: PackageInfo) {
    Window::new("About")
        .open(open)
        .resizable(false)
        .collapsible(false)
        .show(ctx, |ui| {
            ui.heading(package_info.name);
            ui.label(format!("Version {}", package_info.version));

            ui.separator();

            if let Some(description) = package_info.description {
                ui.label(description);
                ui.separator();
            }

            ui.horizontal(|ui| {
                if let Some(homepage) = package_info.homepage {
                    ui.hyperlink_with_icon_to("Home page", homepage);
                }

                if let Some(repository) = package_info.repository {
                    ui.hyperlink_with_icon_to("Repository", repository);
                }
            });

            ui.separator();

            ui.collapsing("Authors", |ui| {
                ui.horizontal(|ui| {
                    for (author_name, author_email) in package_info.authors() {
                        if let Some(author_email) = author_email {
                            if !["noreply@", "no-reply@", "@users.noreply."]
                                .iter()
                                .any(|no_reply| author_email.contains(no_reply))
                            {
                                ui.hyperlink_with_icon_to(
                                    author_name,
                                    format!("mailto:{author_email:}"),
                                );
                            } else {
                                ui.label(author_name);
                            }
                        } else {
                            ui.label(author_name);
                        }
                    }
                });

                // (!) Rust incremental compilation bug:
                // When the 'license' field is changed in the crate's Cargo.toml,
                // source files that include that field through `env!()` macros
                // are not picked up for recompilation.
                // Always do `cargo clean` + full rebuild when changing Cargo.toml metadata.
                if let Some(license) = package_info.license {
                    ui.separator();
                    ui.label(format!("License: {license:}"));
                };

                if let Some(license_file) = package_info.license_file {
                    ui.separator();
                    ui.label(format!(
                        "License: See the {license_file:} file for details."
                    ));
                };
            });
        });
}
