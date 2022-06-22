#![allow(dead_code)]

use crate::scan::model::AuthorizationToken;
use crate::DateTime;
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
            if *key == crate::scan::model::CODE_KEY {
                let value = k_v_array
                    .get(1)
                    .ok_or(anyhow!("goto query param value is None"))?;
                return Ok(String::from(*value));
            }
        }
        anyhow::bail!("Failed to get authorization code")
    }
}

#[derive(Deserialize, Debug)]
pub struct MobileLoginResponse {
    #[serde(default)]
    pub pds_login_result: Option<PdsLoginResult>,
}

impl AuthorizationToken for MobileLoginResponse {
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

#[derive(Deserialize, Debug, Default)]
pub struct PdsLoginResult {
    #[serde(default)]
    pub role: Option<String>,

    #[serde(default)]
    #[serde(rename = "userData")]
    pub user_data: Option<UserData>,

    #[serde(default)]
    #[serde(rename = "isFirstLogin")]
    pub is_first_login: bool,

    #[serde(default)]
    #[serde(rename = "needLink")]
    pub need_link: bool,

    #[serde(default)]
    #[serde(rename = "loginType")]
    pub login_type: Option<String>,

    #[serde(default)]
    #[serde(rename = "nickName")]
    pub nick_name: Option<String>,

    #[serde(default)]
    #[serde(rename = "needRpVerify")]
    pub need_rp_verify: bool,

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

    #[serde(default)]
    #[serde(rename = "expireTime")]
    pub expire_time: Option<DateTime>,

    #[serde(default)]
    #[serde(rename = "requestId")]
    pub request_id: Option<String>,

    #[serde(default)]
    #[serde(rename = "dataPinSetup")]
    pub data_pin_setup: bool,

    #[serde(default)]
    pub state: Option<String>,

    #[serde(default)]
    #[serde(rename = "tokenType")]
    pub token_type: Option<String>,

    #[serde(default)]
    #[serde(rename = "dataPinSaved")]
    pub data_pin_saved: bool,

    #[serde(default)]
    #[serde(rename = "refreshToken")]
    pub refresh_token: Option<String>,

    #[serde(default)]
    pub status: Option<String>,
}

#[derive(Deserialize, Debug, Default)]
pub struct UserData {
    #[serde(default)]
    #[serde(rename = "DingDingRobotUrl")]
    pub ding_ding_robot_url: Option<String>,

    #[serde(default)]
    #[serde(rename = "FeedBackSwitch")]
    pub feed_back_switch: bool,

    #[serde(default)]
    #[serde(rename = "FollowingDesc")]
    pub following_desc: Option<String>,
}

#[derive(Deserialize, Debug, Default)]
pub struct WebLoginResponse {
    #[serde(default)]
    pub default_sbox_drive_id: Option<String>,

    #[serde(default)]
    pub role: Option<String>,

    #[serde(default)]
    pub user_name: Option<String>,

    #[serde(default)]
    pub need_link: bool,

    #[serde(default)]
    pub expire_time: Option<DateTime>,

    #[serde(default)]
    pub pin_setup: bool,

    #[serde(default)]
    pub need_rp_verify: bool,

    #[serde(default)]
    pub avatar: Option<String>,

    #[serde(default)]
    pub user_data: Option<UserData>,

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
    pub is_first_login: bool,

    #[serde(default)]
    pub user_id: Option<String>,

    #[serde(default)]
    pub nick_name: Option<String>,

    #[serde(default)]
    pub state: Option<String>,

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

#[derive(Debug, Clone, Deserialize)]
pub struct RefreshTokenResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: u64,
    pub token_type: String,
    pub user_id: String,
    pub nick_name: String,
    pub default_drive_id: String,
}
