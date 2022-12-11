use egui::{lerp, Color32, Rgba, Stroke, Ui};
use strum::{Display, EnumIter};

// ----------------------------------------------------------------------------

#[derive(Clone, Copy, Debug)]
pub struct DisplayStyle {
    pub background_color: Color32,
    pub active_foreground_color: Color32,
    pub active_foreground_stroke: Stroke,
    pub inactive_foreground_color: Color32,
    pub inactive_foreground_stroke: Stroke,
}

impl DisplayStyle {
    #[must_use]
    pub fn foreground_color(&self, active: bool) -> Color32 {
        if active {
            self.active_foreground_color
        } else {
            self.inactive_foreground_color
        }
    }

    #[must_use]
    pub fn foreground_stroke(&self, active: bool) -> Stroke {
        if active {
            self.active_foreground_stroke
        } else {
            self.inactive_foreground_stroke
        }
    }

    #[must_use]
    pub fn foreground_color_blend(&self, value: f32) -> Color32 {
        Color32::from(lerp(
            Rgba::from(self.inactive_foreground_color)..=Rgba::from(self.active_foreground_color),
            value,
        ))
    }

    #[must_use]
    pub fn foreground_stroke_blend(&self, value: f32) -> Stroke {
        Stroke::new(
            lerp(
                self.inactive_foreground_stroke.width..=self.active_foreground_stroke.width,
                value,
            ),
            Color32::from(lerp(
                Rgba::from(self.inactive_foreground_stroke.color)
                    ..=Rgba::from(self.active_foreground_stroke.color),
                value,
            )),
        )
    }

    #[must_use]
    pub fn system_style(ui: &Ui) -> Self {
        DisplayStyle {
            background_color: Color32::TRANSPARENT,
            active_foreground_color: ui.style().visuals.text_color(),
            active_foreground_stroke: Stroke::NONE,
            inactive_foreground_color: ui.style().visuals.faint_bg_color,
            inactive_foreground_stroke: Stroke::NONE,
        }
    }
}

impl Default for DisplayStyle {
    fn default() -> Self {
        DisplayStylePreset::Default.style()
    }
}

// ----------------------------------------------------------------------------

#[non_exhaustive]
#[derive(Clone, Copy, Debug, Display, EnumIter, Eq, PartialEq)]
pub enum DisplayStylePreset {
    #[strum(to_string = "Default")]
    Default,

    #[strum(to_string = "Calculator")]
    Calculator,

    #[strum(to_string = "Nintendo Game Boy")]
    NintendoGameBoy,

    #[strum(to_string = "Knight Rider")]
    KnightRider,

    #[strum(to_string = "Blue Negative")]
    BlueNegative,

    #[strum(to_string = "Amber")]
    Amber,

    #[strum(to_string = "Light Blue")]
    LightBlue,

    #[strum(to_string = "DeLorean Red")]
    DeLoreanRed,

    #[strum(to_string = "DeLorean Green")]
    DeLoreanGreen,

    #[strum(to_string = "DeLorean Amber")]
    DeLoreanAmber,

    #[strum(to_string = "Yamaha MU2000")]
    YamahaMU2000,
}

