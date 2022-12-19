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

mod directory_tree_view_page;
use directory_tree_view_page::DirectoryTreeViewPage;

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

mod rotated_label_page;
use rotated_label_page::RotatedLabelPage;

mod segmented_display_page;
use segmented_display_page::SegmentedDisplayPage;

mod standard_buttons_page;
use standard_buttons_page::StandardButtonsPage;

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

    #[strum(to_string = "DirectoryTreeView")]
    #[strum(props(feature = "filesystem"))]
    DirectoryTreeViewPage,

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

    #[strum(to_string = "RotatedLabel")]
    #[strum(props(feature = "ui"))]
    RotatedLabelPage,

    #[strum(to_string = "SegmentedDisplayWidget")]
    #[strum(props(feature = "displays"))]
    SegmentedDisplayPage,

    #[strum(to_string = "StandardButtons")]
    #[strum(props(feature = "ui"))]
    StandardButtonsPage,

    #[strum(to_string = "ThumbstickWidget")]
    #[strum(props(feature = "knobs"))]
    ThumbstickWidgetPage,

    #[strum(to_string = "Welcome")]
    WelcomePage,
}

impl PageId {
    pub fn create_page(&self) -> Box<dyn PageImpl> {
        match *self {
            PageId::AngleKnobPage => Box::<AngleKnobPage>::default(),
            PageId::AudioKnobPage => Box::<AudioKnobPage>::default(),
            PageId::BarcodePage => Box::<BarcodePage>::default(),
            PageId::DataMatrixPage => Box::<DataMatrixPage>::default(),
            PageId::DirectoryTreeViewPage => Box::<DirectoryTreeViewPage>::default(),
            PageId::HyperlinkWithIconPage => Box::<HyperlinkWithIconPage>::default(),
            PageId::IndicatorButtonPage => Box::<IndicatorButtonPage>::default(),
            PageId::LedDisplayPage => Box::<LedDisplayPage>::default(),
            PageId::LinearCompassPage => Box::<LinearCompassPage>::default(),
            PageId::PolarCompassPage => Box::<PolarCompassPage>::default(),
            PageId::QrCodePage => Box::<QrCodePage>::default(),
            PageId::RotatedLabelPage => Box::<RotatedLabelPage>::default(),
            PageId::SegmentedDisplayPage => Box::<SegmentedDisplayPage>::default(),
            PageId::StandardButtonsPage => Box::<StandardButtonsPage>::default(),
            PageId::ThumbstickWidgetPage => Box::<ThumbstickWidgetPage>::default(),
            PageId::WelcomePage => Box::<WelcomePage>::default(),
        }
    }
}
