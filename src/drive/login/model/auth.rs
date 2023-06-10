#![allow(dead_code)]
use crate::drive::login::model::suc::GotoResponse;
use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct AppAccessToken {
    #[serde(rename = "token")]
    #[serde(default)]
    value: Option<String>,
}

impl From<&String> for AppAccessToken {
    fn from(token: &String) -> Self {
        Self {
            value: Some(token.to_string()),
        }
    }
}

#[derive(Serialize, Debug)]
pub struct AuthorizationCode {
    #[serde(default)]
    code: Option<String>,

    #[serde(rename = "loginType")]
    #[serde(default)]
    login_type: Option<String>,
}

impl From<GotoResponse> for AuthorizationCode {
    fn from(from: GotoResponse) -> Self {
        let code = from.extract_authorization_code();
        match code {
            Ok(code) => {
                log::debug!("authorization code: {}", &code);
                return Self {
                    code: Some(code),
                    login_type: Some(crate::drive::login::model::LOGIN_TYPE.to_string()),
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
