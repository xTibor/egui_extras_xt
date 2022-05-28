use eframe::egui::{Response, Sense, Ui};
use eframe::emath::{pos2, vec2, Pos2};
use eframe::epaint::{Color32, Shape, Stroke};

pub fn seven_segment(ui: &mut Ui) -> Response {
    let desired_size = vec2(128.0, 256.0);
    let (rect, mut response) = ui.allocate_exact_size(desired_size, Sense::hover());

    ui.painter()
        .rect(rect, 0.0, Color32::RED, Stroke::new(2.0, Color32::BLUE));

    let thickness = 32.0;
    let spacing = 2.0;
    let (width, height) = (rect.width(), rect.height());

    let p = |x, y| rect.center() + vec2(x, y);

    #[rustfmt::skip]
    let segments: [Vec<Pos2>; 7] = [
        vec![
            p(-(width / 2.0 - thickness / 4.0 - spacing), -(height / 2.0 - thickness / 4.0)),
            p(-(width / 2.0 - thickness / 2.0 - spacing), -(height / 2.0)),
            p((width / 2.0 - thickness / 2.0 - spacing), -(height / 2.0)),
            p((width / 2.0 - thickness / 4.0 - spacing), -(height / 2.0 - thickness / 4.0)),
            p((width / 2.0 - thickness - spacing), -(height / 2.0 - thickness)),
            p(-(width / 2.0 - thickness - spacing), -(height / 2.0 - thickness)),
        ],
        vec![
            p((width / 2.0 - thickness), -(height / 2.0 - thickness - spacing)),
            p((width / 2.0 - thickness / 4.0), -(height / 2.0 - thickness / 4.0 - spacing)),
            p((width / 2.0), -(height / 2.0 - thickness / 2.0 - spacing)),
            p((width / 2.0), -(thickness / 2.0 + spacing)),
            p((width / 2.0 - thickness / 2.0), -(spacing)),
            p((width / 2.0 - thickness), -(thickness / 2.0 + spacing)),
        ],
        vec![
            p((width / 2.0 - thickness), (height / 2.0 - thickness - spacing)),
            p((width / 2.0 - thickness / 4.0), (height / 2.0 - thickness / 4.0 - spacing)),
            p((width / 2.0), (height / 2.0 - thickness / 2.0 - spacing)),
            p((width / 2.0), (thickness / 2.0 + spacing)),
            p((width / 2.0 - thickness / 2.0), (spacing)),
            p((width / 2.0 - thickness), (thickness / 2.0 + spacing)),
        ],
        vec![
            p(-(width / 2.0 - thickness / 4.0 - spacing), (height / 2.0 - thickness / 4.0)),
            p(-(width / 2.0 - thickness / 2.0 - spacing), (height / 2.0)),
            p((width / 2.0 - thickness / 2.0 - spacing), (height / 2.0)),
            p((width / 2.0 - thickness / 4.0 - spacing), (height / 2.0 - thickness / 4.0)),
            p((width / 2.0 - thickness - spacing), (height / 2.0 - thickness)),
            p(-(width / 2.0 - thickness - spacing), (height / 2.0 - thickness)),
        ],
        vec![
            p(-(width / 2.0 - thickness), (height / 2.0 - thickness - spacing)),
            p(-(width / 2.0 - thickness / 4.0), (height / 2.0 - thickness / 4.0 - spacing)),
            p(-(width / 2.0), (height / 2.0 - thickness / 2.0 - spacing)),
            p(-(width / 2.0), (thickness / 2.0 + spacing)),
            p(-(width / 2.0 - thickness / 2.0), (spacing)),
            p(-(width / 2.0 - thickness), (thickness / 2.0 + spacing)),
        ],
        vec![
            p(-(width / 2.0 - thickness), -(height / 2.0 - thickness - spacing)),
            p(-(width / 2.0 - thickness / 4.0), -(height / 2.0 - thickness / 4.0 - spacing)),
            p(-(width / 2.0), -(height / 2.0 - thickness / 2.0 - spacing)),
            p(-(width / 2.0), -(thickness / 2.0 + spacing)),
            p(-(width / 2.0 - thickness / 2.0), -(spacing)),
            p(-(width / 2.0 - thickness), -(thickness / 2.0 + spacing)),
        ],
        vec![
            p(-(width / 2.0 - thickness / 2.0 - spacing), 0.0),
            p(-(width / 2.0 - thickness - spacing), -(thickness / 2.0)),
            p((width / 2.0 - thickness - spacing), -(thickness / 2.0)),
            p((width / 2.0 - thickness / 2.0 - spacing), 0.0),
            p((width / 2.0 - thickness - spacing), (thickness / 2.0)),
            p(-(width / 2.0 - thickness - spacing), (thickness / 2.0)),
        ],
    ];

    for segment in segments {
        ui.painter().add(Shape::convex_polygon(
            segment,
            Color32::GREEN,
            Stroke::none(),
        ));
    }

    response
}
