pub mod model;
pub mod qr;

use serde::{de, Deserialize, Deserializer};
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
