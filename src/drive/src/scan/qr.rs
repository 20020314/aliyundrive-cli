use crate::error::QrCodeScannerError;
use crate::r#const::{REQUEST_CONNECT_TIMEOUT, REQUEST_POOL_IDLE_TIMEOUT, REQUEST_TIMEOUT, UA};
use crate::scan::model::{auth, gen, query, suc, CkForm};
use crate::ScanResult;
use reqwest::Response;
use std::fmt::Debug;
use std::sync::Arc;
use std::time;
use tokio::sync::RwLock;

// generator qrcode
const GENERATOR_QRCODE_API: &str = "https://passport.aliyundrive.com/newlogin/qrcode/generate.do?appName=aliyun_drive&fromSite=52&appEntrance=web";
// query scanner result (include mobile token)
const QUERY_API: &str = "https://passport.aliyundrive.com/newlogin/qrcode/query.do?appName=aliyun_drive&fromSite=52&_bx-v=2.0.31";
// get session id
const SESSION_ID_API: &str = "https://auth.aliyundrive.com/v2/oauth/authorize?client_id=25dzX3vbYqktVxyX&redirect_uri=https%3A%2F%2Fwww.aliyundrive.com%2Fsign%2Fcallback&response_type=code&login_type=custom&state=%7B%22origin%22%3A%22https%3A%2F%2Fwww.aliyundrive.com%22%7D#/nestedlogin?keepLogin=false&hidePhoneCode=true&ad__pass__q__rememberLogin=true&ad__pass__q__rememberLoginDefaultValue=true&ad__pass__q__forgotPassword=true&ad__pass__q__licenseMargin=true&ad__pass__q__loginType=normal";
// mobile access token login result（include authorization code）
const MOBILE_TOKEN_LOGIN_API: &str = "https://auth.aliyundrive.com/v2/oauth/token_login";
// get web side token
const GET_WEB_TOKEN_API: &str = "https://api.aliyundrive.com/token/get";

const SESSION_ID: &str = "SESSIONID";

const GENERATOR_QRCODE_REFERER: &str = "https://passport.aliyundrive.com/mini_login.htm?lang=zh_cn&appName=aliyun_drive&appEntrance=web&styleType=auto&bizParams=&notLoadSsoView=false&notKeepLogin=false&isMobile=false&ad__pass__q__rememberLogin=true&ad__pass__q__rememberLoginDefaultValue=true&ad__pass__q__forgotPassword=true&ad__pass__q__licenseMargin=true&ad__pass__q__loginType=normal&hidePhoneCode=true&rnd=0.20099676922221987";

const QUERY_REFERER: &str = "https://passport.aliyundrive.com/mini_login.htm?lang=zh_cn&appName=aliyun_drive&appEntrance=web&styleType=auto&bizParams=&notLoadSsoView=false&notKeepLogin=false&isMobile=false&ad__pass__q__rememberLogin=true&ad__pass__q__rememberLoginDefaultValue=true&ad__pass__q__forgotPassword=true&ad__pass__q__licenseMargin=true&ad__pass__q__loginType=normal&hidePhoneCode=true&rnd=0.17778824737759047";
const QUERY_ORIGIN: &str = "https://passport.aliyundrive.com";

const MOBILE_TOKEN_LOGIN_REFERER: &str = "https://auth.aliyundrive.com";
const MOBILE_TOKEN_LOGIN_ORIGIN: &str = "https://auth.aliyundrive.com/v2/oauth/authorize?client_id=25dzX3vbYqktVxyX&redirect_uri=https%3A%2F%2Fwww.aliyundrive.com%2Fsign%2Fcallback&response_type=code&login_type=custom&state=%7B%22origin%22%3A%22https%3A%2F%2Fwww.aliyundrive.com%22%7D";

const GET_WEB_TOKEN_REFERER: &str = "https://www.aliyundrive.com";
const GET_WEB_TOKEN_ORIGIN: &str = "https://www.aliyundrive.com/";

#[derive(Clone)]
pub struct QrCodeScanner {
    session: Arc<RwLock<String>>,
    client: reqwest::Client,
}

impl QrCodeScanner {
    pub async fn new() -> anyhow::Result<QrCodeScanner> {
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

    async fn do_get_session_retry(&self) -> ScanResult<()> {
        for _i in 0..10 {
            match self.client.get(SESSION_ID_API).send().await {
                Ok(r) => {
                    if r.status().is_success() {
                        for cookie in r.cookies() {
                            if cookie.name() == SESSION_ID {
                                let mut session = self.session.write().await;
                                session.clear();
                                session.push_str(cookie.value());
                                return Ok(());
                            }
                        }
                    }
                }
                Err(e) => {
                    log::error!("get session failed: {}", e)
                }
            }
            tokio::time::sleep(time::Duration::from_secs(1)).await;
        }
        return Err(QrCodeScannerError::from("Failed to get session"));
    }

    // QRCode generation
    pub async fn qrcode_generator(&self) -> ScanResult<gen::GeneratorQrCodeResponse> {
        let resp = self
            .client
            .get(GENERATOR_QRCODE_API)
            .header(reqwest::header::REFERER, GENERATOR_QRCODE_REFERER)
            .send()
            .await?;
        ResponseHandler::response_handler::<gen::GeneratorQrCodeResponse>(resp).await
    }

    // Query QRCode scanning status
    pub async fn do_get_query_response(
        &self,
        from: &impl CkForm,
    ) -> ScanResult<query::QueryQrCodeResponse> {
        log::debug!("request ck form: {:#?}", from);
        let resp = self
            .client
            .post(QUERY_API)
            .header(reqwest::header::ORIGIN, QUERY_ORIGIN)
            .header(reqwest::header::REFERER, QUERY_REFERER)
            .form(&from.map_form())
            .send()
            .await?;
        ResponseHandler::response_handler::<query::QueryQrCodeResponse>(resp).await
    }

    //noinspection DuplicatedCode
    pub async fn do_get_web_token_response(
        &mut self,
        token: auth::MobileAccessToken,
    ) -> ScanResult<suc::WebLoginResponse> {
        self.do_get_session_retry().await?;
        let session_value = self.session.read().await.to_string();
        log::debug!("session value: {}", session_value);
        let resp = self
            .client
            .post(MOBILE_TOKEN_LOGIN_API)
            .header(reqwest::header::ORIGIN, MOBILE_TOKEN_LOGIN_ORIGIN)
            .header(reqwest::header::REFERER, MOBILE_TOKEN_LOGIN_REFERER)
            .header(
                reqwest::header::COOKIE,
                format!("SESSIONID={}", session_value),
            )
            .json(&token)
            .send()
            .await?;
        let goto_response = ResponseHandler::response_handler::<suc::GotoResponse>(resp).await?;
        log::debug!("goto response: {:#?}", goto_response);
        let authorization_code = auth::AuthorizationCode::from(goto_response);
        log::debug!("authorization code {:#?}", authorization_code);
        self.get_web_token_response(authorization_code).await
    }

    //noinspection DuplicatedCode
    async fn get_web_token_response(
        &self,
        authorization_code: auth::AuthorizationCode,
    ) -> ScanResult<suc::WebLoginResponse> {
        let session_value = self.session.read().await.to_string();
        log::debug!("session value: {}", session_value);
        let resp = self
            .client
            .post(GET_WEB_TOKEN_API)
            .header(reqwest::header::ORIGIN, GET_WEB_TOKEN_ORIGIN)
            .header(reqwest::header::REFERER, GET_WEB_TOKEN_REFERER)
            .header(
                reqwest::header::COOKIE,
                format!("SESSIONID={}", session_value),
            )
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
