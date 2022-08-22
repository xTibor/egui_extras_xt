pub mod widget;

use barcoders::sym::codabar::Codabar;
use barcoders::sym::code11::Code11;
use barcoders::sym::code128::Code128;
use barcoders::sym::code39::Code39;
use barcoders::sym::code93::Code93;
use barcoders::sym::ean13::EAN13;
use barcoders::sym::ean8::EAN8;
use barcoders::sym::ean_supp::EANSUPP;
use barcoders::sym::tf::TF;

pub use widget::BarcodeWidget;

#[non_exhaustive]
#[derive(Clone, Copy, Eq, PartialEq)]
pub enum BarcodeKind {
    Codabar,
    Code11,
    Code39,
    Code39Checksum,
    Code93,
    Code128,
    EAN8,
    EAN13,
    EANSUPP,
    ITF,
    STF,
}

impl BarcodeKind {
    fn encode<T: AsRef<str>>(&self, data: T) -> Option<Vec<u8>> {
        match *self {
            BarcodeKind::Codabar => Some(Codabar::new(data).ok()?.encode()),
            BarcodeKind::Code11 => Some(Code11::new(data).ok()?.encode()),
            BarcodeKind::Code39 => Some(Code39::new(data).ok()?.encode()),
            BarcodeKind::Code39Checksum => Some(Code39::with_checksum(data).ok()?.encode()),
            BarcodeKind::Code93 => Some(Code93::new(data).ok()?.encode()),
            BarcodeKind::Code128 => Some(Code128::new(data).ok()?.encode()),
            BarcodeKind::EAN8 => Some(EAN8::new(data).ok()?.encode()),
            BarcodeKind::EAN13 => Some(EAN13::new(data).ok()?.encode()),
            BarcodeKind::EANSUPP => Some(EANSUPP::new(data).ok()?.encode()),
            BarcodeKind::ITF => Some(TF::interleaved(data).ok()?.encode()),
            BarcodeKind::STF => Some(TF::standard(data).ok()?.encode()),
        }
    }
}
