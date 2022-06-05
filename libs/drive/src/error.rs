use std::fmt;
use std::fmt::{Debug, Display, Formatter};

pub struct DriveError {
    kind: String,
    message: String,
}

impl std::error::Error for DriveError {}

impl Display for DriveError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Debug for DriveError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
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

pub struct QrCodeScannerError {
    message: String,
}

impl Debug for QrCodeScannerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("QrCodeScannerError")
            .field("message", &self.message)
            .finish()
    }
}

impl Display for QrCodeScannerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Message: {}", self.message)
    }
}

impl From<&str> for QrCodeScannerError {
    fn from(msg: &str) -> Self {
        Self {
            message: String::from(msg),
        }
    }
}

impl From<String> for QrCodeScannerError {
    fn from(msg: String) -> Self {
        Self { message: msg }
    }
}

impl From<reqwest::Error> for QrCodeScannerError {
    fn from(e: reqwest::Error) -> Self {
        Self {
            message: e.to_string(),
        }
    }
}

impl From<anyhow::Error> for QrCodeScannerError {
    fn from(e: anyhow::Error) -> Self {
        Self {
            message: e.to_string(),
        }
    }
}

impl std::error::Error for QrCodeScannerError {}
