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
    fn encode<T: AsRef<str>>(&self, data: T) -> Vec<u8> {
        match *self {
            BarcodeKind::Codabar => Codabar::new(data).unwrap().encode(),
            BarcodeKind::Code11 => Code11::new(data).unwrap().encode(),
            BarcodeKind::Code39 => Code39::new(data).unwrap().encode(),
            BarcodeKind::Code39Checksum => Code39::with_checksum(data).unwrap().encode(),
            BarcodeKind::Code93 => Code93::new(data).unwrap().encode(),
            BarcodeKind::Code128 => Code128::new(data).unwrap().encode(),
            BarcodeKind::EAN8 => EAN8::new(data).unwrap().encode(),
            BarcodeKind::EAN13 => EAN13::new(data).unwrap().encode(),
            BarcodeKind::EANSUPP => EANSUPP::new(data).unwrap().encode(),
            BarcodeKind::ITF => TF::interleaved(data).unwrap().encode(),
            BarcodeKind::STF => TF::standard(data).unwrap().encode(),
        }
    }
}
