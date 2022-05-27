use anyhow::anyhow;
use reqwest::Url;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct GotoResult {
    #[serde(default)]
    goto: Option<String>,
}

impl From<String> for GotoResult {
    fn from(token: String) -> Self {
        Self { goto: Some(token) }
    }
}

impl GotoResult {
    pub fn extract_authorization_code(&self) -> crate::Result<String> {
        let goto = self.goto.as_ref().ok_or(anyhow!("goto value is None"))?;
        let url = Url::parse(goto.as_str())?;
        if let Some(query) = url.query() {
            let query = query.to_string();
            let param_array: Vec<_> = query.split("&").collect();
            for param in param_array {
                let param = param.to_string();
                let k_v_array: Vec<_> = param.split("=").collect();
                if let Some(key) = k_v_array.get(0) {
                    if *key == "code" {
                        if let Some(value) = k_v_array.get(1) {
                            return Ok(String::from(*value));
                        }
                    }
                }
            }
        }
        Err(anyhow!("get goto result error."))
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct MobileLoginResult {
    pds_login_result: Option<PdsLoginResult>,
}

impl MobileLoginResult {
    pub fn new() -> Self {
        Self {
            pds_login_result: None,
        }
    }

    pub fn get_access_token(&self) -> Option<String> {
        let pds_login_result = self.pds_login_result.as_ref()?;
        let access_token = pds_login_result.access_token.as_ref()?;
        Some(access_token.to_string())
    }

    pub fn get_refresh_token(&self) -> Option<String> {
        let pds_login_result = self.pds_login_result.as_ref()?;
        let refresh_token = pds_login_result.refresh_token.as_ref()?;
        Some(refresh_token.to_string())
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
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

    #[serde(rename = "DingDingRobotUrl")]
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

    #[serde(rename = "DingDingRobotUrl")]
    #[serde(default)]
    status: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
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
