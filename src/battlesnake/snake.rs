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
            color: String::from("#EE82EE"),
            head: String::from("silly"),
            tail: String::from("hook"),
            version: String::from("0.0.0"),
        }
    }

    pub fn get_string(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}
