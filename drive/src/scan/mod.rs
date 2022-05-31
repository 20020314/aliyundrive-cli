pub mod qr;

use crate::scan::State::{CONFIRMED, EXPIRED, NEW};
use serde::{de, Deserialize, Deserializer};
use std::str::FromStr;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum State {
    CONFIRMED,
    EXPIRED,
    NEW,
}

impl FromStr for State {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "NEW" => Ok(NEW),
            "EXPIRED" => Ok(EXPIRED),
            "CONFIRMED" => Ok(CONFIRMED),
            _ => Ok(EXPIRED),
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
