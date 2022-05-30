#![allow(dead_code)]

use crate::models::{suc, CkForm, Ok};
use crate::scan::{QrCodeScannerState, State};
use serde::{Deserialize, Serialize};

// query qrcode scan status
#[derive(Debug, Deserialize, Default)]
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

    pub fn get_mobile_login_result(&self) -> Option<suc::MobileLoginResult> {
        let biz_ext = self.get_biz_ext()?;
        let vec = base64::decode(biz_ext).unwrap();
        let string = vec.iter().map(|&c| c as char).collect::<String>();
        serde_json::from_str::<suc::MobileLoginResult>(string.as_str()).ok()
    }

    fn get_biz_ext(&self) -> Option<String> {
        let content = self.content.as_ref()?;
        let data = content.data.as_ref()?;
        let biz_ext = data.biz_ext.as_ref()?;
        Some(biz_ext.to_string())
    }

    fn get_status(&self) -> Option<String> {
        let content = self.content.as_ref()?;
        let data = content.data.as_ref()?;
        let state = data.qr_code_status.as_ref()?;
        Some(state.to_string())
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

#[derive(Debug, Deserialize, Default)]
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

#[derive(Debug, Deserialize, Default, PartialEq)]
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

impl CkForm for QueryQrCodeCkForm {
    fn map_form(&self) -> std::collections::HashMap<String, String> {
        let mut ck_map = std::collections::HashMap::<String, String>::new();
        ck_map.insert(crate::models::T_KEY.to_string(), self.t.to_string());
        ck_map.insert(crate::models::CK_KEY.to_string(), self.ck.to_string());
        ck_map
    }
}
