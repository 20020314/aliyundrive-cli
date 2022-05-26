pub mod qr;
use crate::models::{gen, query};

pub trait QrCodeScanner {

    // get qrcode generator result contend.
    fn get_generator_result() -> crate::Result<gen::GeneratorQrCodeResult>;

    // query qrcode result
    fn get_query_result(from: &query::QueryQrCodeCkForm)
        -> crate::Result<query::QueryQrCodeResult>;


}
