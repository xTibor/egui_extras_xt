pub mod ui;

pub mod audio_knob_page;
pub mod barcode_page;
pub mod datamatrix_page;
pub mod hyperlink_with_icon_page;
pub mod led_display_page;
pub mod qrcode_page;
pub mod segmented_display_page;
pub mod thumbstick_knob_page;

use eframe::egui::Ui;
use strum::{Display, EnumIter, EnumProperty};

pub trait PageImpl {
    fn ui(&mut self, ui: &mut Ui);
}

// ----------------------------------------------------------------------------

#[derive(Clone, Copy, Display, EnumIter, EnumProperty, Eq, Hash, PartialEq)]
pub enum PageId {
    #[strum(to_string = "QrCodeWidget")]
    #[strum(props(feature = "barcodes"))]
    QrCodePage,

    #[strum(to_string = "DataMatrixWidget")]
    #[strum(props(feature = "barcodes"))]
    DataMatrixPage,

    #[strum(to_string = "BarcodeWidget")]
    #[strum(props(feature = "barcodes"))]
    BarcodePage,

    #[strum(to_string = "HyperlinkWithIcon")]
    #[strum(props(feature = "ui"))]
    HyperlinkWithIconPage,

    #[strum(to_string = "AudioKnob")]
    #[strum(props(feature = "knobs"))]
    AudioKnobPage,

    #[strum(to_string = "ThumbstickKnob")]
    #[strum(props(feature = "knobs"))]
    ThumbstickKnobPage,

    #[strum(to_string = "LedDisplay")]
    #[strum(props(feature = "displays"))]
    LedDisplayPage,

    #[strum(to_string = "SegmentedDisplayWidget")]
    #[strum(props(feature = "displays"))]
    SegmentedDisplayPage,
}

impl PageId {
    pub fn create_page(&self) -> Box<dyn PageImpl> {
        match *self {
            PageId::QrCodePage => Box::new(qrcode_page::QrCodePage::default()),
            PageId::DataMatrixPage => Box::new(datamatrix_page::DataMatrixPage::default()),
            PageId::BarcodePage => Box::new(barcode_page::BarcodePage::default()),
            PageId::HyperlinkWithIconPage => {
                Box::new(hyperlink_with_icon_page::HyperlinkWithIconPage::default())
            }
            PageId::AudioKnobPage => Box::new(audio_knob_page::AudioKnobPage::default()),
            PageId::ThumbstickKnobPage => {
                Box::new(thumbstick_knob_page::ThumbstickKnobPage::default())
            }
            PageId::LedDisplayPage => Box::new(led_display_page::LedDisplayPage::default()),
            PageId::SegmentedDisplayPage => {
                Box::new(segmented_display_page::SegmentedDisplayPage::default())
            }
        }
    }
}
