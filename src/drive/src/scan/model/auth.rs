#![allow(dead_code)]
use crate::scan::model::suc::GotoResponse;
use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct Token {
    #[serde(rename = "token")]
    #[serde(default)]
    value: Option<String>,
}

impl From<&String> for Token {
    fn from(token: &String) -> Self {
        Self {
            value: Some(token.to_string()),
        }
    }
}

#[derive(Serialize, Debug)]
pub struct AuthorizationCode {
    #[serde(rename = "code")]
    #[serde(default)]
    code: Option<String>,

    #[serde(rename = "loginType")]
    #[serde(default)]
    login_type: Option<String>,
}

impl From<&GotoResponse> for AuthorizationCode {
    fn from(from: &GotoResponse) -> Self {
        let code = from.extract_authorization_code();
        match code {
            Ok(code) => {
                log::debug!("authorization code: {}", &code);
                return Self {
                    code: Some(code),
                    login_type: Some(crate::scan::model::LOGIN_TYPE.to_string()),
                };
            }
            Err(e) => {
                log::error!("get authorization error: {}", e)
            }
        }
        Self {
            code: None,
            login_type: None,
        }
    }
}
