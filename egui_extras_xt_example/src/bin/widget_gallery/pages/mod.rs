pub mod datamatrix_page;
pub mod qrcode_page;

use eframe::egui::Ui;
use strum::{Display, EnumIter};

pub trait PageImpl {
    fn ui(&mut self, ui: &mut Ui);
}

// ----------------------------------------------------------------------------

#[derive(Clone, Copy, EnumIter, Eq, Hash, PartialEq, Display)]
pub enum PageId {
    QrCodePage,
    DataMatrixPage,
}

impl PageId {
    pub fn create_page(&self) -> Box<dyn PageImpl> {
        match *self {
            PageId::QrCodePage => Box::new(qrcode_page::QrCodePage::default()),
            PageId::DataMatrixPage => Box::new(datamatrix_page::DataMatrixPage::default()),
        }
    }
}
