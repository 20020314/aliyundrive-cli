#![allow(dead_code)]

use crate::scan::model::AuthorizationToken;
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
    pds_login_result: Option<PdsLoginResult>,
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
    role: Option<String>,

    #[serde(rename = "userData")]
    user_data: Option<UserData>,

    #[serde(rename = "isFirstLogin")]
    is_first_login: bool,

    #[serde(rename = "needLink")]
    need_link: bool,

    #[serde(rename = "loginType")]
    login_type: Option<String>,

    #[serde(rename = "nickName")]
    nick_name: Option<String>,

    #[serde(rename = "needRpVerify")]
    need_rp_verify: bool,

    avatar: Option<String>,

    #[serde(rename = "accessToken")]
    access_token: Option<String>,

    #[serde(rename = "userName")]
    user_name: Option<String>,

    #[serde(rename = "userId")]
    user_id: Option<String>,

    #[serde(rename = "defaultDriveId")]
    default_drive_id: Option<String>,

    #[serde(rename = "expiresIn")]
    expires_in: i64,

    #[serde(rename = "expireTime")]
    expire_time: Option<String>,

    #[serde(rename = "requestId")]
    request_id: Option<String>,

    #[serde(rename = "dataPinSetup")]
    data_pin_setup: bool,

    state: Option<String>,

    #[serde(rename = "tokenType")]
    token_type: Option<String>,

    #[serde(rename = "dataPinSaved")]
    data_pin_saved: bool,

    #[serde(rename = "refreshToken")]
    refresh_token: Option<String>,

    status: Option<String>,
}

#[derive(Deserialize, Debug, Default)]
pub struct UserData {
    #[serde(rename = "DingDingRobotUrl")]
    ding_ding_robot_url: Option<String>,

    #[serde(rename = "FeedBackSwitch")]
    feed_back_switch: bool,

    #[serde(rename = "FollowingDesc")]
    following_desc: Option<String>,
}

#[derive(Deserialize, Debug, Default)]
pub struct WebLoginResponse {
    default_sbox_drive_id: Option<String>,

    role: Option<String>,

    user_name: Option<String>,

    need_link: bool,

    expire_time: Option<String>,

    pin_setup: bool,

    need_rp_verify: bool,

    avatar: Option<String>,

    user_data: Option<UserData>,

    token_type: Option<String>,

    access_token: Option<String>,

    default_drive_id: Option<String>,

    domain_id: Option<String>,

    refresh_token: Option<String>,

    is_first_login: bool,

    user_id: Option<String>,

    nick_name: Option<String>,

    state: Option<String>,

    // Expiration
    expires_in: i64,

    // login status
    status: Option<String>,
}

impl WebLoginResponse {
    pub fn get_drive_id(self) -> Option<String> {
        self.default_drive_id
    }
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