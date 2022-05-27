use crate::models::suc::TokenLoginResult;
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

impl From<TokenLoginResult> for AuthorizationCode {
    fn from(from: TokenLoginResult) -> Self {
        let code = from.get_authorization_code();
        if let Ok(code) = code {
            log::debug!("authorization code: {}", &code);
            return Self {
                code: Some(code),
                login_type: Some(String::from("")),
            };
        }
        Self {
            code: None,
            login_type: None,
        }
    }
}
