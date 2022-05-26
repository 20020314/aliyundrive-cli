use crate::models::*;
use anyhow::anyhow;
use reqwest::blocking::Response;
use crate::token::QrCodeScanner;

const GEN_API: &str = "https://passport.aliyundrive.com/newlogin/qrcode/generate.do?appName=aliyun_drive&fromSite=52&isMobile=true&lang=zh_CN&hsiz=1ebf2c3687ef453fedfbf0486671cd0c&_bx-v=2.0.31";
const QUERY_API: &str = "https://passport.aliyundrive.com/newlogin/qrcode/query.do?appName=aliyun_drive&fromSite=52&_bx-v=2.0.31";

#[allow(dead_code)]
pub enum State {
    CONFIRMED,
    EXPIRED,
    NEW,
}

impl ToString for State {
    fn to_string(&self) -> String {
        match self {
            State::NEW => "NEW".to_string(),
            State::EXPIRED => "EXPIRED".to_string(),
            State::CONFIRMED => "CONFIRMED".to_string(),
        }
    }
}

pub struct AliyunQrCodeScanner {
    referer: &'static str
}

impl AliyunQrCodeScanner {
    pub fn new() -> Self {
        Self {
            referer: "https://auth.aliyundrive.com/v2/oauth/authorize?client_id=25dzX3vbYqktVxyX&redirect_uri=https%3A%2F%2Fwww.aliyundrive.com%2Fsign%2Fcallback&response_type=code&login_type=custom&state=%7B%22origin%22%3A%22https%3A%2F%2Fwww.aliyundrive.com%22%7D"
        }
    }
}

impl QrCodeScanner for AliyunQrCodeScanner {

    fn get_generator_result() -> crate::Result<gen::GeneratorQrCodeResult> {
        let resp = reqwest::blocking::get(GEN_API)?;
        ResponseHandler::response_handler::<gen::GeneratorQrCodeResult>(resp)
    }

    fn get_query_result(
        from: &query::QueryQrCodeCkForm,
    ) -> crate::Result<query::QueryQrCodeResult> {
        let client = reqwest::blocking::Client::new();
        let resp = client.post(QUERY_API).form(&from.to_map()).send()?;
        ResponseHandler::response_handler::<query::QueryQrCodeResult>(resp)
    }
}

struct ResponseHandler;

impl ResponseHandler {

    #[allow(dead_code)]
    fn response_unit_handler(resp: Response) -> crate::Result<()> {
        if resp.status().is_success() {
            return Ok(());
        }
        let error_handler_msg = ResponseHandler::response_error_handler(resp);
        Err(anyhow!(error_handler_msg))
    }

    fn response_handler<T: serde::de::DeserializeOwned>(resp: Response) -> crate::Result<T> {
        if resp.status().is_success() {
            let result = resp.json::<T>()?;
            return Ok(result);
        }
        let error_handler_msg = ResponseHandler::response_error_handler(resp);
        Err(anyhow!(error_handler_msg))
    }

    fn response_error_handler(resp: Response) -> String {
        let msg = resp
            .text()
            .unwrap_or(String::from("An error occurred while extracting the body."));
        log::debug!(
            "defined in file: {}, defined on line: {}\nmessage: {:?}",
            file!(),
            line!(),
            &msg
        );
        msg
    }
}
