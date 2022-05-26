use crate::models::Ok;
use serde::{Deserialize, Serialize};

// build qrcode result
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct GeneratorQrCodeResult {
    #[serde(default)]
    #[serde(rename = "content")]
    pub content: Option<GeneratorQrCodeContent>,

    #[serde(rename = "hasError")]
    #[serde(default)]
    pub has_error: bool,
}

impl GeneratorQrCodeResult {
    #[allow(dead_code)]
    fn new() -> Self {
        Self {
            content: None,
            has_error: false,
        }
    }
}

impl Ok for GeneratorQrCodeResult {
    fn ok(&self) -> bool {
        if let Some(ref t) = self.content {
            return !self.has_error && t.success;
        }
        !self.has_error
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct GeneratorQrCodeContent {
    #[serde(default)]
    #[serde(rename = "data")]
    pub data: Option<GeneratorQrCodeContentData>,

    #[serde(rename = "status")]
    #[serde(default)]
    pub status: i32,

    #[serde(rename = "success")]
    #[serde(default)]
    pub success: bool,
}

impl GeneratorQrCodeContent {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            data: None,
            status: 0,
            success: false,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct GeneratorQrCodeContentData {
    #[serde(rename = "t")]
    #[serde(default)]
    pub t: i64,

    #[serde(default)]
    #[serde(rename = "codeContent")]
    pub code_content: Option<String>,

    #[serde(default)]
    #[serde(rename = "ck")]
    pub ck: Option<String>,

    #[serde(default)]
    #[serde(rename = "resultCode")]
    pub result_code: i32,

    #[serde(default)]
    #[serde(rename = "titleMsg")]
    pub title_msg: Option<String>,

    #[serde(default)]
    #[serde(rename = "traceId")]
    pub trace_id: Option<String>,

    #[serde(default)]
    #[serde(rename = "errorCode")]
    pub error_code: Option<String>,

    #[serde(default)]
    #[serde(rename = "isMobile")]
    pub is_mobile: bool,
}

impl GeneratorQrCodeContentData {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            t: 0,
            code_content: None,
            ck: None,
            result_code: 0,
            title_msg: None,
            trace_id: None,
            error_code: None,
            is_mobile: false,
        }
    }
}
