#![allow(dead_code)]

use crate::models::AuthorizationToken;
use anyhow::anyhow;
use reqwest::Url;
use serde::{Deserialize};

#[derive(Deserialize, Debug)]
pub struct GotoResult {
    #[serde(default)]
    goto: Option<String>,
}

impl<'a> From<&'a String> for GotoResult {
    fn from(token: &String) -> Self {
        Self {
            goto: Some(token.to_string()),
        }
    }
}

impl GotoResult {
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
            if *key == crate::models::CODE_KEY {
                let value = k_v_array
                    .get(1)
                    .ok_or(anyhow!("goto query param value is None"))?;
                return Ok(String::from(*value));
            }
        }
        Err(anyhow!("Failed to get authorization code"))
    }
}

#[derive(Deserialize, Debug)]
pub struct MobileLoginResult {
    #[serde(default)]
    pds_login_result: Option<PdsLoginResult>,
}

impl AuthorizationToken for MobileLoginResult {
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

#[derive(Deserialize, Debug)]
pub struct PdsLoginResult {
    #[serde(rename = "role")]
    #[serde(default)]
    role: Option<String>,

    #[serde(rename = "userData")]
    #[serde(default)]
    user_data: Option<UserData>,

    #[serde(rename = "isFirstLogin")]
    #[serde(default)]
    is_first_login: bool,

    #[serde(rename = "needLink")]
    #[serde(default)]
    need_link: bool,

    #[serde(rename = "loginType")]
    #[serde(default)]
    login_type: Option<String>,

    #[serde(rename = "nickName")]
    #[serde(default)]
    nick_name: Option<String>,

    #[serde(rename = "needRpVerify")]
    #[serde(default)]
    need_rp_verify: bool,

    #[serde(rename = "avatar")]
    #[serde(default)]
    avatar: Option<String>,

    #[serde(rename = "accessToken")]
    #[serde(default)]
    access_token: Option<String>,

    #[serde(rename = "userName")]
    #[serde(default)]
    user_name: Option<String>,

    #[serde(rename = "userId")]
    #[serde(default)]
    user_id: Option<String>,

    #[serde(rename = "defaultDriveId")]
    #[serde(default)]
    default_drive_id: Option<String>,

    #[serde(rename = "expiresIn")]
    #[serde(default)]
    expires_in: i64,

    #[serde(rename = "expireTime")]
    #[serde(default)]
    expire_time: Option<String>,

    #[serde(rename = "requestId")]
    #[serde(default)]
    request_id: Option<String>,

    #[serde(rename = "dataPinSetup")]
    #[serde(default)]
    data_pin_setup: bool,

    #[serde(rename = "state")]
    #[serde(default)]
    state: Option<String>,

    #[serde(rename = "tokenType")]
    #[serde(default)]
    token_type: Option<String>,

    #[serde(rename = "dataPinSaved")]
    #[serde(default)]
    data_pin_saved: bool,

    #[serde(rename = "refreshToken")]
    #[serde(default)]
    refresh_token: Option<String>,

    #[serde(rename = "status")]
    #[serde(default)]
    status: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct UserData {
    #[serde(rename = "DingDingRobotUrl")]
    #[serde(default)]
    ding_ding_robot_url: Option<String>,

    #[serde(rename = "FeedBackSwitch")]
    #[serde(default)]
    feed_back_switch: bool,

    #[serde(rename = "FollowingDesc")]
    #[serde(default)]
    following_desc: Option<String>,
}

#[derive(Deserialize, Debug, Default)]
pub struct WebLoginResult {
    #[serde(default)]
    default_sbox_drive_id: Option<String>,

    #[serde(default)]
    role: Option<String>,

    #[serde(default)]
    user_name: Option<String>,

    #[serde(default)]
    need_link: bool,

    #[serde(default)]
    expire_time: Option<String>,

    #[serde(default)]
    pin_setup: bool,

    #[serde(default)]
    need_rp_verify: bool,

    #[serde(default)]
    avatar: Option<String>,

    #[serde(default)]
    user_data: Option<UserData>,

    #[serde(default)]
    token_type: Option<String>,

    #[serde(default)]
    access_token: Option<String>,

    #[serde(default)]
    default_drive_id: Option<String>,

    #[serde(default)]
    domain_id: Option<String>,

    #[serde(default)]
    refresh_token: Option<String>,

    #[serde(default)]
    is_first_login: bool,

    #[serde(default)]
    user_id: Option<String>,

    #[serde(default)]
    nick_name: Option<String>,

    #[serde(default)]
    state: Option<String>,

    #[serde(default)]
    expires_in: i64,

    #[serde(default)]
    status: Option<String>,
}

impl AuthorizationToken for WebLoginResult {
    fn access_token(&self) -> Option<String> {
        let access_token = self.access_token.as_ref()?;
        Some(access_token.to_string())
    }

    fn refresh_token(&self) -> Option<String> {
        let refresh_token = self.refresh_token.as_ref()?;
        Some(refresh_token.to_string())
    }
}
