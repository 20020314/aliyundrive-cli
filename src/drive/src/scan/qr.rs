use crate::error::QrCodeScannerError;
use crate::r#const::{REQUEST_CONNECT_TIMEOUT, REQUEST_POOL_IDLE_TIMEOUT, REQUEST_TIMEOUT, UA};
use crate::scan::model::{auth, gen, query, suc, CkForm};
use crate::ScanResult;
use reqwest::Response;
use std::fmt::Debug;
use std::sync::Arc;
use tokio::sync::RwLock;

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

const SESSION_ID: &str = "SESSIONID";

pub struct QrCodeScanner {
    session: Arc<RwLock<String>>,
    client: reqwest::Client,
}

impl QrCodeScanner {
    pub async fn new() -> ScanResult<QrCodeScanner> {
        let client = reqwest::Client::builder()
            .user_agent(UA)
            .pool_idle_timeout(REQUEST_POOL_IDLE_TIMEOUT)
            .connect_timeout(REQUEST_CONNECT_TIMEOUT)
            .timeout(REQUEST_TIMEOUT)
            .build()?;

        Ok(Self {
            session: Arc::new(RwLock::new(String::new())),
            client,
        })
    }

    async fn init_session(&self) -> ScanResult<String> {
        let resp = self.client.get(SESSION_ID_API).send().await?;
        if resp.status().is_success() {
            for cookie in resp.cookies() {
                if cookie.name() == SESSION_ID {
                    return Ok(String::from(cookie.value()));
                }
            }
        }
        return Err(QrCodeScannerError::from("Failed to get session id."));
    }
}

impl QrCodeScanner {
    pub async fn qrcode_generator(&self) -> ScanResult<gen::GeneratorQrCodeResponse> {
        let resp = self.client.get(GENERATOR_QRCODE_API).send().await?;
        ResponseHandler::response_handler::<gen::GeneratorQrCodeResponse>(resp).await
    }

    pub async fn do_get_mobile_response(
        &self,
        from: &impl CkForm,
    ) -> ScanResult<query::QueryQrCodeResponse> {
        log::debug!("request ck form: {:#?}", from);
        let resp = self
            .client
            .post(QUERY_API)
            .form(&from.map_form())
            .send()
            .await?;
        ResponseHandler::response_handler::<query::QueryQrCodeResponse>(resp).await
    }

    pub async fn do_get_web_token_response(
        &mut self,
        token: auth::MobileAccessToken,
    ) -> ScanResult<suc::WebLoginResponse> {
        if self.session.read().await.is_empty() {
            let mut rw_session = self.session.write().await;
            let session_value = self.init_session().await?;
            rw_session.push_str(session_value.as_str())
        }
        let resp = self
            .client
            .post(TOKEN_LOGIN_API)
            .header(reqwest::header::COOKIE, format!("SESSIONID={}", ""))
            .json(&token)
            .send()
            .await?;
        let goto_response = ResponseHandler::response_handler::<suc::GotoResponse>(resp).await?;
        let authorization_code = auth::AuthorizationCode::from(goto_response);
        log::debug!("authorization code {:#?}", authorization_code);
        self.get_web_token(authorization_code).await
    }

    async fn get_web_token(
        &mut self,
        authorization_code: auth::AuthorizationCode,
    ) -> ScanResult<suc::WebLoginResponse> {
        let resp = self
            .client
            .post(GET_WEB_TOKEN_API)
            .header(reqwest::header::COOKIE, format!("SESSIONID={}", ""))
            .json(&authorization_code)
            .send()
            .await?;
        ResponseHandler::response_handler::<suc::WebLoginResponse>(resp).await
    }
}

struct ResponseHandler;

impl ResponseHandler {
    #[allow(dead_code)]
    async fn response_unit_handler(resp: Response) -> ScanResult<()> {
        if resp.status().is_success() {
            return Ok(());
        }
        let msg = ResponseHandler::response_error_msg_handler(resp).await;
        Err(QrCodeScannerError::from(msg))
    }

    async fn response_handler<T: serde::de::DeserializeOwned + Debug>(
        resp: Response,
    ) -> ScanResult<T> {
        if resp.status().is_success() {
            let result = resp.json::<T>().await?;
            log::debug!("response result: {:#?}", result);
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
