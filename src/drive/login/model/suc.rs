#![allow(dead_code)]

use crate::drive::conf::Credentials;
use crate::drive::login::model::AuthorizationToken;
use crate::drive::login::ClientType;
use crate::drive::time::DateTime;
use anyhow::anyhow;
use reqwest::Url;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct GotoResponse {
    #[serde(default)]
    goto: Option<String>,
}

impl<'a> From<&'a String> for GotoResponse {
    fn from(token: &String) -> Self {
        Self {
            goto: Some(token.to_string()),
        }
    }
}

impl GotoResponse {
    pub fn extract_authorization_code(&self) -> anyhow::Result<String> {
        let goto = self.goto.as_ref().ok_or(anyhow!("goto value is None"))?;
        let url = Url::parse(goto.as_str())?;
        let query = url.query().ok_or(anyhow!("goto query is None"))?;
        let param_array = query.split("&").collect::<Vec<&str>>();
        for param in param_array {
            let param = param.to_string();
            let k_v_array = param.split("=").collect::<Vec<&str>>();
            let key = k_v_array
                .get(0)
                .ok_or(anyhow!("goto query param key is None"))?;
            if *key == crate::drive::login::model::CODE_KEY {
                let value = k_v_array
                    .get(1)
                    .ok_or(anyhow!("goto query param value is None"))?;
                return Ok(String::from(*value));
            }
        }
        anyhow::bail!("Failed to get authorization code")
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct AppLoginResponse {
    #[serde(default)]
    pub pds_login_result: Option<PdsLoginResult>,
}

impl AuthorizationToken for AppLoginResponse {
    fn access_token(&self) -> Option<String> {
        let pds_login_result = self.pds_login_result.as_ref()?;
        let access_token = pds_login_result.access_token.as_ref()?;
        Some(access_token.to_string())
    }

    fn refresh_token(&self) -> Option<String> {
        let pds_login_result = self.pds_login_result.as_ref()?;
        let refresh_token = pds_login_result.refresh_token.as_ref()?;
        Some(refresh_token.to_string())
    }
}

impl TryInto<Credentials> for AppLoginResponse {
    type Error = anyhow::Error;

    fn try_into(self) -> Result<Credentials, Self::Error> {
        let pds_login_result = self
            .pds_login_result
            .ok_or(anyhow!("failed to get pds login result"))?;
        Ok(Credentials {
            user_id: pds_login_result.user_id,
            nick_name: pds_login_result.nick_name,
            client_type: ClientType::App,
            access_token: pds_login_result.access_token,
            refresh_token: pds_login_result.refresh_token,
            expire_time: pds_login_result.expire_time.to_string(),
            timestamp: pds_login_result.expire_time.to_timestamp(),
        })
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct PdsLoginResult {
    #[serde(default)]
    #[serde(rename = "nickName")]
    pub nick_name: Option<String>,

    #[serde(default)]
    pub avatar: Option<String>,

    #[serde(default)]
    #[serde(rename = "accessToken")]
    pub access_token: Option<String>,

    #[serde(default)]
    #[serde(rename = "userName")]
    pub user_name: Option<String>,

    #[serde(default)]
    #[serde(rename = "userId")]
    pub user_id: Option<String>,

    #[serde(default)]
    #[serde(rename = "defaultDriveId")]
    pub default_drive_id: Option<String>,

    #[serde(default)]
    #[serde(rename = "expiresIn")]
    pub expires_in: i64,

    #[serde(rename = "expireTime")]
    pub expire_time: DateTime,

    #[serde(default)]
    #[serde(rename = "tokenType")]
    pub token_type: Option<String>,

    #[serde(default)]
    #[serde(rename = "refreshToken")]
    pub refresh_token: Option<String>,

    #[serde(default)]
    pub status: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct WebLoginResponse {
    #[serde(default)]
    pub default_sbox_drive_id: Option<String>,

    #[serde(default)]
    pub user_name: Option<String>,

    pub expire_time: DateTime,

    #[serde(default)]
    pub avatar: Option<String>,

    #[serde(default)]
    pub token_type: Option<String>,

    #[serde(default)]
    pub access_token: Option<String>,

    #[serde(default)]
    pub default_drive_id: Option<String>,

    #[serde(default)]
    pub domain_id: Option<String>,

    #[serde(default)]
    pub refresh_token: Option<String>,

    #[serde(default)]
    pub user_id: Option<String>,

    #[serde(default)]
    pub nick_name: Option<String>,

    #[serde(default)]
    pub expires_in: i64,

    #[serde(default)]
    pub status: Option<String>,
}

impl AuthorizationToken for WebLoginResponse {
    fn access_token(&self) -> Option<String> {
        self.access_token.as_ref().cloned()
    }

    fn refresh_token(&self) -> Option<String> {
        self.refresh_token.as_ref().cloned()
    }
}

impl TryInto<Credentials> for WebLoginResponse {
    type Error = anyhow::Error;

    fn try_into(self) -> Result<Credentials, Self::Error> {
        Ok(Credentials {
            user_id: self.user_id,
            nick_name: self.nick_name,
            client_type: ClientType::Web,
            access_token: self.access_token,
            refresh_token: self.refresh_token,
            expire_time: self.expire_time.to_string(),
            timestamp: self.expire_time.to_timestamp(),
        })
    }
}
