use std::f32::consts::TAU;

use eframe::egui::{self, DragValue};

use egui_extras_xt::displays::{BufferLayout, WaveformDisplayWidget};

const BUFFER_SIZE: usize = 1024;
const OUTPUT_FREQUENCY: usize = 44100;

struct WaveformDemoApp {
    enabled: bool,
    buffer: [f32; BUFFER_SIZE],
    left_frequency: f32,
    right_frequency: f32,
    phase: f32,
}

impl Default for WaveformDemoApp {
    fn default() -> Self {
        let mut tmp = Self {
            enabled: true,
            buffer: [0.0; BUFFER_SIZE],
            left_frequency: 440.0,
            right_frequency: 440.0,
            phase: 0.0,
        };
        tmp.regenerate_buffer();
        tmp
    }
}

impl WaveformDemoApp {
    #[allow(clippy::iter_skip_zero)]
    fn regenerate_buffer(&mut self) {
        for (index, sample) in self.buffer.iter_mut().skip(0).step_by(2).enumerate() {
            let q = index as f32 * (self.left_frequency / OUTPUT_FREQUENCY as f32) + self.phase;
            *sample = (q % 1.0) * 2.0 - 1.0;
        }

        for (index, sample) in self.buffer.iter_mut().skip(1).step_by(2).enumerate() {
            let q = index as f32 * (self.right_frequency / OUTPUT_FREQUENCY as f32) + self.phase;
            *sample = (q * TAU).sin();
        }
    }
}

impl eframe::App for WaveformDemoApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.add(DragValue::new(&mut self.left_frequency)).changed() {
                    self.regenerate_buffer();
                }

                if ui.add(DragValue::new(&mut self.right_frequency)).changed() {
                    self.regenerate_buffer();
                }

                if ui.add(DragValue::new(&mut self.phase).speed(0.1)).changed() {
                    self.regenerate_buffer();
                }
            });

            ui.separator();

            ui.add(
                WaveformDisplayWidget::new(&mut self.enabled)
                    .track_name("Track #1")
                    .channels(2)
                    .channel_names(&["Left", "Right"])
                    .buffer(&self.buffer)
                    .buffer_layout(BufferLayout::Interleaved),
            );

            ui.separator();
            egui::ScrollArea::both().show(ui, |ui| {
                ctx.settings_ui(ui);
            });
        });
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([640.0, 480.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Waveform Display",
        options,
        Box::new(|_| Ok(Box::<WaveformDemoApp>::default())),
    )
}
