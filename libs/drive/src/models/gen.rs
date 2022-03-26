use crate::models::Ok;
use serde::{Deserialize, Serialize};

// 构建二维码Data Result
#[derive(Debug, Serialize, Deserialize)]
pub struct GeneratorQrCodeResult {
    #[serde(rename = "content")]
    pub content: GeneratorQrCodeContent,

    #[serde(rename = "hasError")]
    #[serde(default)]
    pub has_error: bool,
}

#[allow(dead_code)]
impl GeneratorQrCodeResult {
    pub fn from(from: GeneratorQrCodeResult) -> GeneratorQrCodeResult {
        GeneratorQrCodeResult { ..from }
    }
}

impl Ok for GeneratorQrCodeResult {
    fn ok(&self) -> bool {
        !self.has_error && self.content.success
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GeneratorQrCodeContent {
    #[serde(rename = "data")]
    pub data: GeneratorQrCodeContentData,

    #[serde(rename = "status")]
    #[serde(default)]
    pub status: i32,

    #[serde(rename = "success")]
    #[serde(default)]
    pub success: bool,
}

#[allow(dead_code)]
impl GeneratorQrCodeContent {
    pub fn from(from: GeneratorQrCodeContent) -> GeneratorQrCodeContent {
        GeneratorQrCodeContent { ..from }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GeneratorQrCodeContentData {
    #[serde(rename = "t")]
    #[serde(default)]
    pub t: i64,

    #[serde(default)]
    #[serde(rename = "codeContent")]
    pub code_content: String,

    #[serde(default)]
    #[serde(rename = "ck")]
    pub ck: String,

    #[serde(default)]
    #[serde(rename = "resultCode")]
    pub result_code: i32,

    #[serde(default)]
    #[serde(rename = "titleMsg")]
    pub title_msg: String,

    #[serde(default)]
    #[serde(rename = "traceId")]
    pub trace_id: String,

    #[serde(default)]
    #[serde(rename = "errorCode")]
    pub error_code: String,

    #[serde(default)]
    #[serde(rename = "isMobile")]
    pub is_mobile: bool,
}

#[allow(dead_code)]
impl GeneratorQrCodeContentData {
    pub fn new(from: GeneratorQrCodeContentData) -> GeneratorQrCodeContentData {
        GeneratorQrCodeContentData { ..from }
    }
}
