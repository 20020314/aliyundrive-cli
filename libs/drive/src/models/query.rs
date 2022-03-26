use super::gen;
use crate::models::Ok;
use serde::{Deserialize, Serialize};

// 查询二维码状态表单 Form
#[derive(Debug, Deserialize, Serialize)]
pub struct QueryQrCodeCkForm {
    pub t: i64,
    pub ck: String,
    pub code_content: String,
}

impl QueryQrCodeCkForm {
    pub fn new() -> Self {
        QueryQrCodeCkForm {
            t: 0,
            ck: String::from(""),
            code_content: String::from(""),
        }
    }

    pub fn from(gen: gen::GeneratorQrCodeResult) -> Self {
        let data = gen.content.data;
        QueryQrCodeCkForm {
            t: data.t,
            ck: data.ck,
            code_content: data.code_content,
        }
    }

    pub fn to_form(&self) -> std::collections::HashMap<String, String> {
        let mut params = std::collections::HashMap::<String, String>::new();
        params.insert("t".to_string(), self.t.to_string());
        params.insert("ck".to_string(), self.ck.to_string());
        params
    }

    pub fn get_code_content(&self) -> &String {
        &self.code_content
    }
}

// 查询二维码登录状态 Result
#[derive(Debug, Deserialize, Serialize)]
pub struct QueryQrCodeResult {
    #[serde(rename = "content")]
    pub content: QueryQrCodeContent,
    #[serde(default)]
    #[serde(rename = "hasError")]
    pub has_error: bool,
}

impl Ok for QueryQrCodeResult {
    fn ok(&self) -> bool {
        !self.has_error && self.content.success
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct QueryQrCodeContent {
    #[serde(rename = "data")]
    pub data: QueryQrCodeContentData,

    #[serde(default)]
    #[serde(rename = "status")]
    pub status: i32,

    #[serde(default)]
    #[serde(rename = "success")]
    pub success: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct QueryQrCodeContentData {
    #[serde(default)]
    #[serde(rename = "loginResult")]
    pub login_result: String,

    #[serde(default)]
    #[serde(rename = "loginSucResultAction")]
    pub login_suc_result_action: String,

    #[serde(default)]
    #[serde(rename = "st")]
    pub st: String,

    #[serde(default)]
    #[serde(rename = "qrCodeStatus")]
    pub qr_code_status: String,

    #[serde(default)]
    #[serde(rename = "loginType")]
    pub login_type: String,

    #[serde(default)]
    #[serde(rename = "bizExt")]
    pub biz_ext: String,

    #[serde(default)]
    #[serde(rename = "loginScene")]
    pub login_scene: String,

    #[serde(default)]
    #[serde(rename = "resultCode")]
    pub result_code: i32,

    #[serde(default)]
    #[serde(rename = "appEntrance")]
    pub app_entrance: String,

    #[serde(default)]
    #[serde(rename = "smartlock")]
    pub smart_lock: String,
}
