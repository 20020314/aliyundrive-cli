use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Token {
    #[serde(rename = "token")]
    #[serde(default)]
    value: Option<String>,
}

impl From<String> for Token {
    fn from(token: String) -> Self {
        Self {
            value: Some(token)
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct AuthorizationCode {
    #[serde(rename = "token")]
    #[serde(default)]
    code: Option<String>,

    #[serde(rename = "loginType")]
    #[serde(default)]
    login_type: Option<String>,
}



