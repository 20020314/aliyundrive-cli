use anyhow::Context;
use drive::scan;
use drive::scan::model::query::QueryQrCodeCkForm;
use drive::scan::model::{auth, AuthorizationToken, Ok};

pub(crate) async fn qrcode_token_handler(
    web_token: bool,
    app_token: bool,
) -> anyhow::Result<String> {
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
                        .get_mobile_login_result()
                        .context("failed to get mobile app login result")?;
                    log::debug!("app_login_response: {:#?}", app_login_response);
                    // Mobile App refresh token
                    let app_refresh_token = app_login_response
                        .refresh_token()
                        .context("failed to get mobile app refresh token")?;
                    if app_token || (!web_token) {
                        log::info!("app-refresh_token: {}\n", app_refresh_token);
                        return Ok(app_refresh_token);
                    }

                    if web_token {
                        let access_token = app_login_response
                            .access_token()
                            .context("failed to get web access token")?;
                        let web_login_response = scan
                            .do_get_web_token_response(auth::MobileAccessToken::from(&access_token))
                            .await
                            .context("failed to get web login result")?;
                        log::debug!("web_login_response: {:#?}", app_login_response);

                        let web_refresh_token = web_login_response
                            .refresh_token()
                            .context("failed to get web refresh token")?;
                        log::info!("web-refresh_token: {}\n", web_refresh_token);
                        return Ok(web_refresh_token);
                    }
                }
            }
        }
    }
    anyhow::bail!("Login failed");
}
