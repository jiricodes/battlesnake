//! Movement API module
//!

// External
use serde::{Deserialize, Serialize};

// Std
use std::fmt;

// Local
use crate::point::Direction;
use crate::utils::status::{Error, Result};

/// Struct to handle movement responses
#[derive(Serialize, Deserialize, Debug)]
pub struct Movement {
    #[serde(rename = "move")]
    direction: Direction,
    #[serde(skip_serializing_if = "Option::is_none")]
    shout: Option<String>,
}

impl Movement {
    /// Constructor
    pub fn new(direction: Direction, shout: Option<String>) -> Self {
        Self { direction, shout }
    }

    pub fn json(&self) -> Result<String> {
        match serde_json::to_string(self) {
            Ok(val) => Ok(val),
            Err(e) => Err(Error::Serde(e)),
        }
    }
}

impl fmt::Display for Movement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let shout = match self.shout.as_ref() {
            Some(val) => val,
            None => "",
        };
        write!(f, "{} | {}", self.direction, shout)
    }
}
