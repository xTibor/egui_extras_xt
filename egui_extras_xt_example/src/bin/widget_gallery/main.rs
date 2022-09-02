mod pages;

use std::collections::HashMap;

use eframe::egui::panel::Side;
use eframe::egui::{self, TextEdit};
use eframe::emath::vec2;

use egui_extras_xt::show_about_window;

use itertools::Itertools;
use strum::{EnumProperty, IntoEnumIterator};

use pages::{PageId, PageImpl};

struct WidgetGallery {
    // Pages
    current_page: PageId,
    pages: HashMap<PageId, Box<dyn PageImpl>>,
    search_query: String,

    // Sub-windows
    settings_window_open: bool,
    inspector_window_open: bool,
    memory_window_open: bool,
    about_window_open: bool,
}

impl Default for WidgetGallery {
    fn default() -> Self {
        Self {
            // Pages
            current_page: PageId::WelcomePage,
            pages: HashMap::from_iter(
                PageId::iter().map(|page_id| (page_id, page_id.create_page())),
            ),
            search_query: "".to_owned(),

            // Sub-windows
            settings_window_open: false,
            inspector_window_open: false,
            memory_window_open: false,
            about_window_open: false,
        }
    }
}

impl eframe::App for WidgetGallery {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("mainmenu").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.menu_button("Debug", |ui| {
                    ui.checkbox(&mut self.settings_window_open, "\u{1F527} Settings");
                    ui.checkbox(&mut self.inspector_window_open, "\u{1F50D} Inspection");
                    ui.checkbox(&mut self.memory_window_open, "\u{1F4DD} Memory");
                });

                if ui.button("About").clicked() {
                    self.about_window_open = true;
                }
            });
        });

        // egui layout bug: SidePanel width gets progressively fucked when dragging
        // the main window between screens with different PPI.
        // SidePanel resizing is also fucked, it's mirroring mouse movements along
        // along the left edge of the window (SidePanel `.abs()` bug).
        egui::SidePanel::new(Side::Left, "sidepanel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.add(
                    TextEdit::singleline(&mut self.search_query)
                        .desired_width(150.0)
                        .hint_text("\u{1F50D} Search..."),
                );
                ui.add_enabled_ui(!self.search_query.is_empty(), |ui| {
                    // Default font doesn't have "\u{232B}"
                    if ui.button("\u{1F5D9}").clicked() {
                        self.search_query.clear();
                    }
                });
            });
            ui.separator();

            egui::ScrollArea::vertical().show(ui, |ui| {
                PageId::iter()
                    .map(|page_id| (page_id, page_id.get_str("feature")))
                    .filter(|(page_id, _feature)| {
                        let q = self.search_query.to_lowercase();
                        page_id.to_string().to_lowercase().contains(&q)
                    })
                    .sorted_by_key(|(_page_id, feature)| *feature)
                    .group_by(|(_page_id, feature)| *feature)
                    .into_iter()
                    .for_each(|(feature, pages)| {
                        if let Some(feature) = feature {
                            ui.label(format!("\u{1F4E6} {feature:}"));
                        }
                        pages
                            .sorted_by_key(|(page_id, _feature)| page_id.to_string())
                            .for_each(|(page_id, _feature)| {
                                ui.selectable_value(
                                    &mut self.current_page,
                                    page_id,
                                    page_id.to_string(),
                                );
                            });

                        ui.separator();
                    });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading(self.current_page.to_string());
            if let Some(feature) = self.current_page.get_str("feature") {
                ui.label(format!("\u{1F4E6} {feature:}"))
                    .on_hover_text("Cargo feature");
            }
            ui.separator();

            egui::ScrollArea::both().show(ui, |ui| {
                self.pages
                    .get_mut(&self.current_page)
                    .expect("failed to get page")
                    .ui(ui);

                ui.separator();

                if ui.button("\u{1F504} Reset").clicked() {
                    self.pages
                        .insert(self.current_page, self.current_page.create_page());
                }
            });
        });

        egui::Window::new("\u{1F527} Settings")
            .open(&mut self.settings_window_open)
            .vscroll(true)
            .show(ctx, |ui| {
                ctx.settings_ui(ui);
            });

        egui::Window::new("\u{1F50D} Inspection")
            .open(&mut self.inspector_window_open)
            .vscroll(true)
            .show(ctx, |ui| {
                ctx.inspection_ui(ui);
            });

        egui::Window::new("\u{1F4DD} Memory")
            .open(&mut self.memory_window_open)
            .resizable(false)
            .show(ctx, |ui| {
                ctx.memory_ui(ui);
            });

        show_about_window!(ctx, &mut self.about_window_open);
    }
}

fn main() {
    let options = eframe::NativeOptions {
        initial_window_size: Some(vec2(800.0, 600.0)),
        ..Default::default()
    };

    eframe::run_native(
        "Widget Gallery",
        options,
        Box::new(|_| Box::new(WidgetGallery::default())),
    );
}
