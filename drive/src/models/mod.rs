use std::fmt;

pub mod auth;
pub mod gen;
pub mod query;
pub mod suc;

pub trait Ok {
    fn ok(&self) -> bool;
}

pub struct DriveError {
    kind: String,
    message: String,
}

impl fmt::Display for DriveError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl fmt::Debug for DriveError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DriveError")
            .field("kind", &self.kind)
            .field("message", &self.message)
            .finish()
    }
}

impl From<Box<dyn std::error::Error>> for DriveError {
    fn from(err: Box<dyn std::error::Error>) -> Self {
        Self {
            kind: String::from("reqwest"),
            message: err.to_string(),
        }
    }
}
