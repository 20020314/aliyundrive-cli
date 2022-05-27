use super::gen;
use crate::models::Ok;
use serde::{Deserialize, Serialize};

// query qrcode login status
#[derive(Debug, Deserialize, Serialize, Default)]
pub struct QueryQrCodeResult {
    #[serde(default)]
    #[serde(rename = "content")]
    pub content: Option<QueryQrCodeContent>,

    #[serde(default)]
    #[serde(rename = "hasError")]
    pub has_error: bool,
}

impl QueryQrCodeResult {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            content: None,
            has_error: false,
        }
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

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct QueryQrCodeContent {
    #[serde(rename = "data")]
    pub data: Option<QueryQrCodeContentData>,

    #[serde(default)]
    pub status: i32,

    #[serde(default)]
    pub success: bool,
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

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct QueryQrCodeContentData {
    #[serde(default)]
    #[serde(rename = "loginResult")]
    pub login_result: Option<String>,

    #[serde(default)]
    #[serde(rename = "loginSucResultAction")]
    pub login_suc_result_action: Option<String>,

    #[serde(default)]
    #[serde(rename = "st")]
    pub st: Option<String>,

    #[serde(default)]
    #[serde(rename = "qrCodeStatus")]
    pub qr_code_status: Option<String>,

    #[serde(default)]
    #[serde(rename = "loginType")]
    pub login_type: Option<String>,

    #[serde(default)]
    #[serde(rename = "bizExt")]
    pub biz_ext: Option<String>,

    #[serde(default)]
    #[serde(rename = "loginScene")]
    pub login_scene: Option<String>,

    #[serde(default)]
    #[serde(rename = "resultCode")]
    pub result_code: i32,

    #[serde(default)]
    #[serde(rename = "appEntrance")]
    pub app_entrance: Option<String>,

    #[serde(default)]
    #[serde(rename = "smartlock")]
    pub smart_lock: bool,
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
        let data = gen.content.unwrap_or_default().data.unwrap_or_default();
        QueryQrCodeCkForm {
            t: data.t,
            ck: data.ck.unwrap_or_default(),
            code_content: data.code_content.unwrap_or_default(),
        }
    }
}
