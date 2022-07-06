use anyhow::Context;
use drive::scan;
use drive::scan::model::query::QueryQrCodeCkForm;
use drive::scan::model::{auth, AuthorizationToken, Ok};

pub(crate) async fn qrcode_scan_handler(
    web_token: bool,
    app_token: bool,
) -> anyhow::Result<drive::conf::Authorization> {
    if web_token || app_token {
        let mut scan = scan::qr::QrCodeScanner::new().await?;
        // Return QR code content result set
        let generator_response = scan.qrcode_generator().await?;
        // Content that needs to generate a QR code
        let qrcode_content = generator_response.get_qrcode_content();
        let ck_form = QueryQrCodeCkForm::from(generator_response);
        // print QRCode
        qrcode::qr_print(&qrcode_content)?;
        println!("Please scan the qrcode to login in 30 seconds");
        for _i in 0..10 {
            tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
            // Simulate rotation training to query QRCode status
            let query_response = scan.do_get_query_response(&ck_form).await?;
            if query_response.ok() {
                // Indicates new status
                if query_response.is_new() {
                    log::debug!("qrcode is new");
                    continue;
                }
                // Indicates that the scan code was successful, but did not click to confirm the login
                if query_response.is_expired() {
                    log::debug!("login expired");
                    continue;
                }
                // The mobile APP scans the code successfully and confirms the login
                if query_response.is_confirmed() {
                    // Get the mobile login Result
                    let app_login_response = query_response
                        .get_app_login_result()
                        .context("failed to get mobile app login result")?;
                    log::debug!("app_login_response: {:#?}", app_login_response);
                    // Mobile App refresh token
                    let app_authorization = app_login_response
                        .clone()
                        .try_into()
                        .context("failed to get mobile app authentication")?;
                    if app_token || (!web_token) {
                        return Ok(app_authorization);
                    }

                    if web_token {
                        let app_access_token = app_login_response
                            .access_token()
                            .context("failed to get web access token")?;
                        let web_login_response = scan
                            .do_get_web_token_response(auth::AppAccessToken::from(
                                &app_access_token,
                            ))
                            .await
                            .context("failed to get web login result")?;
                        log::debug!("web_login_response: {:#?}", app_login_response);
                        let authorization = web_login_response
                            .try_into()
                            .context("failed to get web authentication")?;
                        return Ok(authorization);
                    }
                }
            }
        }
    }
    anyhow::bail!("Login failed");
}
