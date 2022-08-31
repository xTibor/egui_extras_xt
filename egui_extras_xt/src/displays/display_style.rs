use egui::{lerp, Color32, Rgba, Stroke, Ui};
use strum::{Display, EnumIter};

// ----------------------------------------------------------------------------

#[derive(Clone, Copy, Debug)]
pub struct DisplayStyle {
    pub background_color: Color32,
    pub foreground_on_color: Color32,
    pub foreground_off_color: Color32,
    pub foreground_on_stroke: Stroke,
    pub foreground_off_stroke: Stroke,
}

impl DisplayStyle {
    pub fn foreground_color(&self, active: bool) -> Color32 {
        if active {
            self.foreground_on_color
        } else {
            self.foreground_off_color
        }
    }

    pub fn foreground_stroke(&self, active: bool) -> Stroke {
        if active {
            self.foreground_on_stroke
        } else {
            self.foreground_off_stroke
        }
    }

    pub fn foreground_color_blend(&self, value: f32) -> Color32 {
        Color32::from(lerp(
            Rgba::from(self.foreground_off_color)..=Rgba::from(self.foreground_on_color),
            value,
        ))
    }

    pub fn foreground_stroke_blend(&self, value: f32) -> Stroke {
        Stroke::new(
            lerp(
                self.foreground_off_stroke.width..=self.foreground_on_stroke.width,
                value,
            ),
            Color32::from(lerp(
                Rgba::from(self.foreground_off_stroke.color)
                    ..=Rgba::from(self.foreground_on_stroke.color),
                value,
            )),
        )
    }

    pub fn system_style(ui: &Ui) -> Self {
        DisplayStyle {
            background_color: Color32::TRANSPARENT,
            foreground_on_color: ui.style().visuals.text_color(),
            foreground_off_color: ui.style().visuals.faint_bg_color,
            foreground_on_stroke: Stroke::none(),
            foreground_off_stroke: Stroke::none(),
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
}

impl DisplayStylePreset {
    pub fn style(&self) -> DisplayStyle {
        match *self {
            DisplayStylePreset::Default => DisplayStyle {
                background_color: Color32::from_rgb(0x00, 0x20, 0x00),
                foreground_on_color: Color32::from_rgb(0x00, 0xF0, 0x00),
                foreground_off_color: Color32::from_rgb(0x00, 0x30, 0x00),
                foreground_on_stroke: Stroke::none(),
                foreground_off_stroke: Stroke::none(),
            },
            DisplayStylePreset::Calculator => DisplayStyle {
                background_color: Color32::from_rgb(0xC5, 0xCB, 0xB6),
                foreground_on_color: Color32::from_rgb(0x00, 0x00, 0x00),
                foreground_off_color: Color32::from_rgb(0xB9, 0xBE, 0xAB),
                foreground_on_stroke: Stroke::none(),
                foreground_off_stroke: Stroke::none(),
            },
            DisplayStylePreset::NintendoGameBoy => DisplayStyle {
                background_color: Color32::from_rgb(0x9B, 0xBC, 0x0F),
                foreground_on_color: Color32::from_rgb(0x0F, 0x38, 0x0F),
                foreground_off_color: Color32::from_rgb(0x8B, 0xAC, 0x0F),
                foreground_on_stroke: Stroke::none(),
                foreground_off_stroke: Stroke::none(),
            },
            DisplayStylePreset::KnightRider => DisplayStyle {
                background_color: Color32::from_rgb(0x10, 0x00, 0x00),
                foreground_on_color: Color32::from_rgb(0xC8, 0x00, 0x00),
                foreground_off_color: Color32::from_rgb(0x20, 0x00, 0x00),
                foreground_on_stroke: Stroke::none(),
                foreground_off_stroke: Stroke::none(),
            },
            DisplayStylePreset::BlueNegative => DisplayStyle {
                background_color: Color32::from_rgb(0x00, 0x00, 0xFF),
                foreground_on_color: Color32::from_rgb(0xE0, 0xFF, 0xFF),
                foreground_off_color: Color32::from_rgb(0x28, 0x28, 0xFF),
                foreground_on_stroke: Stroke::none(),
                foreground_off_stroke: Stroke::none(),
            },
            DisplayStylePreset::Amber => DisplayStyle {
                background_color: Color32::from_rgb(0x1D, 0x12, 0x07),
                foreground_on_color: Color32::from_rgb(0xFF, 0x9A, 0x21),
                foreground_off_color: Color32::from_rgb(0x33, 0x20, 0x00),
                foreground_on_stroke: Stroke::none(),
                foreground_off_stroke: Stroke::none(),
            },
            DisplayStylePreset::LightBlue => DisplayStyle {
                background_color: Color32::from_rgb(0x0F, 0xB0, 0xBC),
                foreground_on_color: Color32::from_black_alpha(223),
                foreground_off_color: Color32::from_black_alpha(60),
                foreground_on_stroke: Stroke::none(),
                foreground_off_stroke: Stroke::none(),
            },
            DisplayStylePreset::DeLoreanRed => DisplayStyle {
                background_color: Color32::from_rgb(0x12, 0x07, 0x0A),
                foreground_on_color: Color32::from_rgb(0xFF, 0x59, 0x13),
                foreground_off_color: Color32::from_rgb(0x48, 0x0A, 0x0B),
                foreground_on_stroke: Stroke::none(),
                foreground_off_stroke: Stroke::none(),
            },
            DisplayStylePreset::DeLoreanGreen => DisplayStyle {
                background_color: Color32::from_rgb(0x05, 0x0A, 0x0A),
                foreground_on_color: Color32::from_rgb(0x4A, 0xF5, 0x0F),
                foreground_off_color: Color32::from_rgb(0x07, 0x29, 0x0F),
                foreground_on_stroke: Stroke::none(),
                foreground_off_stroke: Stroke::none(),
            },
            DisplayStylePreset::DeLoreanAmber => DisplayStyle {
                background_color: Color32::from_rgb(0x08, 0x08, 0x0B),
                foreground_on_color: Color32::from_rgb(0xF2, 0xC4, 0x21),
                foreground_off_color: Color32::from_rgb(0x51, 0x2C, 0x0F),
                foreground_on_stroke: Stroke::none(),
                foreground_off_stroke: Stroke::none(),
            },
        }
    }
}
