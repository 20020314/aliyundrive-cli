use crate::scan::model::AuthorizationToken;
use crate::scan::ClientType;
use crate::DateTime;
use anyhow::{anyhow, Context};
use chrono::format::Fixed::TimezoneOffset;
use chrono::FixedOffset;
use dirs;
use lazy_static::lazy_static;
use serde::de::Unexpected::Str;
use serde::ser;
use serde::{Deserialize, Serialize};
use std::io::Read;
use std::ops::Deref;
use std::path::PathBuf;
use std::time::SystemTime;
use tokio::io::AsyncReadExt;

lazy_static! {
    static ref APP_CONFIG_PATH: Option<&'static PathBuf> = {
        let p = init_conf("app_credentials");
        let x = Box::new(p);
        Some(Box::leak(x))
    };
    static ref WEB_CONFIG_PATH: Option<&'static PathBuf> = {
        let p = init_conf("web_credentials");
        let x = Box::new(p);
        Some(Box::leak(x))
    };
}

fn init_conf(path: &str) -> PathBuf {
    let cache_dir = dirs::cache_dir().expect("Get directory error");
    let workspace_dir = cache_dir.join("aliyundrive-cli");
    if !workspace_dir.exists() {
        std::fs::create_dir(&workspace_dir).unwrap();
        log::debug!(
            "Initialize aliyundrive-cli directory: {}",
            workspace_dir.display()
        )
    }

    // app config file not exists
    let conf_path = workspace_dir.join(path);
    if !conf_path.exists() {
        let res = std::fs::File::create(&conf_path).unwrap();
        log::debug!("Initialize config file: {}", conf_path.display());
    }
    conf_path
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Credentials {
    pub(crate) user_id: Option<String>,
    pub(crate) nick_name: Option<String>,
    pub(crate) access_token: Option<String>,
    pub(crate) refresh_token: Option<String>,
    pub(crate) client_type: ClientType,
    pub(crate) expire_time: String,
    pub(crate) timestamp: i64,
}

impl Credentials {
    pub fn new(client_type: ClientType) -> Self {
        let mut authorization = Credentials::default();
        authorization.client_type = client_type;
        authorization
    }

    pub fn read_refresh_token(&self) -> Option<String> {
        if !self.is_expired() {
            return self.refresh_token.as_ref().cloned();
        }
        None
    }

    pub fn read_access_token(&self) -> Option<String> {
        if !self.is_expired() {
            return self.access_token.as_ref().cloned();
        }
        None
    }

    fn is_expired(&self) -> bool {
        let end_time = DateTime::new(self.expire_time.clone());
        let end_timestamp = end_time.to_timestamp();
        let start_timestamp = chrono::prelude::Utc::now().timestamp();
        (end_timestamp - start_timestamp < 0)
    }
}

impl AuthorizationToken for Credentials {
    fn access_token(&self) -> Option<String> {
        self.access_token.as_ref().cloned()
    }

    fn refresh_token(&self) -> Option<String> {
        self.refresh_token.as_ref().cloned()
    }
}

pub struct AppConf;

impl AppConf {
    pub async fn print_std() -> anyhow::Result<()> {
        let p = APP_CONFIG_PATH.expect("Initialize aliyundrive directory error");
        let str = tokio::fs::read_to_string(p)
            .await
            .context("Read configuration error!")?;
        print!("{}", str);
        Ok(())
    }

    pub async fn write(t: Credentials) -> anyhow::Result<()> {
        let p = APP_CONFIG_PATH.expect("Initialize app_credentials file error");
        let str = serde_yaml::to_string(&t).context("Serialized write configuration failed")?;
        tokio::fs::write(p, str).await?;
        Ok(())
    }

    pub async fn read() -> anyhow::Result<Credentials> {
        let p = APP_CONFIG_PATH.expect("Initialize app_credentials file error");
        let str = tokio::fs::read_to_string(p)
            .await
            .context("Read configuration error!")?;
        serde_yaml::from_str(str.as_str()).context("Serialized read configuration failed")
    }
}
