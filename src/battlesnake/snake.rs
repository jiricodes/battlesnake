use serde::{Deserialize, Serialize};

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

pub struct Snake {

}
