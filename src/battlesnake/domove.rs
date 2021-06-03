use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
#[serde(rename_all(serialize = "lowercase", deserialize = "lowercase"))]
pub enum Movement {
    Right,
    Left,
    Up,
    Down,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Move {
	#[serde(rename = "move")]
	movement: Movement,
	#[serde(skip_serializing_if = "Option::is_none")]
	shout: Option<String>,
}

impl Move {
	pub fn new(input: &str) -> Self {
		Self {
			movement: Movement::Right,
			shout: None,
		}
	}

	pub fn as_option_string(input: &str) -> Option<String> {
		Some(serde_json::to_string(&Move::new(&input)).unwrap())
	}
}