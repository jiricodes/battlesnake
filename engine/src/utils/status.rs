//! Error utility module

use std::cmp::{PartialEq, PartialOrd};
use std::fmt;
use std::io;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ErrorKind {
	FailedConversion,
	FailedSnakeConversion,
	OutOfHealth,
}

impl ErrorKind {
	fn as_str(&self) -> &str {
		match *self {
			ErrorKind::FailedConversion => "Conversion Attempt failed",
			ErrorKind::FailedSnakeConversion => {
				"Conversion from api::battlesnake::Battlesnake to Snake failed."
			}
			ErrorKind::OutOfHealth => "Snake is out of health with current move",
		}
	}

	fn error_name(&self) -> &str {
		match *self {
			ErrorKind::FailedConversion | ErrorKind::FailedSnakeConversion => "FailedConversion",
			ErrorKind::OutOfHealth => "SnakeDeath",
		}
	}
}

impl fmt::Debug for ErrorKind {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.debug_struct(self.error_name())
			.field("message", &self.as_str())
			.finish()
	}
}

#[derive(Debug)]
pub enum Error {
	Io(io::Error),
	Serde(serde_json::Error),
	Engine(ErrorKind),
	Custom(String),
}

impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match *self {
			Error::Io(ref err) => err.fmt(f),
			Error::Serde(ref err) => err.fmt(f),
			Error::Engine(ref err) => write!(f, "Engine Error: {:?}", err),
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
