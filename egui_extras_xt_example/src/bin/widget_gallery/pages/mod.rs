mod ui;

use eframe::egui::Ui;
use strum::{Display, EnumIter, EnumProperty};

// ----------------------------------------------------------------------------

mod angle_knob_page;
use angle_knob_page::AngleKnobPage;

mod audio_knob_page;
use audio_knob_page::AudioKnobPage;

mod barcode_page;
use barcode_page::BarcodePage;

mod datamatrix_page;
use datamatrix_page::DataMatrixPage;

mod hyperlink_with_icon_page;
use hyperlink_with_icon_page::HyperlinkWithIconPage;

mod indicator_button_page;
use indicator_button_page::IndicatorButtonPage;

mod led_display_page;
use led_display_page::LedDisplayPage;

mod linear_compass_page;
use linear_compass_page::LinearCompassPage;

mod polar_compass_page;
use polar_compass_page::PolarCompassPage;

mod qrcode_page;
use qrcode_page::QrCodePage;

mod segmented_display_page;
use segmented_display_page::SegmentedDisplayPage;

mod thumbstick_widget_page;
use thumbstick_widget_page::ThumbstickWidgetPage;

mod welcome_page;
use welcome_page::WelcomePage;

// ----------------------------------------------------------------------------

pub trait PageImpl {
    fn ui(&mut self, ui: &mut Ui);
}

#[allow(clippy::enum_variant_names)]
#[derive(Clone, Copy, Display, EnumIter, EnumProperty, Eq, Hash, PartialEq)]
pub enum PageId {
    #[strum(to_string = "AngleKnob")]
    #[strum(props(feature = "knobs"))]
    AngleKnobPage,

    #[strum(to_string = "AudioKnob")]
    #[strum(props(feature = "knobs"))]
    AudioKnobPage,

    #[strum(to_string = "BarcodeWidget")]
    #[strum(props(feature = "barcodes"))]
    BarcodePage,

    #[strum(to_string = "DataMatrixWidget")]
    #[strum(props(feature = "barcodes"))]
    DataMatrixPage,

    #[strum(to_string = "HyperlinkWithIcon")]
    #[strum(props(feature = "ui"))]
    HyperlinkWithIconPage,

    #[strum(to_string = "IndicatorButton")]
    #[strum(props(feature = "displays"))]
    IndicatorButtonPage,

    #[strum(to_string = "LedDisplay")]
    #[strum(props(feature = "displays"))]
    LedDisplayPage,

    #[strum(to_string = "LinearCompass")]
    #[strum(props(feature = "compasses"))]
    LinearCompassPage,

    #[strum(to_string = "PolarCompass")]
    #[strum(props(feature = "compasses"))]
    PolarCompassPage,

    #[strum(to_string = "QrCodeWidget")]
    #[strum(props(feature = "barcodes"))]
    QrCodePage,

    #[strum(to_string = "SegmentedDisplayWidget")]
    #[strum(props(feature = "displays"))]
    SegmentedDisplayPage,

    #[strum(to_string = "ThumbstickWidget")]
    #[strum(props(feature = "knobs"))]
    ThumbstickWidgetPage,

    #[strum(to_string = "Welcome")]
    WelcomePage,
}

impl PageId {
    pub fn create_page(&self) -> Box<dyn PageImpl> {
        match *self {
            PageId::AngleKnobPage => Box::new(AngleKnobPage::default()),
            PageId::AudioKnobPage => Box::new(AudioKnobPage::default()),
            PageId::BarcodePage => Box::new(BarcodePage::default()),
            PageId::DataMatrixPage => Box::new(DataMatrixPage::default()),
            PageId::HyperlinkWithIconPage => Box::new(HyperlinkWithIconPage::default()),
            PageId::IndicatorButtonPage => Box::new(IndicatorButtonPage::default()),
            PageId::LedDisplayPage => Box::new(LedDisplayPage::default()),
            PageId::LinearCompassPage => Box::new(LinearCompassPage::default()),
            PageId::PolarCompassPage => Box::new(PolarCompassPage::default()),
            PageId::QrCodePage => Box::new(QrCodePage::default()),
            PageId::SegmentedDisplayPage => Box::new(SegmentedDisplayPage::default()),
            PageId::ThumbstickWidgetPage => Box::new(ThumbstickWidgetPage::default()),
            PageId::WelcomePage => Box::new(WelcomePage::default()),
        }
    }
}
