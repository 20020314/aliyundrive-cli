use super::gen;
use crate::login::{QrCodeScannerState, State};
use crate::models::Ok;
use serde::{Deserialize, Serialize};

// query qrcode login status
#[derive(Debug, Deserialize, Serialize, Default)]
pub struct QueryQrCodeResult {
    #[serde(default)]
    #[serde(rename = "content")]
    content: Option<QueryQrCodeContent>,

    #[serde(default)]
    #[serde(rename = "hasError")]
    has_error: bool,
}

impl QueryQrCodeResult {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            content: None,
            has_error: false,
        }
    }

    pub fn get_biz_ext(&self) -> Option<String> {
        if let Some(ref content) = self.content {
            if let Some(ref data) = content.data {
                if let Some(ref biz_ext) = data.biz_ext {
                    return Some(biz_ext.to_string());
                }
            }
        }
        None
    }

    fn get_status(&self) -> Option<String> {
        if let Some(ref content) = self.content {
            if let Some(ref data) = content.data {
                if let Some(ref state) = data.qr_code_status {
                    return Some(state.to_string())
                }
            }
        }
        None
    }
}

impl Ok for QueryQrCodeResult {
    fn ok(&self) -> bool {
        if let Some(ref t) = self.content {
            return !self.has_error && t.success;
        }
        !self.has_error
    }
}

impl QrCodeScannerState for QueryQrCodeResult {
    fn is_new(&self) -> bool {
        if let Some(ref state) = self.get_status() {
            if State::NEW.eq(state) {
                return true;
            }
        }
        false
    }

    fn is_expired(&self) -> bool {
        if let Some(ref state) = self.get_status() {
            if State::EXPIRED.eq(state) {
                return true;
            }
        }
        false
    }

    fn is_confirmed(&self) -> bool {
        if let Some(ref state) = self.get_status() {
            if State::CONFIRMED.eq(state) {
                return true;
            }
        }
        false
    }
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct QueryQrCodeContent {
    #[serde(rename = "data")]
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

#[derive(Debug, Deserialize, Serialize, Default, PartialEq)]
pub struct QueryQrCodeContentData {
    #[serde(default)]
    #[serde(rename = "loginResult")]
    login_result: Option<String>,

    #[serde(default)]
    #[serde(rename = "loginSucResultAction")]
    login_suc_result_action: Option<String>,

    #[serde(default)]
    #[serde(rename = "st")]
    st: Option<String>,

    #[serde(default)]
    #[serde(rename = "qrCodeStatus")]
    qr_code_status: Option<String>,

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

impl QueryQrCodeContentData {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            login_result: None,
            login_suc_result_action: None,
            st: None,
            qr_code_status: None,
            login_type: None,
            biz_ext: None,
            login_scene: None,
            result_code: 0,
            app_entrance: None,
            smart_lock: false,
        }
    }
}

// query qrcode status form
#[derive(Debug, Deserialize, Serialize, Default)]
pub struct QueryQrCodeCkForm {
    t: i64,
    ck: String,
    code_content: String,
}

impl QueryQrCodeCkForm {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            t: 0,
            ck: String::new(),
            code_content: String::new(),
        }
    }

    pub fn get_content(&self) -> &str {
        self.code_content.as_str()
    }

    pub fn to_map(&self) -> std::collections::HashMap<String, String> {
        let mut params = std::collections::HashMap::<String, String>::new();
        params.insert("t".to_string(), self.t.to_string());
        params.insert("ck".to_string(), self.ck.to_string());
        params
    }
}

impl From<gen::GeneratorQrCodeResult> for QueryQrCodeCkForm {
    fn from(gen: gen::GeneratorQrCodeResult) -> Self {
        let data = gen.get_tuple();
        QueryQrCodeCkForm {
            t: data.0,
            ck: data.1,
            code_content: data.2,
        }
    }
}
