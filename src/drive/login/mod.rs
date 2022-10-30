pub mod model;
pub mod qr;

use crate::drive::conf::Credentials;
use crate::drive::login::model::{auth, AuthorizationToken, Ok};
use crate::drive::login::model::query::QueryQrCodeCkForm;
use anyhow::{anyhow, Context};
use serde::{de, Deserialize, Deserializer, Serialize};
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum State {
    Confirmed,
    Expired,
    New,
}

impl FromStr for State {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use State::*;

        match s {
            "NEW" => Ok(New),
            "EXPIRED" => Ok(Expired),
            "CONFIRMED" => Ok(Confirmed),
            _ => Ok(Expired),
        }
    }
}

impl<'de> Deserialize<'de> for State {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        FromStr::from_str(&s).map_err(de::Error::custom)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum ClientType {
    Web,
    App,
}

impl ClientType {
    fn refresh_token_url(&self) -> &'static str {
        match self {
            ClientType::Web => "https://api.aliyundrive.com/token/refresh",
            ClientType::App => "https://auth.aliyundrive.com/v2/account/token",
        }
    }
}

impl FromStr for ClientType {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "web" => Ok(ClientType::Web),
            "app" => Ok(ClientType::App),
            _ => anyhow::bail!("invalid client type '{}'", s),
        }
    }
}

impl fmt::Display for ClientType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ClientType::Web => write!(f, "web"),
            ClientType::App => write!(f, "app"),
        }
    }
}

impl Default for ClientType {
    fn default() -> Self {
        ClientType::App
    }
}

pub struct QrCodeHandler;

impl QrCodeHandler {
    pub async fn qrcode_scan_handler(
        web_token: bool,
        app_token: bool,
    ) -> anyhow::Result<Credentials> {
        if web_token || app_token {
            let mut scan = qr::QrCodeScanner::new().await?;
            // Return QR code content result set
            let generator_response = scan.qrcode_generator().await?;
            // Content that needs to generate a QR code
            let qrcode_content = generator_response.get_qrcode_content();
            let ck_form = QueryQrCodeCkForm::from(generator_response);
            // print QRCode
            qr2term::print_qr(&qrcode_content)?;
            log::info!("Please scan the qrcode to login in 30 seconds");
            for _i in 0..10 {
                tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
                // Simulate rotation training to query QRCode status
                let response = scan.do_get_query_response(&ck_form).await?;
                if response.ok() {
                    // Indicates new status
                    if response.is_new() {
                        log::debug!("qrcode is new");
                        continue;
                    }
                    // Indicates that the scan code was successful, but did not click to confirm the login
                    if response.is_expired() {
                        log::debug!("login expired");
                        continue;
                    }
                    // The mobile app scans the code successfully and confirms the login
                    if response.is_confirmed() {
                        // Get the app login Result
                        let app_credentials = response
                            .get_app_login_result()
                            .context("failed to get mobile app login result")?
                            .clone()
                            .try_into()
                            .context("failed to get mobile app authentication")?;
                        if app_token || (!web_token) {
                            return Ok(app_credentials);
                        }

                        if web_token {
                            let app_access_token = app_credentials
                                .access_token()
                                .context("failed to get app access token")?;

                            let credentials = scan
                                .do_get_web_token_response(auth::AppAccessToken::from(
                                    &app_access_token,
                                ))
                                .await
                                .context("failed to get web login result")?
                                .try_into()
                                .context("failed to get web authentication")?;
                            return Ok(credentials);
                        }
                    }
                }
            }
        }
        anyhow::bail!("Login failed");
    }

    pub async fn print_qrcode_content_std() -> anyhow::Result<()> {
        let scan = qr::QrCodeScanner::new().await?;
        let result = scan.qrcode_generator().await?;
        let data = result.get_content_data().context("failed to get QRCode")?;
        println!("{}", serde_json::to_string_pretty(&data)?);
        Ok(())
    }

    pub async fn query_qrcode_app_login_result(t: i64, ck: String) -> anyhow::Result<Credentials> {
        let scan = qr::QrCodeScanner::new().await?;
        let form = QueryQrCodeCkForm::new((t, ck));
        let query_result = scan.do_get_query_response(&form).await?;
        if query_result.is_confirmed() {
            let credentials = query_result
                .get_app_login_result()
                .context("failed to get app login result")?
                .try_into()
                .context("failed to get app credentials")?;
            return Ok(credentials);
        }
        Err(anyhow!("failed to get app credentials"))
    }

    pub async fn query_qrcode_web_login_result(t: i64, ck: String) -> anyhow::Result<Credentials> {
        let mut scan = qr::QrCodeScanner::new().await?;
        let form = QueryQrCodeCkForm::new((t, ck));
        let query_result = scan.do_get_query_response(&form).await?;
        if query_result.is_confirmed() {
            let app_access_token = query_result
                .get_app_login_result()
                .context("failed to get app login result")?
                .access_token()
                .context("failed to get app access token")?;

            let credentials = scan
                .do_get_web_token_response(auth::AppAccessToken::from(&app_access_token))
                .await
                .context("failed to get web login result")?
                .try_into()
                .context("failed to get web credentials")?;
            return Ok(credentials);
        }
        Err(anyhow!("failed to get web credentials"))
    }
}
