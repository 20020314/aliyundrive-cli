use chrono::FixedOffset;
use serde::{Deserialize, Deserializer, Serialize};

#[derive(Serialize, Debug, Clone)]
pub struct DateTime(String);

impl DateTime {
    pub fn new(st: String) -> Self {
        Self(st)
    }

    pub fn to_timestamp(&self) -> i64 {
        let time = chrono::NaiveDateTime::parse_from_str(self.0.as_str(), crate::drive::standard::TIME_FORMAT)
            .expect("Failed to format time");
        time.timestamp() - (8 * 3600)
    }

    pub fn to_string(&self) -> String {
        self.0.clone()
    }
}

impl<'a> Deserialize<'a> for DateTime {
    fn deserialize<D: Deserializer<'a>>(deserializer: D) -> Result<Self, D::Error> {
        let result = <&str>::deserialize(deserializer)?;

        let dt = chrono::DateTime::parse_from_rfc3339(result).map_err(serde::de::Error::custom)?;
        let format = dt
            .with_timezone(&FixedOffset::east(8 * 3600))
            .format(crate::drive::standard::TIME_FORMAT);
        Ok(DateTime::new(format.to_string()))
    }
}