use egui::{Response, Ui};
use strum::{Display, EnumIter};

#[derive(Clone, Copy, Debug, EnumIter, PartialEq, Display)]
pub enum ButtonKind {
    #[strum(to_string = "\u{2714} OK")]
    Ok,

    #[strum(to_string = "\u{1F5D9} Cancel")]
    Cancel,

    #[strum(to_string = "\u{2714} Apply")]
    Apply,

    #[strum(to_string = "\u{1F504} Reset")]
    Reset,
}

pub trait StandardButtons {
    fn standard_button(&mut self, button_kind: ButtonKind) -> Response;

    fn ok_button(&mut self) -> Response {
        self.standard_button(ButtonKind::Ok)
    }

    fn cancel_button(&mut self) -> Response {
        self.standard_button(ButtonKind::Cancel)
    }

    fn apply_button(&mut self) -> Response {
        self.standard_button(ButtonKind::Apply)
    }

    fn reset_button(&mut self) -> Response {
        self.standard_button(ButtonKind::Reset)
    }
}

impl StandardButtons for Ui {
    fn standard_button(&mut self, button_kind: ButtonKind) -> Response {
        // TODO: Consistent button width
        self.button(button_kind.to_string())
    }
}
