use crate::models::suc::GotoResult;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Token {
    #[serde(rename = "login")]
    #[serde(default)]
    value: Option<String>,
}

impl From<String> for Token {
    fn from(token: String) -> Self {
        Self { value: Some(token) }
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct AuthorizationCode {
    #[serde(rename = "login")]
    #[serde(default)]
    code: Option<String>,

    #[serde(rename = "loginType")]
    #[serde(default)]
    login_type: Option<String>,
}

impl From<GotoResult> for AuthorizationCode {
    fn from(from: GotoResult) -> Self {
        let code = from.extract_authorization_code();
        match code {
            Ok(code) => {
                log::debug!("authorization code: {}", &code);
                return Self {
                    code: Some(code),
                    login_type: Some(String::from("normal")),
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
