use drive::scan::model::{query, Ok};

pub(crate) async fn login_handler() -> anyhow::Result<()> {
    let qr_scan = drive::scan::qr::QrCodeScanner::new().await?;
    let gen = qr_scan.generator().await?;
    let _ck_form: query::QueryQrCodeCkForm = gen.into();

    Ok(())
}

pub(crate) async fn get_mobile_token_handler() -> anyhow::Result<()> {
    drive::scan::qr::QrCodeScanner::new();
    Ok(())
}
