use eframe::egui::{self, Style, Visuals};
use eframe::emath::vec2;
use egui_extras_xt::ui::hyperlink_with_icon::HyperlinkWithIcon;

struct CargoPkgInfo {
    name: String,
    version: String,
    authors: Vec<(String, Option<String>)>,
    description: Option<String>,
    homepage: Option<String>,
    repository: Option<String>,
    license: Option<String>,
}

struct AboutDemoApp {
    pkg_info: CargoPkgInfo,
}

macro_rules! option_env_some {
    ( $x:expr ) => {
        match option_env!($x) {
            Some("") => None,
            opt => opt,
        }
    };
}

impl Default for AboutDemoApp {
    fn default() -> Self {
        Self {
            pkg_info: CargoPkgInfo {
                name: env!("CARGO_PKG_NAME").to_owned(),
                version: env!("CARGO_PKG_VERSION").to_owned(),
                authors: env!("CARGO_PKG_AUTHORS")
                    .split(':')
                    .map(|author_line| {
                        let author_parts = author_line
                            .split(|c| ['<', '>'].contains(&c))
                            .map(str::trim)
                            .collect::<Vec<_>>();

                        (
                            author_parts[0].to_owned(),
                            author_parts.get(1).map(|&s| s.to_owned()),
                        )
                    })
                    .collect::<Vec<(String, Option<String>)>>(),
                description: option_env_some!("CARGO_PKG_DESCRIPTION").map(|s| s.to_owned()),
                license: option_env_some!("CARGO_PKG_LICENSE").map(|s| s.to_owned()),
                homepage: option_env_some!("CARGO_PKG_HOMEPAGE").map(|s| s.to_owned()),
                repository: option_env_some!("CARGO_PKG_REPOSITORY").map(|s| s.to_owned()),
            },
        }
    }
}

impl eframe::App for AboutDemoApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {});

        egui::Window::new("About").show(ctx, |ui| {
            ui.heading(&self.pkg_info.name);
            ui.label(format!("Version {}", self.pkg_info.version));

            ui.separator();

            if let Some(description) = &self.pkg_info.description {
                ui.label(description);
                ui.separator();
            }

            ui.horizontal(|ui| {
                if let Some(homepage) = &self.pkg_info.homepage {
                    ui.hyperlink_with_icon_to("Home page", homepage);
                }

                if let Some(repository) = &self.pkg_info.repository {
                    ui.hyperlink_with_icon_to("Repository", repository);
                }
            });

            ui.separator();

            ui.collapsing("Authors", |ui| {
                ui.horizontal(|ui| {
                    for (author_name, author_email) in &self.pkg_info.authors {
                        if let Some(author_email) = author_email {
                            ui.hyperlink_with_icon_to(
                                author_name,
                                format!("mailto:{author_email:}"),
                            );
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
                if let Some(license) = &self.pkg_info.license {
                    ui.separator();
                    ui.label(format!("License: {license:}"));
                };
            });
        });
    }
}

fn main() {
    // TODO: Move to egui_extras_xt/src/ui as a reusable component.

    let options = eframe::NativeOptions {
        initial_window_size: Some(vec2(580.0, 680.0)),
        ..Default::default()
    };

    eframe::run_native(
        "About demo",
        options,
        Box::new(|cc| {
            cc.egui_ctx.set_style(Style {
                visuals: Visuals::dark(),
                ..Style::default()
            });

            Box::new(AboutDemoApp::default())
        }),
    );
}
