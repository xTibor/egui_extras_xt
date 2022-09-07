use egui::{vec2, Align2, FontFamily, FontId, Rect, Response, Sense, Stroke, Ui, Widget};

use crate::displays::{DisplayStyle, DisplayStylePreset};

// ----------------------------------------------------------------------------

/// Combined into one function (rather than two) to make it easier
/// for the borrow checker.
type GetSetValue<'a> = Box<dyn 'a + FnMut(Option<bool>) -> bool>;

fn get(get_set_value: &mut GetSetValue<'_>) -> bool {
    (get_set_value)(None)
}

fn set(get_set_value: &mut GetSetValue<'_>, value: bool) {
    (get_set_value)(Some(value));
}

// ----------------------------------------------------------------------------

#[must_use = "You should put this widget in an ui with `ui.add(widget);`"]
pub struct IndicatorButton<'a> {
    get_set_value: GetSetValue<'a>,
    width: f32,
    height: f32,
    label: Option<String>,
    style: DisplayStyle,
    animated: bool,
    interactive: bool,
}

impl<'a> IndicatorButton<'a> {
    pub fn new(value: &'a mut bool) -> Self {
        Self::from_get_set(move |v: Option<bool>| {
            if let Some(v) = v {
                *value = v;
            }
            *value
        })
    }

    pub fn from_get_set(get_set_value: impl 'a + FnMut(Option<bool>) -> bool) -> Self {
        Self {
            get_set_value: Box::new(get_set_value),
            width: 64.0,
            height: 40.0,
            label: None,
            style: DisplayStylePreset::Default.style(),
            animated: true,
            interactive: true,
        }
    }

    pub fn width(mut self, width: impl Into<f32>) -> Self {
        self.width = width.into();
        self
    }

    pub fn height(mut self, height: impl Into<f32>) -> Self {
        self.height = height.into();
        self
    }

    pub fn label(mut self, label: impl ToString) -> Self {
        self.label = Some(label.to_string());
        self
    }

    pub fn style(mut self, style: DisplayStyle) -> Self {
        self.style = style;
        self
    }

    pub fn style_preset(mut self, preset: DisplayStylePreset) -> Self {
        self.style = preset.style();
        self
    }

    pub fn animated(mut self, animated: bool) -> Self {
        self.animated = animated;
        self
    }

    pub fn interactive(mut self, interactive: bool) -> Self {
        self.interactive = interactive;
        self
    }
}

impl<'a> Widget for IndicatorButton<'a> {
    fn ui(mut self, ui: &mut Ui) -> Response {
        let desired_size = vec2(self.width, self.height);

        let (rect, mut response) = ui.allocate_exact_size(
            desired_size,
            if self.interactive {
                Sense::click()
            } else {
                Sense::hover()
            },
        );

        if response.clicked() {
            let value = get(&mut self.get_set_value);
            set(&mut self.get_set_value, !value);

            response.mark_changed();
        }

        if ui.is_rect_visible(rect) {
            let visuals = *ui.style().interact(&response);

            let value = if self.animated {
                ui.ctx()
                    .animate_bool(response.id, get(&mut self.get_set_value))
            } else {
                if get(&mut self.get_set_value) {
                    1.0
                } else {
                    0.0
                }
            };

            ui.painter()
                .rect(rect, visuals.rounding, visuals.bg_fill, visuals.bg_stroke);

            let top_rect = Rect::from_min_max(rect.left_top(), rect.right_center());
            let bottom_rect = Rect::from_min_max(rect.left_center(), rect.right_bottom());

            {
                let indicator_rect = if self.label.is_some() { top_rect } else { rect };

                ui.painter().rect(
                    indicator_rect.shrink(4.0),
                    4.0,
                    self.style.background_color,
                    Stroke::none(),
                );

                ui.painter().rect(
                    indicator_rect.shrink(6.0),
                    4.0,
                    self.style.foreground_color_blend(value),
                    Stroke::none(),
                );
            }

            if let Some(label) = self.label {
                ui.painter().text(
                    bottom_rect.center() - vec2(0.0, 2.0),
                    Align2::CENTER_CENTER,
                    label,
                    FontId::new(bottom_rect.height() - 4.0, FontFamily::Proportional),
                    visuals.text_color(),
                );
            }
        }

        response
    }
}
