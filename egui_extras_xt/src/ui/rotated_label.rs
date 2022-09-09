use egui::{Color32, FontSelection, Pos2, Rect, Response, Sense, Ui, Widget};
use emath::Rot2;
use epaint::TextShape;

#[must_use = "You should put this widget in an ui with `ui.add(widget);`"]
pub struct RotatedLabel {
    value: String,
    angle: f32,
    color: Option<Color32>,
}

impl RotatedLabel {
    pub fn new(value: impl ToString) -> Self {
        Self {
            value: value.to_string(),
            angle: 0.0,
            color: None,
        }
    }

    pub fn angle(mut self, angle: impl Into<f32>) -> Self {
        self.angle = angle.into();
        self
    }

    pub fn color(mut self, color: Option<Color32>) -> Self {
        self.color = color;
        self
    }
}

impl Widget for RotatedLabel {
    fn ui(self, ui: &mut Ui) -> Response {
        let color = self
            .color
            .unwrap_or_else(|| ui.style().visuals.text_color());

        let font_id = FontSelection::Default.resolve(ui.style());
        let galley = ui.painter().layout_no_wrap(self.value, font_id, color);

        let rotation = Rot2::from_angle(self.angle);

        let bounding_rect = Rect::from_center_size(Pos2::ZERO, galley.size()).rotate_bb(rotation);
        let (rect, response) = ui.allocate_exact_size(bounding_rect.size(), Sense::hover());

        if ui.is_rect_visible(rect) {
            let pos = rect.center() - (rotation * (galley.size() / 2.0));

            ui.painter().add(TextShape {
                angle: self.angle,
                ..TextShape::new(pos, galley)
            });
        }

        response
    }
}
