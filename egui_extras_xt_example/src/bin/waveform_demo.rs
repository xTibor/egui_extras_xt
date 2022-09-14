use eframe::egui::{self, DragValue};
use eframe::emath::vec2;

use egui_extras_xt::displays::{BufferLayout, WaveformDisplayWidget};

const BUFFER_SIZE: usize = 1024;
const OUTPUT_FREQUENCY: usize = 44100;

struct WaveformDemoApp {
    enabled: bool,
    buffer: [u8; BUFFER_SIZE],
    left_frequency: f32,
    right_frequency: f32,
}

impl Default for WaveformDemoApp {
    fn default() -> Self {
        let mut tmp = Self {
            enabled: true,
            buffer: [0; BUFFER_SIZE],
            left_frequency: 440.0,
            right_frequency: 440.0,
        };
        tmp.regenerate_buffer();
        tmp
    }
}

impl WaveformDemoApp {
    fn regenerate_buffer(&mut self) {
        for (index, sample) in self.buffer.iter_mut().skip(0).step_by(2).enumerate() {
            let q = index as f32 * (self.left_frequency / OUTPUT_FREQUENCY as f32);
            *sample = if (q % 1.0) < 0.5 { 255 } else { 0 };
        }

        for (index, sample) in self.buffer.iter_mut().skip(1).step_by(2).enumerate() {
            let q = index as f32 * (self.right_frequency / OUTPUT_FREQUENCY as f32);
            *sample = if (q % 1.0) < 0.5 { 255 } else { 0 };
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
            });

            ui.add(
                WaveformDisplayWidget::new(&mut self.enabled)
                    .channels(2)
                    .buffer(&self.buffer)
                    .buffer_layout(BufferLayout::Interleaved)
                    .label("Channel #1"),
            );
        });
    }
}

fn main() {
    let options = eframe::NativeOptions {
        initial_window_size: Some(vec2(640.0, 480.0)),
        ..Default::default()
    };

    eframe::run_native(
        "Waveform Display",
        options,
        Box::new(|_| Box::new(WaveformDemoApp::default())),
    );
}
