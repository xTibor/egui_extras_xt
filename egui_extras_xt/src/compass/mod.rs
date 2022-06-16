pub mod linear_compass;

use std::f32::consts::TAU;

use egui::{vec2, Align2, Color32, FontFamily, FontId, Rect, Shape, Stroke, Ui, Vec2};
use itertools::Itertools;

// ----------------------------------------------------------------------------

pub type CompassLabels<'a> = [&'a str; 4];

// ----------------------------------------------------------------------------

#[non_exhaustive]
#[derive(Clone, Copy)]
pub enum CompassMarkerShape {
    Square,
    Circle,
    RightArrow,
    UpArrow,
    LeftArrow,
    DownArrow,
    Diamond,
    Star(usize, f32),
    Emoji(char),
}

impl CompassMarkerShape {
    pub fn paint(&self, ui: &mut Ui, rect: Rect, fill: Color32, stroke: Stroke) {
        match *self {
            CompassMarkerShape::Square => {
                ui.painter().rect(rect, 0.0, fill, stroke);
            }
            CompassMarkerShape::Circle => {
                ui.painter().rect(rect, rect.width() / 2.0, fill, stroke);
            }
            CompassMarkerShape::RightArrow => {
                let rect = Rect::from_center_size(
                    rect.center(),
                    rect.size() * vec2(3.0f32.sqrt() / 2.0, 1.0),
                );

                ui.painter().add(Shape::convex_polygon(
                    vec![rect.right_center(), rect.left_bottom(), rect.left_top()],
                    fill,
                    stroke,
                ));
            }
            CompassMarkerShape::UpArrow => {
                let rect = Rect::from_center_size(
                    rect.center(),
                    rect.size() * vec2(1.0, 3.0f32.sqrt() / 2.0),
                );

                ui.painter().add(Shape::convex_polygon(
                    vec![rect.center_top(), rect.right_bottom(), rect.left_bottom()],
                    fill,
                    stroke,
                ));
            }
            CompassMarkerShape::LeftArrow => {
                let rect = Rect::from_center_size(
                    rect.center(),
                    rect.size() * vec2(3.0f32.sqrt() / 2.0, 1.0),
                );

                ui.painter().add(Shape::convex_polygon(
                    vec![rect.left_center(), rect.right_top(), rect.right_bottom()],
                    fill,
                    stroke,
                ));
            }
            CompassMarkerShape::DownArrow => {
                let rect = Rect::from_center_size(
                    rect.center(),
                    rect.size() * vec2(1.0, 3.0f32.sqrt() / 2.0),
                );

                ui.painter().add(Shape::convex_polygon(
                    vec![rect.left_top(), rect.right_top(), rect.center_bottom()],
                    fill,
                    stroke,
                ));
            }
            CompassMarkerShape::Diamond => {
                ui.painter().add(Shape::convex_polygon(
                    vec![
                        rect.center_top(),
                        rect.right_center(),
                        rect.center_bottom(),
                        rect.left_center(),
                    ],
                    fill,
                    stroke,
                ));
            }
            CompassMarkerShape::Star(rays, ratio) => {
                assert!(rays >= 2, "star-shaped markers must have at least 2 rays");
                assert!(
                    (0.0..=1.0).contains(&ratio),
                    "ray ratio of star-shaped markers must be normalized"
                );

                let outer_radius = rect.width() * 0.5;
                let inner_radius = outer_radius * ratio;
                let star_rotation = -TAU * 0.25;

                let outer_points = (0..rays).map(|point_index| {
                    rect.center()
                        + Vec2::angled(
                            star_rotation + TAU * ((point_index as f32 + 0.0) / rays as f32),
                        ) * outer_radius
                });

                let inner_points = (0..rays).map(|point_index| {
                    rect.center()
                        + Vec2::angled(
                            star_rotation + TAU * ((point_index as f32 + 0.5) / rays as f32),
                        ) * inner_radius
                });

                // TODO: Broken polygon renderer
                // https://github.com/emilk/egui/issues/513
                ui.painter().add(Shape::convex_polygon(
                    outer_points.interleave(inner_points).collect_vec(),
                    fill,
                    stroke,
                ));
            }
            CompassMarkerShape::Emoji(emoji) => {
                ui.painter().text(
                    rect.center(),
                    Align2::CENTER_CENTER,
                    emoji,
                    FontId::new(rect.height(), FontFamily::Proportional),
                    fill,
                );
            }
        }
    }
}
