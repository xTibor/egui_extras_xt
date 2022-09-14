use eframe::egui::{self, DragValue};
use eframe::emath::vec2;

use egui_extras_xt::displays::{BufferLayout, WaveformDisplayWidget};

const BUFFER_SIZE: usize = 1024;
const OUTPUT_FREQUENCY: usize = 44100;

struct WaveformDemoApp {
    enabled: bool,
    frequency: f32,
    buffer: [u8; BUFFER_SIZE],
}

impl Default for WaveformDemoApp {
    fn default() -> Self {
        let mut tmp = Self {
            enabled: true,
            frequency: 440.0,
            buffer: [0; BUFFER_SIZE],
        };
        tmp.regenerate_buffer();
        tmp
    }
}

impl WaveformDemoApp {
    fn regenerate_buffer(&mut self) {
        for (index, sample) in self.buffer.iter_mut().enumerate() {
            let q = index as f32 * (self.frequency / OUTPUT_FREQUENCY as f32);
            *sample = if (q % 1.0) > 0.5 { 0 } else { 255 };
        }
    }
}

impl eframe::App for WaveformDemoApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if ui.add(DragValue::new(&mut self.frequency)).changed() {
                self.regenerate_buffer();
            }

            ui.add(
                WaveformDisplayWidget::new(&mut self.enabled)
                    .channels(1)
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
