//! Module to handle Point
//!

// External
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Point {
    x: u32,
    y: u32,
}
