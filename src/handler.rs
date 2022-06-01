use drive::scan::model::{query, Ok};
use drive::scan::QrCodeScanner;

pub(crate) fn login_handler() -> anyhow::Result<()> {
    let qr_scan = drive::scan::qr::QrCodeScanner::new();
    let gen = qr_scan.generator()?;
    let _ck_form: query::QueryQrCodeCkForm = gen.into();

    Ok(())
}
