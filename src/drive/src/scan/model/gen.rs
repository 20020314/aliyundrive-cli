#![allow(dead_code)]

use crate::scan::model::Ok;
use serde::{Deserialize, Serialize};

// build qrcode result
#[derive(Deserialize, Serialize, Debug)]
pub struct GeneratorQrCodeResponse {
    #[serde(default)]
    content: Option<GeneratorQrCodeContent>,

    #[serde(rename = "hasError")]
    #[serde(default)]
    has_error: bool,
}

impl GeneratorQrCodeResponse {
    pub fn get_content(&self) -> Option<&GeneratorQrCodeContent> {
        self.content.as_ref()
    }

    pub fn get_content_data(self) -> Option<GeneratorQrCodeContentData> {
        self.content?.data
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

impl Ok for GeneratorQrCodeResponse {
    fn ok(&self) -> bool {
        if let Some(ref t) = self.content {
            return !self.has_error && t.success;
        }
        !self.has_error
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GeneratorQrCodeContent {
    #[serde(default)]
    data: Option<GeneratorQrCodeContentData>,

    #[serde(default)]
    status: i32,

    #[serde(default)]
    success: bool,
}

impl GeneratorQrCodeContent {
    pub fn get_data(&self) -> Option<GeneratorQrCodeContentData> {
        self.data.as_ref().cloned()
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GeneratorQrCodeContentData {
    #[serde(default)]
    t: i64,

    #[serde(default)]
    #[serde(rename = "codeContent")]
    code_content: Option<String>,

    #[serde(default)]
    ck: Option<String>,
}

impl GeneratorQrCodeContentData {
    pub fn get_ck(&self) -> Option<String> {
        self.ck.as_ref().cloned()
    }

    pub fn get_t(&self) -> i64 {
        self.t
    }
}
