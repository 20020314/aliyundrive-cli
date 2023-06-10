#![allow(dead_code)]

use crate::drive::login::model::gen::GeneratorQrCodeResponse;
use crate::drive::login::model::{suc, CkForm, Ok};
use crate::drive::login::State;
use serde::{Deserialize, Serialize};

// query qrcode scan status
#[derive(Debug, Deserialize)]
pub struct QueryQrCodeResponse {
    #[serde(default)]
    content: Option<QueryQrCodeContent>,

    #[serde(default)]
    #[serde(rename = "hasError")]
    has_error: bool,
}

impl QueryQrCodeResponse {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            content: None,
            has_error: false,
        }
    }

    pub fn get_app_login_result(&self) -> Option<suc::AppLoginResponse> {
        let biz_ext = self.get_biz_ext()?;
        let vec = base64::decode(biz_ext).unwrap();
        let string = vec.iter().map(|&c| c as char).collect::<String>();
        serde_json::from_str::<suc::AppLoginResponse>(string.as_str()).ok()
    }

    fn get_biz_ext(&self) -> Option<String> {
        let content = self.content.as_ref()?;
        let data = content.data.as_ref()?;
        let biz_ext = data.biz_ext.as_ref()?;
        Some(biz_ext.to_string())
    }

    fn get_status(&self) -> Option<State> {
        let content = self.content.as_ref()?;
        let data = content.data.as_ref()?;
        let state = data.qr_code_status.as_ref().cloned()?;
        Some(state)
    }
}

impl Ok for QueryQrCodeResponse {
    fn ok(&self) -> bool {
        if let Some(ref t) = self.content {
            return !self.has_error && t.success;
        }
        !self.has_error
    }
}

impl QueryQrCodeResponse {
    pub fn is_new(&self) -> bool {
        if let Some(ref state) = self.get_status() {
            if State::New.eq(state) {
                return true;
            }
        }
        false
    }

    pub fn is_expired(&self) -> bool {
        if let Some(ref state) = self.get_status() {
            if State::Expired.eq(state) {
                return true;
            }
        }
        false
    }

    pub fn is_confirmed(&self) -> bool {
        if let Some(ref state) = self.get_status() {
            if State::Confirmed.eq(state) {
                return true;
            }
        }
        false
    }
}

#[derive(Debug, Deserialize)]
pub struct QueryQrCodeContent {
    #[serde(default)]
    data: Option<QueryQrCodeContentData>,

    #[serde(default)]
    status: i32,

    #[serde(default)]
    success: bool,
}

impl QueryQrCodeContent {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            data: None,
            status: 0,
            success: false,
        }
    }
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct QueryQrCodeContentData {
    #[serde(default)]
    #[serde(rename = "loginResult")]
    login_result: Option<String>,

    #[serde(default)]
    #[serde(rename = "loginSucResultAction")]
    login_suc_result_action: Option<String>,

    #[serde(default)]
    st: Option<String>,

    #[serde(default)]
    #[serde(rename = "qrCodeStatus")]
    qr_code_status: Option<State>,

    #[serde(default)]
    #[serde(rename = "loginType")]
    login_type: Option<String>,

    #[serde(default)]
    #[serde(rename = "bizExt")]
    biz_ext: Option<String>,

    #[serde(default)]
    #[serde(rename = "loginScene")]
    login_scene: Option<String>,

    #[serde(default)]
    #[serde(rename = "resultCode")]
    result_code: i32,

    #[serde(default)]
    #[serde(rename = "appEntrance")]
    app_entrance: Option<String>,

    #[serde(default)]
    #[serde(rename = "smartlock")]
    smart_lock: bool,
}

// query qrcode status form
#[derive(Debug, Serialize, Default)]
pub struct QueryQrCodeCkForm {
    t: i64,
    ck: String,
}

impl QueryQrCodeCkForm {
    pub fn new(from: (i64, String)) -> Self {
        Self {
            t: from.0,
            ck: from.1,
        }
    }
}

impl From<GeneratorQrCodeResponse> for QueryQrCodeCkForm {
    fn from(from: GeneratorQrCodeResponse) -> Self {
        if let Some(ref content) = from.get_content() {
            if let Some(ref data) = content.get_data() {
                let ck = match data.get_ck() {
                    None => String::new(),
                    Some(ck) => ck,
                };
                return Self {
                    t: data.get_t(),
                    ck,
                };
            }
        }
        Self {
            t: 0,
            ck: String::new(),
        }
    }
}

impl CkForm for QueryQrCodeCkForm {
    fn map_form(&self) -> std::collections::HashMap<String, String> {
        let mut ck_map = std::collections::HashMap::<String, String>::new();
        ck_map.insert(crate::drive::login::model::T_KEY.to_string(), self.t.to_string());
        ck_map.insert(crate::drive::login::model::CK_KEY.to_string(), self.ck.to_string());
        ck_map
    }
}
