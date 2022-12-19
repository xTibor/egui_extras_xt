use egui::{Response, Ui};
use strum::{Display, EnumIter};

#[derive(Clone, Copy, Debug, EnumIter, PartialEq, Display)]
pub enum ButtonKind {
    #[strum(to_string = "\u{2714} OK")]
    Ok,

    #[strum(to_string = "\u{1F6AB} Cancel")]
    Cancel,

    #[strum(to_string = "\u{2714} Apply")]
    Apply,

    #[strum(to_string = "\u{1F504} Reset")]
    Reset,

    #[strum(to_string = "\u{1F5C1} Open")]
    Open,

    #[strum(to_string = "\u{1F4BE} Save")]
    Save,

    #[strum(to_string = "\u{1F4BE} Save As...")]
    SaveAs,

    #[strum(to_string = "\u{1F5D9} Close")]
    Close,

    #[strum(to_string = "\u{1F5D1} Delete")]
    Delete,

    #[strum(to_string = "\u{25B6} Play")]
    Play,

    #[strum(to_string = "\u{23F8} Pause")]
    Pause,

    #[strum(to_string = "\u{23F9} Stop")]
    Stop,

    #[strum(to_string = "\u{23FA} Record")]
    Record,

    #[strum(to_string = "\u{23ED} Next")]
    Next,

    #[strum(to_string = "\u{23EE} Previous")]
    Previous,

    #[strum(to_string = "\u{26F6} Full Screen")]
    FullScreen,

    #[strum(to_string = "\u{1F3B2} Random")]
    Random,

    #[strum(to_string = "\u{270F} Edit")]
    Edit,

    #[strum(to_string = "\u{2605} Favorite")]
    Favorite,

    #[strum(to_string = "\u{2606} Unfavorite")]
    Unfavorite,

    #[strum(to_string = "\u{1F507} Mute")]
    Mute,

    #[strum(to_string = "\u{1F50A} Unmute")]
    Unmute,

    #[strum(to_string = "\u{1F512} Lock")]
    Lock,

    #[strum(to_string = "\u{1F513} Unlock")]
    Unlock,

    #[strum(to_string = "\u{1F503} Refresh")]
    Refresh,

    #[strum(to_string = "\u{1F5CB} New")]
    New,

    #[strum(to_string = "\u{1F5D0} Copy")]
    Copy,

    #[strum(to_string = "\u{1F4CB} Paste")]
    Paste,

    #[strum(to_string = "\u{2702} Cut")]
    Cut,
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

    fn open_button(&mut self) -> Response {
        self.standard_button(ButtonKind::Open)
    }

    fn save_button(&mut self) -> Response {
        self.standard_button(ButtonKind::Save)
    }

    fn save_as_button(&mut self) -> Response {
        self.standard_button(ButtonKind::SaveAs)
    }

    fn close_button(&mut self) -> Response {
        self.standard_button(ButtonKind::Close)
    }

    fn delete_button(&mut self) -> Response {
        self.standard_button(ButtonKind::Delete)
    }

    fn play_button(&mut self) -> Response {
        self.standard_button(ButtonKind::Play)
    }

    fn pause_button(&mut self) -> Response {
        self.standard_button(ButtonKind::Pause)
    }

    fn stop_button(&mut self) -> Response {
        self.standard_button(ButtonKind::Stop)
    }

    fn record_button(&mut self) -> Response {
        self.standard_button(ButtonKind::Record)
    }

    fn next_button(&mut self) -> Response {
        self.standard_button(ButtonKind::Next)
    }

    fn previous_button(&mut self) -> Response {
        self.standard_button(ButtonKind::Previous)
    }

    fn full_screen_button(&mut self) -> Response {
        self.standard_button(ButtonKind::FullScreen)
    }

    fn random_button(&mut self) -> Response {
        self.standard_button(ButtonKind::Random)
    }

    fn edit_button(&mut self) -> Response {
        self.standard_button(ButtonKind::Edit)
    }

    fn favorite_button(&mut self) -> Response {
        self.standard_button(ButtonKind::Favorite)
    }

    fn unfavorite_button(&mut self) -> Response {
        self.standard_button(ButtonKind::Unfavorite)
    }

    fn mute_button(&mut self) -> Response {
        self.standard_button(ButtonKind::Mute)
    }

    fn unmute_button(&mut self) -> Response {
        self.standard_button(ButtonKind::Unmute)
    }

    fn lock_button(&mut self) -> Response {
        self.standard_button(ButtonKind::Lock)
    }

    fn unlock_button(&mut self) -> Response {
        self.standard_button(ButtonKind::Unlock)
    }

    fn refresh_button(&mut self) -> Response {
        self.standard_button(ButtonKind::Refresh)
    }

    fn new_button(&mut self) -> Response {
        self.standard_button(ButtonKind::New)
    }

    fn copy_button(&mut self) -> Response {
        self.standard_button(ButtonKind::Copy)
    }

    fn paste_button(&mut self) -> Response {
        self.standard_button(ButtonKind::Paste)
    }

    fn cut_button(&mut self) -> Response {
        self.standard_button(ButtonKind::Cut)
    }
}

impl StandardButtons for Ui {
    fn standard_button(&mut self, button_kind: ButtonKind) -> Response {
        self.button(button_kind.to_string())
    }
}
