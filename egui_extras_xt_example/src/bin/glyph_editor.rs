use eframe::egui::{self};
use eframe::emath::vec2;
use eframe::epaint::Vec2;

use strum::IntoEnumIterator;

use egui_extras_xt::displays::segmented_display::{DisplayDigit, DisplayGlyph};
use egui_extras_xt::displays::{DisplayKind, SegmentedDisplayWidget};
use egui_extras_xt::ui::standard_buttons::StandardButtons;
use egui_extras_xt::ui::widgets_from_iter::SelectableValueFromIter;

struct GlyphEditorApp {
    display_kind: DisplayKind,
    digit: DisplayDigit,
}

impl Default for GlyphEditorApp {
    fn default() -> Self {
        Self {
            display_kind: DisplayKind::SixteenSegment,
            digit: DisplayDigit::default(),
        }
    }
}

impl eframe::App for GlyphEditorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.add(
                    SegmentedDisplayWidget::new(self.display_kind)
                        .digit_height(192.0)
                        .push_digit(self.digit),
                );

                ui.separator();

                ui.vertical(|ui| {
                    ui.horizontal(|ui| {
                        ui.selectable_value_from_iter(&mut self.display_kind, DisplayKind::iter());
                    });

                    ui.horizontal(|ui| {
                        if ui.reset_button().clicked() {
                            self.digit = DisplayDigit::default();
                        }

                        {
                            let hex_value = format!("0x{:04X}", self.digit.glyph);
                            if ui
                                .button(&hex_value)
                                .on_hover_text("\u{1F5D0} Copy to clipboard")
                                .clicked()
                            {
                                ui.output_mut(|output| output.copied_text = hex_value);
                            }
                        }
                    });
                })
            });

            ui.separator();

            ui.horizontal_wrapped(|ui| {
                ui.spacing_mut().item_spacing = Vec2::ZERO;

                for segment_index in (0..self.display_kind.segment_count()).rev() {
                    let mut segment_state = ((self.digit.glyph >> segment_index) & 0x01) != 0x00;

                    if ui
                        .add(
                            SegmentedDisplayWidget::new(self.display_kind)
                                .digit_height(64.0)
                                .push_digit(DisplayDigit {
                                    glyph: 1 << segment_index,
                                    ..Default::default()
                                }),
                        )
                        .clicked()
                    {
                        segment_state = !segment_state;
                    }

                    self.digit.glyph = (self.digit.glyph & !(1 << segment_index))
                        | ((segment_state as DisplayGlyph) << segment_index);
                }
            });
        });
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        initial_window_size: Some(vec2(570.0, 410.0)),
        ..Default::default()
    };

    eframe::run_native(
        "Glyph Editor",
        options,
        Box::new(|_| Box::<GlyphEditorApp>::default()),
    )
}
