use egui::{FontSelection, Pos2, Rect, Response, Sense, Ui, Widget};
use emath::Rot2;
use epaint::TextShape;

#[must_use = "You should put this widget in an ui with `ui.add(widget);`"]
pub struct RotatedLabel {
    text: String,
    angle: f32,
}

impl RotatedLabel {
    pub fn new(text: impl ToString) -> Self {
        Self {
            text: text.to_string(),
            angle: 0.0,
        }
    }

    pub fn angle(mut self, angle: impl Into<f32>) -> Self {
        self.angle = angle.into();
        self
    }
}

impl Widget for RotatedLabel {
    fn ui(self, ui: &mut Ui) -> Response {
        let text_color = ui.style().visuals.text_color();

        let galley = {
            let font_id = FontSelection::Default.resolve(ui.style());
            ui.painter().layout_no_wrap(self.text, font_id, text_color)
        };

        let rotation = Rot2::from_angle(self.angle);

        let (rect, response) = {
            let bounding_rect =
                Rect::from_center_size(Pos2::ZERO, galley.size()).rotate_bb(rotation);
            ui.allocate_exact_size(bounding_rect.size(), Sense::hover())
        };

        if ui.is_rect_visible(rect) {
            let pos = rect.center() - (rotation * (galley.size() / 2.0));

            ui.painter().add(TextShape {
                angle: self.angle,
                ..TextShape::new(pos, galley, text_color)
            });
        }

        response
    }
}
