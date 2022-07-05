use anyhow::Context;
use drive::scan;
use drive::scan::model::query::QueryQrCodeCkForm;
use drive::scan::model::{auth, AuthorizationToken, Ok};

pub(crate) async fn subcommands_handler(
    subcommands: &Option<crate::Commands>,
) -> anyhow::Result<()> {
    use crate::Commands;
    use crate::QrCommand;
    match &subcommands {
        Some(Commands::QR(QrCommand::Login { app, web, sava })) => {
            // qrcode scan
            if *web || *app {
                let refresh_token = qrcode_token_handler(*web, *app).await?;
                // Sava the authorization token to config file
                if *sava {
                    // saa::Context::init()?;
                    // let authorization_token = saa::Authorization::new(None, Some(refresh_token));
                    // saa::Context::write_token(*mobile_token, authorization_token)
                    //     .context("Failed to save configuration!")?
                }
            }
        }
        Some(Commands::QR(QrCommand::Generate)) => {
            let scan = scan::qr::QrCodeScanner::new().await?;
            let result = scan.qrcode_generator().await?;
            let data = result.get_content_data().context("failed to get QRCode")?;
            println!("{}", serde_json::to_string_pretty(&data)?);
        }
        Some(Commands::QR(QrCommand::Query { t, ck })) => {
            let scan = scan::qr::QrCodeScanner::new().await?;
            let form = QueryQrCodeCkForm::new((*t, ck.to_string()));
            let query_result = scan.do_get_query_response(&form).await?;
            if query_result.is_confirmed() {
                let refresh_token = query_result
                    .get_app_login_result()
                    .context("failed to get mobile login result")?
                    .refresh_token()
                    .context("failed to get refresh token")?;
                println!("{}", refresh_token)
            }
        }
        Some(Commands::Config { cat }) => {

        }
        Some(Commands::Daemon) => {
            loop {
                log::info!("daemon running..");
                tokio::time::sleep(tokio::time::Duration::from_secs(10)).await
            }
        }
        None => {}
    }

    Ok(())
}

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
                        .get_app_login_result()
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