impl DisplayStylePreset {
    #[must_use]
    pub fn style(&self) -> DisplayStyle {
        match *self {
            DisplayStylePreset::Default => DisplayStyle {
                background_color: Color32::from_rgb(0x00, 0x20, 0x00),
                active_foreground_color: Color32::from_rgb(0x00, 0xF0, 0x00),
                active_foreground_stroke: Stroke::NONE,
                inactive_foreground_color: Color32::from_rgb(0x00, 0x30, 0x00),
                inactive_foreground_stroke: Stroke::NONE,
            },
            DisplayStylePreset::Calculator => DisplayStyle {
                background_color: Color32::from_rgb(0xC5, 0xCB, 0xB6),
                active_foreground_color: Color32::from_rgb(0x00, 0x00, 0x00),
                active_foreground_stroke: Stroke::NONE,
                inactive_foreground_color: Color32::from_rgb(0xB9, 0xBE, 0xAB),
                inactive_foreground_stroke: Stroke::NONE,
            },
            DisplayStylePreset::NintendoGameBoy => DisplayStyle {
                background_color: Color32::from_rgb(0x9B, 0xBC, 0x0F),
                active_foreground_color: Color32::from_rgb(0x0F, 0x38, 0x0F),
                active_foreground_stroke: Stroke::NONE,
                inactive_foreground_color: Color32::from_rgb(0x8B, 0xAC, 0x0F),
                inactive_foreground_stroke: Stroke::NONE,
            },
            DisplayStylePreset::KnightRider => DisplayStyle {
                background_color: Color32::from_rgb(0x10, 0x00, 0x00),
                active_foreground_color: Color32::from_rgb(0xC8, 0x00, 0x00),
                active_foreground_stroke: Stroke::NONE,
                inactive_foreground_color: Color32::from_rgb(0x20, 0x00, 0x00),
                inactive_foreground_stroke: Stroke::NONE,
            },
            DisplayStylePreset::BlueNegative => DisplayStyle {
                background_color: Color32::from_rgb(0x00, 0x00, 0xFF),
                active_foreground_color: Color32::from_rgb(0xE0, 0xFF, 0xFF),
                active_foreground_stroke: Stroke::NONE,
                inactive_foreground_color: Color32::from_rgb(0x28, 0x28, 0xFF),
                inactive_foreground_stroke: Stroke::NONE,
            },
            DisplayStylePreset::Amber => DisplayStyle {
                background_color: Color32::from_rgb(0x1D, 0x12, 0x07),
                active_foreground_color: Color32::from_rgb(0xFF, 0x9A, 0x21),
                active_foreground_stroke: Stroke::NONE,
                inactive_foreground_color: Color32::from_rgb(0x33, 0x20, 0x00),
                inactive_foreground_stroke: Stroke::NONE,
            },
            DisplayStylePreset::LightBlue => DisplayStyle {
                background_color: Color32::from_rgb(0x0F, 0xB0, 0xBC),
                active_foreground_color: Color32::from_black_alpha(223),
                active_foreground_stroke: Stroke::NONE,
                inactive_foreground_color: Color32::from_black_alpha(60),
                inactive_foreground_stroke: Stroke::NONE,
            },
            DisplayStylePreset::DeLoreanRed => DisplayStyle {
                background_color: Color32::from_rgb(0x12, 0x07, 0x0A),
                active_foreground_color: Color32::from_rgb(0xFF, 0x59, 0x13),
                active_foreground_stroke: Stroke::NONE,
                inactive_foreground_color: Color32::from_rgb(0x48, 0x0A, 0x0B),
                inactive_foreground_stroke: Stroke::NONE,
            },
            DisplayStylePreset::DeLoreanGreen => DisplayStyle {
                background_color: Color32::from_rgb(0x05, 0x0A, 0x0A),
                active_foreground_color: Color32::from_rgb(0x4A, 0xF5, 0x0F),
                active_foreground_stroke: Stroke::NONE,
                inactive_foreground_color: Color32::from_rgb(0x07, 0x29, 0x0F),
                inactive_foreground_stroke: Stroke::NONE,
            },
            DisplayStylePreset::DeLoreanAmber => DisplayStyle {
                background_color: Color32::from_rgb(0x08, 0x08, 0x0B),
                active_foreground_color: Color32::from_rgb(0xF2, 0xC4, 0x21),
                active_foreground_stroke: Stroke::NONE,
                inactive_foreground_color: Color32::from_rgb(0x51, 0x2C, 0x0F),
                inactive_foreground_stroke: Stroke::NONE,
            },
            DisplayStylePreset::YamahaMU2000 => DisplayStyle {
                background_color: Color32::from_rgb(0x8C, 0xD7, 0x01),
                active_foreground_color: Color32::from_rgb(0x04, 0x4A, 0x00),
                active_foreground_stroke: Stroke::NONE,
                inactive_foreground_color: Color32::from_rgb(0x7B, 0xCE, 0x02),
                inactive_foreground_stroke: Stroke::NONE,
            },
        }
    }
}
