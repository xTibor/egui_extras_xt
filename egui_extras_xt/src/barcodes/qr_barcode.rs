use egui::{vec2, Color32, Response, Sense, Stroke, Ui, Widget};

// ----------------------------------------------------------------------------

#[must_use = "You should put this widget in an ui with `ui.add(widget);`"]
pub struct QrBarcodeWidget<'a> {
    value: &'a str,
    cell_size: usize,
    padding: f32,
}

impl<'a> QrBarcodeWidget<'a> {
    pub fn new(value: &'a str) -> Self {
        Self {
            value,
            cell_size: 2,
            padding: 20.0,
        }
    }

    pub fn cell_size(mut self, cell_size: impl Into<usize>) -> Self {
        self.cell_size = cell_size.into();
        self
    }

    pub fn padding(mut self, padding: impl Into<f32>) -> Self {
        self.padding = padding.into();
        self
    }
}

impl<'a> Widget for QrBarcodeWidget<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        let cell_size = self.cell_size as f32 / ui.ctx().pixels_per_point();

        let desired_size = vec2(256.0, 256.0);

        let (rect, response) = ui.allocate_exact_size(desired_size, Sense::hover());

        if ui.is_rect_visible(rect) {
            ui.painter().rect(
                rect,
                ui.style().visuals.noninteractive().rounding,
                Color32::WHITE,
                Stroke::none(),
            );
        }

        response
    }
}
