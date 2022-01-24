//! Error utility module

use std::fmt;
use std::io;

pub type Result<T> = std::result::Result<T, Error>;

pub enum ApiErrorKind {}

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    Serde(serde_json::Error),
    Custom(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Error::Io(ref err) => err.fmt(f),
            Error::Serde(ref err) => err.fmt(f),
            Error::Custom(ref err) => write!(f, "Custom Error: {:?}", err),
        }
    }
}

impl From<io::Error> for Error {
    fn from(f: io::Error) -> Self {
        Self::Io(f)
    }
}

impl From<serde_json::Error> for Error {
    fn from(f: serde_json::Error) -> Self {
        Self::Serde(f)
    }
}
