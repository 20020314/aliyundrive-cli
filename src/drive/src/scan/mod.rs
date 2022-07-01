pub mod model;
pub mod qr;

use serde::{de, Deserialize, Deserializer, Serialize};
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum State {
    Confirmed,
    Expired,
    New,
}

impl FromStr for State {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use State::*;

        match s {
            "NEW" => Ok(New),
            "EXPIRED" => Ok(Expired),
            "CONFIRMED" => Ok(Confirmed),
            _ => Ok(Expired),
        }
    }
}

impl<'de> Deserialize<'de> for State {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        FromStr::from_str(&s).map_err(de::Error::custom)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum ClientType {
    Web,
    App,
}

impl ClientType {
    fn refresh_token_url(&self) -> &'static str {
        match self {
            ClientType::Web => "https://api.aliyundrive.com/token/refresh",
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
