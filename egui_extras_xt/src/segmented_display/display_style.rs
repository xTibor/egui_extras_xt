use egui::{Color32, Stroke, Ui};

// ----------------------------------------------------------------------------

#[derive(Clone, Copy)]
pub struct DisplayStyle {
    pub background_color: Color32,
    pub segment_on_color: Color32,
    pub segment_off_color: Color32,
    pub segment_on_stroke: Stroke,
    pub segment_off_stroke: Stroke,
}

impl DisplayStyle {
    pub fn segment_color(&self, active: bool) -> Color32 {
        if active {
            self.segment_on_color
        } else {
            self.segment_off_color
        }
    }

    pub fn segment_stroke(&self, active: bool) -> Stroke {
        if active {
            self.segment_on_stroke
        } else {
            self.segment_off_stroke
        }
    }

    pub fn system_style(ui: &Ui) -> Self {
        DisplayStyle {
            background_color: Color32::TRANSPARENT,
            segment_on_color: ui.style().visuals.text_color(),
            segment_off_color: ui.style().visuals.faint_bg_color,
            segment_on_stroke: Stroke::none(),
            segment_off_stroke: Stroke::none(),
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
#[derive(Clone, Copy)]
pub enum DisplayStylePreset {
    Default,
    Calculator,
    NintendoGameBoy,
    KnightRider,
    BlueNegative,
    Amber,
    LightBlue,
    DeLoreanRed,
    DeLoreanGreen,
    DeLoreanAmber,
}

impl DisplayStylePreset {
    pub fn style(&self) -> DisplayStyle {
        match *self {
            DisplayStylePreset::Default => DisplayStyle {
                background_color: Color32::from_rgb(0x00, 0x20, 0x00),
                segment_on_color: Color32::from_rgb(0x00, 0xF0, 0x00),
                segment_off_color: Color32::from_rgb(0x00, 0x30, 0x00),
                segment_on_stroke: Stroke::none(),
                segment_off_stroke: Stroke::none(),
            },
            DisplayStylePreset::Calculator => DisplayStyle {
                background_color: Color32::from_rgb(0xC5, 0xCB, 0xB6),
                segment_on_color: Color32::from_rgb(0x00, 0x00, 0x00),
                segment_off_color: Color32::from_rgb(0xB9, 0xBE, 0xAB),
                segment_on_stroke: Stroke::none(),
                segment_off_stroke: Stroke::none(),
            },
            DisplayStylePreset::NintendoGameBoy => DisplayStyle {
                background_color: Color32::from_rgb(0x9B, 0xBC, 0x0F),
                segment_on_color: Color32::from_rgb(0x0F, 0x38, 0x0F),
                segment_off_color: Color32::from_rgb(0x8B, 0xAC, 0x0F),
                segment_on_stroke: Stroke::none(),
                segment_off_stroke: Stroke::none(),
            },
            DisplayStylePreset::KnightRider => DisplayStyle {
                background_color: Color32::from_rgb(0x10, 0x00, 0x00),
                segment_on_color: Color32::from_rgb(0xC8, 0x00, 0x00),
                segment_off_color: Color32::from_rgb(0x20, 0x00, 0x00),
                segment_on_stroke: Stroke::none(),
                segment_off_stroke: Stroke::none(),
            },
            DisplayStylePreset::BlueNegative => DisplayStyle {
                background_color: Color32::from_rgb(0x00, 0x00, 0xFF),
                segment_on_color: Color32::from_rgb(0xE0, 0xFF, 0xFF),
                segment_off_color: Color32::from_rgb(0x28, 0x28, 0xFF),
                segment_on_stroke: Stroke::none(),
                segment_off_stroke: Stroke::none(),
            },
            DisplayStylePreset::Amber => DisplayStyle {
                background_color: Color32::from_rgb(0x1D, 0x12, 0x07),
                segment_on_color: Color32::from_rgb(0xFF, 0x9A, 0x21),
                segment_off_color: Color32::from_rgb(0x33, 0x20, 0x00),
                segment_on_stroke: Stroke::none(),
                segment_off_stroke: Stroke::none(),
            },
            DisplayStylePreset::LightBlue => DisplayStyle {
                background_color: Color32::from_rgb(0x0F, 0xB0, 0xBC),
                segment_on_color: Color32::from_black_alpha(223),
                segment_off_color: Color32::from_black_alpha(60),
                segment_on_stroke: Stroke::none(),
                segment_off_stroke: Stroke::none(),
            },
            DisplayStylePreset::DeLoreanRed => todo!(),
            DisplayStylePreset::DeLoreanGreen => todo!(),
            DisplayStylePreset::DeLoreanAmber => todo!(),
        }
    }
}
