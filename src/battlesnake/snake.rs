use std::convert::TryFrom;

use serde::{Deserialize, Serialize};
use super::path::Path;
use super::input::ApiSnake;

#[derive(Serialize, Deserialize, Debug)]
pub struct SnakeProps {
    apiversion: String,
    author: String,
    color: String,
    head: String,
    tail: String,
    version: String,
}

impl SnakeProps {
    pub fn new() -> Self {
        Self {
            apiversion: String::from("1"),
            author: String::from("jiricodes"),
            color: String::from("#622BAA"),
            head: String::from("evil"),
            tail: String::from("rattle"),
            version: String::from("0.0.1"),
        }
    }

    pub fn get_string(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}

#[derive(Debug, Clone)]
pub struct Snake {
    pub health: u8,
    pub body: Path,
}

impl TryFrom<&ApiSnake> for Snake {
    type Error = &'static str;

    fn try_from(input: &ApiSnake) -> Result<Self, Self::Error> {
        if input.body.is_empty() {
            return Err("Empty ApiSnake Body")
        }
        let body = Path::from(&input.body);
        Ok(
            Snake {
                health: input.health as u8,
                body,
            }
        )
    }
}