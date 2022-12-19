use egui::{Response, Ui};

pub trait StandardButtons {
    fn reset_button(&mut self) -> Response;
}

impl StandardButtons for Ui {
    fn reset_button(&mut self) -> Response {
        self.button("\u{1F504} Reset")
    }
}
