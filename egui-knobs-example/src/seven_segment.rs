use eframe::egui::{Response, Sense, Ui};
use eframe::emath::{pos2, vec2, Pos2};
use eframe::epaint::{Color32, Shape, Stroke};

pub fn seven_segment(ui: &mut Ui, thickness: f32, spacing: f32, slant: f32) -> Response {
    let desired_size = vec2(128.0, 256.0);
    let (rect, mut response) = ui.allocate_exact_size(desired_size, Sense::hover());

    ui.painter()
        .rect(rect, 0.0, Color32::RED, Stroke::new(2.0, Color32::BLUE));

    let (width, height) = (rect.width(), rect.height());

    //#[rustfmt::skip]
    let segments: [Vec<Pos2>; 7] = {
        let p =
            |x, y| rect.center() + (vec2(x, y) - vec2((y / (rect.height() / 2.0)) * slant, 0.0));

        let (w2, h2) = (width / 2.0, height / 2.0);
        let (t1, t2, t4) = (thickness / 1.0, thickness / 2.0, thickness / 4.0);
        let sp = spacing;

        [
            vec![
                p(-w2 + t4 + sp, -h2 + t4),
                p(-w2 + t2 + sp, -h2),
                p(w2 - t2 - sp, -h2),
                p(w2 - t4 - sp, -h2 + t4),
                p(w2 - t1 - sp, -h2 + t1),
                p(-w2 + t1 + sp, -h2 + t1),
            ],
            vec![
                p(w2 - t1, -h2 + t1 + sp),
                p(w2 - t4, -h2 + t4 + sp),
                p(w2, -h2 + t2 + sp),
                p(w2, -t2 - sp),
                p(w2 - t2, -sp),
                p(w2 - t1, -t2 - sp),
            ],
            vec![
                p(w2 - t1, h2 - t1 - sp),
                p(w2 - t4, h2 - t4 - sp),
                p(w2, h2 - t2 - sp),
                p(w2, t2 + sp),
                p(w2 - t2, sp),
                p(w2 - t1, t2 + sp),
            ],
            vec![
                p(-w2 + t4 + sp, h2 - t4),
                p(-w2 + t2 + sp, h2),
                p(w2 - t2 - sp, h2),
                p(w2 - t4 - sp, h2 - t4),
                p(w2 - t1 - sp, h2 - t1),
                p(-w2 + t1 + sp, h2 - t1),
            ],
            vec![
                p(-w2 + t1, h2 - t1 - sp),
                p(-w2 + t4, h2 - t4 - sp),
                p(-w2, h2 - t2 - sp),
                p(-w2, t2 + sp),
                p(-w2 + t2, sp),
                p(-w2 + t1, t2 + sp),
            ],
            vec![
                p(-w2 + t1, -h2 + t1 + sp),
                p(-w2 + t4, -h2 + t4 + sp),
                p(-w2, -h2 + t2 + sp),
                p(-w2, -t2 - sp),
                p(-w2 + t2, -sp),
                p(-w2 + t1, -t2 - sp),
            ],
            vec![
                p(-w2 + t2 + sp, 0.0),
                p(-w2 + t1 + sp, -t2),
                p(w2 - t1 - sp, -t2),
                p(w2 - t2 - sp, 0.0),
                p(w2 - t1 - sp, t2),
                p(-w2 + t1 + sp, t2),
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
