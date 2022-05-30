use crate::models::{query, Ok};
use serde::{Deserialize, Serialize};

// build qrcode result
#[derive(Debug, Serialize, Deserialize, Default)]
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

    pub fn get_content(&self) -> String {
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

impl Into<query::QueryQrCodeCkForm> for GeneratorQrCodeResult {
    fn into(self) -> query::QueryQrCodeCkForm {
        if let Some(ref content) = self.content {
            if let Some(ref data) = content.data {
                let ck = match &data.ck {
                    None => String::new(),
                    Some(ck) => ck.to_string(),
                };
                return query::QueryQrCodeCkForm::new((data.t, ck));
            }
        }
        query::QueryQrCodeCkForm::new((0, String::new()))
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
    data: Option<GeneratorQrCodeContentData>,

    #[serde(rename = "status")]
    #[serde(default)]
    status: i32,

    #[serde(rename = "success")]
    #[serde(default)]
    success: bool,
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
