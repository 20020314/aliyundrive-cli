use anyhow::{Context};
use drive::scan;
use drive::scan::model::auth::{AuthorizationCode, Token};
use drive::scan::model::query::QueryQrCodeCkForm;
use drive::scan::model::{AuthorizationToken, Ok};
use crate::conf::rw::RW;

pub(crate) async fn qrcode_token_handler(
    web_token: bool,
    mobile_token: bool,
    sava: bool
) -> anyhow::Result<()> {
    if web_token || mobile_token {
        let mut scan = scan::qr::QrCodeScanner::new().await?;
        // Return QR code content result set
        let generator_result = scan.generator().await?;
        // Content that needs to generate a QR code
        let qrcode_content = generator_result.get_qrcode_content();
        let ck_form = QueryQrCodeCkForm::from(generator_result);
        // print QRCode
        qrcode::qr_print(&qrcode_content)?;
        log::info!("Please scan the qrcode to login in 30 seconds");
        for _i in 0..10 {
            tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
            // Simulate rotation training to query QRCode status
            let query_result = scan.query(&ck_form).await?;
            if query_result.ok() {
                // Indicates new status
                if query_result.is_new() {
                    continue;
                }
                // Indicates that the scan code was successful, but did not click to confirm the login
                if query_result.is_expired() {
                    log::debug!("login expired");
                    continue;
                }
                // The mobile APP scans the code successfully and confirms the login
                if query_result.is_confirmed() {
                    // Get the mobile login Result
                    let mobile_login_result = query_result
                        .get_mobile_login_result()
                        .context("failed to get mobile login result")?;
                    // Mobile refresh token
                    let mobile_refresh_token = mobile_login_result
                        .refresh_token()
                        .context("failed to get mobile refresh token")?;
                    if mobile_token || (!web_token){
                        log::info!("mobile-refresh_token: {}\n", mobile_refresh_token);
                        return Ok(());
                    }

                    let web_refresh_token = if web_token {
                        let access_token = mobile_login_result.access_token()
                            .context("failed to get web access token")?;
                        let goto_result = scan.token_login(Token::from(&access_token)).await?;
                        let web_login_result = scan
                            .get_token(AuthorizationCode::from(&goto_result))
                            .await
                            .context("failed to get web login result")?;

                        let refresh_token = web_login_result
                            .refresh_token()
                            .context("failed to get web refresh token")?;
                        log::info!("web-refresh_token: {}\n", refresh_token);
                        Some(refresh_token)
                    } else {
                        None
                    };

                    if sava {
                        crate::Context::init()?;
                        let web_authorization_token = crate::conf::AuthorizationToken::new(None, web_refresh_token);
                        let mobile_authorization_token = crate::conf::AuthorizationToken::new(None, Some(mobile_refresh_token));
                        let config = crate::conf::Config::new(Some(web_authorization_token), Some(mobile_authorization_token));
                        crate::Context::write(config)
                            .context("Failed to save configuration!")?
                    }
                    return Ok(());
                }
            }
        }
    }
    anyhow::bail!("Login failed");
}
