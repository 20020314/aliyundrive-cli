use std::fmt;
use std::str::FromStr;
use serde::{Deserialize, Deserializer, Serialize};
use std::time::SystemTime;

#[derive(Debug, Clone, Deserialize)]
pub struct CloudFile {
    pub name: String,

    #[serde(rename = "file_id")]
    pub id: String,

    pub r#type: FileType,

    pub created_at: DateTime,

    pub updated_at: DateTime,

    #[serde(default)]
    pub size: u64,

    pub url: Option<String>,
}

#[derive(Debug, Clone, Copy, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum FileType {
    Folder,
    File,
}

#[derive(Debug, Clone)]
pub struct DateTime(SystemTime);

impl DateTime {
    pub fn new(st: SystemTime) -> Self {
        Self(st)
    }
}

impl<'a> Deserialize<'a> for DateTime {
    fn deserialize<D: Deserializer<'a>>(deserializer: D) -> Result<Self, D::Error> {
        let result = <&str>::deserialize(deserializer)?;
        let dt = chrono::DateTime::parse_from_rfc3339(result).map_err(serde::de::Error::custom)?;
        Ok(Self(dt.into()))
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct ListFileRequest<'a> {
    pub drive_id: &'a str,
    pub parent_file_id: &'a str,
    pub limit: u64,
    pub all: bool,
    pub image_thumbnail_process: &'a str,
    pub image_url_process: &'a str,
    pub video_thumbnail_process: &'a str,
    pub fields: &'a str,
    pub order_by: &'a str,
    pub order_direction: &'a str,
    pub marker: Option<&'a str>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ListFileResponse {
    pub items: Vec<ListFileItem>,
    pub next_marker: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ListFileItem {
    pub name: String,

    pub category: Option<String>,
    #[serde(rename = "file_id")]
    pub id: String,

    pub r#type: FileType,

    pub created_at: DateTime,

    pub updated_at: DateTime,
    #[serde(default)]
    pub size: u64,

    pub url: Option<String>,
}

#[derive(Debug, Clone, Copy)]
pub enum ClientType {
    Web,
    App,
}

impl ClientType {
    fn refresh_token_url(&self) -> &'static str {
        match self {
            ClientType::Web => "https://websv.aliyundrive.com/token/refresh",
            ClientType::App => "https://auth.aliyundrive.com/v2/account/token",
        }
    }
}

impl FromStr for ClientType {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "web" | "" => Ok(ClientType::Web),
            "app" => Ok(ClientType::App),
            _ => anyhow::bail!("invalid client type '{}'", s),
        }
    }
}

impl fmt::Display for ClientType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ClientType::Web => write!(f, "web"),
            ClientType::App => write!(f, "app"),
        }
    }
}
