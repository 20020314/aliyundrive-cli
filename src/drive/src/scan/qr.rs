use crate::error::QrCodeScannerError;
use crate::scan::model::{auth, gen, query, suc, CkForm};
use anyhow::anyhow;
use reqwest::Response;
use std::fmt::Debug;
use std::time;

// generator qrcode
const GENERATOR_QRCODE_API: &str = "https://passport.aliyundrive.com/newlogin/qrcode/generate.do?appName=aliyun_drive&fromSite=52&appEntrance=web";
// query scanner result (include mobile token)
const QUERY_API: &str = "https://passport.aliyundrive.com/newlogin/qrcode/query.do?appName=aliyun_drive&fromSite=52&_bx-v=2.0.31";
// get session id
const SESSION_ID_API: &str = "https://auth.aliyundrive.com/v2/oauth/authorize?client_id=25dzX3vbYqktVxyX&redirect_uri=https%3A%2F%2Fwww.aliyundrive.com%2Fsign%2Fcallback&response_type=code&login_type=custom&state=%7B%22origin%22%3A%22https%3A%2F%2Fwww.aliyundrive.com%22%7D#/nestedlogin?keepLogin=false&hidePhoneCode=true&ad__pass__q__rememberLogin=true&ad__pass__q__rememberLoginDefaultValue=true&ad__pass__q__forgotPassword=true&ad__pass__q__licenseMargin=true&ad__pass__q__loginType=normal";
// token login result（include authorization code）
const TOKEN_LOGIN_API: &str = "https://auth.aliyundrive.com/v2/oauth/token_login";
// get web side token
const GET_WEB_TOKEN_API: &str = "https://api.aliyundrive.com/token/get";

const UA: &str = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/100.0.4896.127 Safari/537.36";
const SESSION_ID: &str = "SESSIONID";

pub struct QrCodeScanner {
    session_id: String,
    client: reqwest::Client,
}

impl QrCodeScanner {
    pub async fn new() -> anyhow::Result<QrCodeScanner> {
        let client = reqwest::Client::builder()
            .pool_idle_timeout(time::Duration::from_secs(50))
            .connect_timeout(time::Duration::from_secs(10))
            .timeout(time::Duration::from_secs(30))
            .build()?;

        Ok(Self {
            session_id: String::new(),
            client,
        })
    }

    async fn init_session(&self) -> anyhow::Result<String> {
        let resp = self
            .client
            .get(SESSION_ID_API)
            .header(reqwest::header::USER_AGENT, UA)
            .send()
            .await?;
        if resp.status().is_success() {
            for cookie in resp.cookies() {
                if cookie.name() == SESSION_ID {
                    return Ok(String::from(cookie.value()));
                }
            }
        }
        return Err(anyhow!("Failed to get session id."));
    }
}

impl QrCodeScanner {
    pub async fn generator(&self) -> crate::ScanResult<gen::GeneratorQrCodeResult> {
        let resp = self.client.get(GENERATOR_QRCODE_API).send().await?;
        ResponseHandler::response_handler::<gen::GeneratorQrCodeResult>(resp).await
    }

    pub async fn query(&self, from: &impl CkForm) -> crate::ScanResult<query::QueryQrCodeResult> {
        log::debug!("request ck form: {:#?}", from);
        let resp = self
            .client
            .post(QUERY_API)
            .form(&from.map_form())
            .send()
            .await?;
        ResponseHandler::response_handler::<query::QueryQrCodeResult>(resp).await
    }

    pub async fn token_login(&mut self, token: auth::Token) -> crate::ScanResult<suc::GotoResult> {
        if self.session_id.is_empty() {
            self.session_id = self.init_session().await?
        }
        let resp = self
            .client
            .post(TOKEN_LOGIN_API)
            .header(
                reqwest::header::COOKIE,
                format!("SESSIONID={}", &self.session_id),
            )
            .json(&token)
            .send()
            .await?;
        ResponseHandler::response_handler::<suc::GotoResult>(resp).await
    }

    pub async fn get_token(
        &mut self,
        auth: auth::AuthorizationCode,
    ) -> crate::ScanResult<suc::WebLoginResult> {
        if self.session_id.is_empty() {
            self.session_id = self.init_session().await?
        }
        let resp = self
            .client
            .post(GET_WEB_TOKEN_API)
            .header(
                reqwest::header::COOKIE,
                format!("SESSIONID={}", &self.session_id),
            )
            .json(&auth)
            .send()
            .await?;
        ResponseHandler::response_handler::<suc::WebLoginResult>(resp).await
    }
}

struct ResponseHandler;

impl ResponseHandler {
    #[allow(dead_code)]
    async fn response_unit_handler(resp: Response) -> crate::ScanResult<()> {
        if resp.status().is_success() {
            return Ok(());
        }
        let msg = ResponseHandler::response_error_msg_handler(resp).await;
        Err(QrCodeScannerError::from(msg))
    }

    async fn response_handler<T: serde::de::DeserializeOwned + Debug>(
        resp: Response,
    ) -> crate::ScanResult<T> {
        if resp.status().is_success() {
            let result = resp.json::<T>().await?;
            log::debug!("response result: {:#?}", &result);
            return Ok(result);
        }
        let msg = ResponseHandler::response_error_msg_handler(resp).await;
        Err(QrCodeScannerError::from(msg))
    }

    async fn response_error_msg_handler(resp: Response) -> String {
        let msg = resp
            .text()
            .await
            .unwrap_or_else(|e| format!("An error occurred while extracting the body: {:?}", e));
        log::debug!(
            "defined in file: {}, defined on line: {}\nmessage: {:?}",
            file!(),
            line!(),
            &msg
        );
        msg
    }
}
