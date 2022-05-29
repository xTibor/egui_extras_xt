use eframe::egui::{Response, Sense, Ui};
use eframe::emath::{pos2, vec2, Pos2};
use eframe::epaint::{Color32, Shape, Stroke};

pub enum SevenSegmentPreset {
    DeLoreanRed,
    DeLoreanGreen,
    DeLoreanAmber,
}

pub fn seven_segment(ui: &mut Ui, thickness: f32, spacing: f32, slant: f32) -> Response {
    let desired_size = vec2(128.0, 256.0);
    let (rect, mut response) = ui.allocate_exact_size(desired_size, Sense::hover());

    ui.painter()
        .rect(rect, 0.0, Color32::RED, Stroke::new(2.0, Color32::BLUE));

    let (width, height) = (rect.width(), rect.height());

    #[rustfmt::skip]
    let segments: [Vec<Pos2>; 7] = {
        let p =
            |x, y| rect.center() + (vec2(x, y) - vec2((y / (rect.height() / 2.0)) * slant, 0.0));

        #[allow(unused_parens)]
        [
            vec![
                p(-(width / 2.0) + (thickness / 4.0) + spacing, -(height / 2.0) + (thickness / 4.0)          ),
                p(-(width / 2.0) + (thickness / 2.0) + spacing, -(height / 2.0)                              ),
                p( (width / 2.0) - (thickness / 2.0) - spacing, -(height / 2.0)                              ),
                p( (width / 2.0) - (thickness / 4.0) - spacing, -(height / 2.0) + (thickness / 4.0)          ),
                p( (width / 2.0) - (thickness / 1.0) - spacing, -(height / 2.0) + (thickness / 1.0)          ),
                p(-(width / 2.0) + (thickness / 1.0) + spacing, -(height / 2.0) + (thickness / 1.0)          ),
            ],
            vec![
                p( (width / 2.0) - (thickness / 1.0)          , -(height / 2.0) + (thickness / 1.0) + spacing),
                p( (width / 2.0) - (thickness / 4.0)          , -(height / 2.0) + (thickness / 4.0) + spacing),
                p( (width / 2.0)                              , -(height / 2.0) + (thickness / 2.0) + spacing),
                p( (width / 2.0)                              ,                 - (thickness / 2.0) - spacing),
                p( (width / 2.0) - (thickness / 2.0)          ,                                     - spacing),
                p( (width / 2.0) - (thickness / 1.0)          ,                 - (thickness / 2.0) - spacing),
            ],
            vec![
                p( (width / 2.0) - (thickness / 1.0)          ,  (height / 2.0) - (thickness / 1.0) - spacing),
                p( (width / 2.0) - (thickness / 4.0)          ,  (height / 2.0) - (thickness / 4.0) - spacing),
                p( (width / 2.0)                              ,  (height / 2.0) - (thickness / 2.0) - spacing),
                p( (width / 2.0)                              ,                   (thickness / 2.0) + spacing),
                p( (width / 2.0) - (thickness / 2.0)          ,                                       spacing),
                p( (width / 2.0) - (thickness / 1.0)          ,                   (thickness / 2.0) + spacing),
            ],
            vec![
                p(-(width / 2.0) + (thickness / 4.0) + spacing,  (height / 2.0) - (thickness / 4.0)          ),
                p(-(width / 2.0) + (thickness / 2.0) + spacing,  (height / 2.0)                              ),
                p( (width / 2.0) - (thickness / 2.0) - spacing,  (height / 2.0)                              ),
                p( (width / 2.0) - (thickness / 4.0) - spacing,  (height / 2.0) - (thickness / 4.0)          ),
                p( (width / 2.0) - (thickness / 1.0) - spacing,  (height / 2.0) - (thickness / 1.0)          ),
                p(-(width / 2.0) + (thickness / 1.0) + spacing,  (height / 2.0) - (thickness / 1.0)          ),
            ],
            vec![
                p(-(width / 2.0) + (thickness / 1.0)          ,  (height / 2.0) - (thickness / 1.0) - spacing),
                p(-(width / 2.0) + (thickness / 4.0)          ,  (height / 2.0) - (thickness / 4.0) - spacing),
                p(-(width / 2.0)                              ,  (height / 2.0) - (thickness / 2.0) - spacing),
                p(-(width / 2.0)                              ,                   (thickness / 2.0) + spacing),
                p(-(width / 2.0) + (thickness / 2.0)          ,                                       spacing),
                p(-(width / 2.0) + (thickness / 1.0)          ,                   (thickness / 2.0) + spacing),
            ],
            vec![
                p(-(width / 2.0) + (thickness / 1.0)          , -(height / 2.0) + (thickness / 1.0) + spacing),
                p(-(width / 2.0) + (thickness / 4.0)          , -(height / 2.0) + (thickness / 4.0) + spacing),
                p(-(width / 2.0)                              , -(height / 2.0) + (thickness / 2.0) + spacing),
                p(-(width / 2.0)                              ,                 - (thickness / 2.0) - spacing),
                p(-(width / 2.0) + (thickness / 2.0)          ,                                     - spacing),
                p(-(width / 2.0) + (thickness / 1.0)          ,                 - (thickness / 2.0) - spacing),
            ],
            vec![
                p(-(width / 2.0) + (thickness / 2.0) + spacing,                                           0.0),
                p(-(width / 2.0) + (thickness / 1.0) + spacing,                 - (thickness / 2.0)          ),
                p( (width / 2.0) - (thickness / 1.0) - spacing,                 - (thickness / 2.0)          ),
                p( (width / 2.0) - (thickness / 2.0) - spacing,                                           0.0),
                p( (width / 2.0) - (thickness / 1.0) - spacing,                   (thickness / 2.0)          ),
                p(-(width / 2.0) + (thickness / 1.0) + spacing,                   (thickness / 2.0)          ),
            ],
        ]
    };

    for segment in segments {
        ui.painter().add(Shape::convex_polygon(
            segment,
            Color32::GREEN,
            Stroke::new(2.0, Color32::DARK_GREEN),
        ));
    }

    response
}
