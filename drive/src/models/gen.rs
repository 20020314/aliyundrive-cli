#![allow(dead_code)]

use crate::models::Ok;
use serde::Deserialize;

// build qrcode result
#[derive(Debug, Deserialize)]
pub struct GeneratorQrCodeResult {
    #[serde(default)]
    #[serde(rename = "content")]
    content: Option<GeneratorQrCodeContent>,

    #[serde(rename = "hasError")]
    #[serde(default)]
    has_error: bool,
}

impl GeneratorQrCodeResult {
    #[allow(dead_code)]
    fn new() -> Self {
        Self {
            content: None,
            has_error: false,
        }
    }

    pub fn get_content(&self) -> Option<GeneratorQrCodeContent> {
        self.content.as_ref().cloned()
    }

    pub fn get_qrcode_content(&self) -> String {
        if let Some(ref content) = self.content {
            if let Some(ref data) = content.data {
                let code_content = match &data.code_content {
                    None => String::new(),
                    Some(code_content) => code_content.to_string(),
                };
                return code_content;
            }
        }
        String::new()
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

#[derive(Debug, Deserialize, Clone)]
pub struct GeneratorQrCodeContent {
    #[serde(default)]
    #[serde(rename = "data")]
    data: Option<GeneratorQrCodeContentData>,

    #[serde(rename = "status")]
    #[serde(default)]
    status: i32,

    #[serde(rename = "success")]
    #[serde(default)]
    success: bool,
}

impl GeneratorQrCodeContent {
    pub fn get_data(&self) -> Option<GeneratorQrCodeContentData> {
        self.data.as_ref().cloned()
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct GeneratorQrCodeContentData {
    #[serde(rename = "t")]
    #[serde(default)]
    t: i64,

    #[serde(default)]
    #[serde(rename = "codeContent")]
    code_content: Option<String>,

    #[serde(default)]
    #[serde(rename = "ck")]
    ck: Option<String>,

    #[serde(default)]
    #[serde(rename = "resultCode")]
    result_code: i32,

    #[serde(default)]
    #[serde(rename = "titleMsg")]
    title_msg: Option<String>,

    #[serde(default)]
    #[serde(rename = "traceId")]
    trace_id: Option<String>,

    #[serde(default)]
    #[serde(rename = "errorCode")]
    error_code: Option<String>,

    #[serde(default)]
    #[serde(rename = "isMobile")]
    is_mobile: bool,
}

impl GeneratorQrCodeContentData {
    pub fn get_ck(&self) -> Option<String> {
        self.ck.as_ref().cloned()
    }

    pub fn get_t(&self) -> i64 {
        self.t
    }
}
