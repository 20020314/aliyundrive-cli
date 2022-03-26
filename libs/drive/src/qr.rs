use crate::models::*;

const GEN_API: &str = "https://passport.aliyundrive.com/newlogin/qrcode/generate.do?appName=aliyun_drive&fromSite=52&isMobile=true&lang=zh_CN&hsiz=1ebf2c3687ef453fedfbf0486671cd0c&_bx-v=2.0.31";
const QUERY_API: &str = "https://passport.aliyundrive.com/newlogin/qrcode/query.do?appName=aliyun_drive&fromSite=52&_bx-v=2.0.31";

#[allow(dead_code)]
pub enum State {
    CONFIRMED,
    EXPIRED,
    NEW,
}

impl ToString for State {
    fn to_string(&self) -> std::string::String {
        match self {
            State::NEW => "NEW".to_string(),
            State::EXPIRED => "EXPIRED".to_string(),
            State::CONFIRMED => "CONFIRMED".to_string(),
        }
    }
}

pub struct QrCodeScanner;

impl QrCodeScanner {
    pub fn get_generator_qrcode_content_result() -> Result<query::QueryQrCodeCkForm, reqwest::Error> {
        let resp = reqwest::blocking::get(GEN_API)?.json::<gen::GeneratorQrCodeResult>()?;
        Ok(query::QueryQrCodeCkForm::from(resp))
    }

    pub fn get_query_qrcode_result(
        from: &query::QueryQrCodeCkForm,
    ) -> Result<query::QueryQrCodeResult, reqwest::Error> {
        let client = reqwest::blocking::Client::new();
        let resp = client
            .post(QUERY_API)
            .form(&from.to_form())
            .send()?
            .json::<query::QueryQrCodeResult>()?;
        Ok(resp)
    }
}
