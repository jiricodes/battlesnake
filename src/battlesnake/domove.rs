use serde::{Serialize, Deserialize};
use super::input::GameInfo;
use super::grid::GameGrid;
use super::grid::GridObject;

use std::fmt;

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
#[serde(rename_all(serialize = "lowercase", deserialize = "lowercase"))]
pub enum Movement {
    Right,
    Left,
    Up,
    Down,
}

impl fmt::Display for Movement {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let move_text = match self {
			Movement::Right => "right",
			Movement::Left => "left",
			Movement::Up => "up",
			Movement::Down => "down",
		};
        write!(f, "{}", move_text)
    }
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
		// Parse game information
		let gameinfo = GameInfo::new(&input);

		// Create grid and fill it with snake bodies
		let mut grid = GameGrid::new(gameinfo.get_board_dimensions());
		grid.set_snakes(gameinfo.get_snake_bodies());

		// Log my snakes id
		println!("Turn: {}", gameinfo.get_turn());
		println!("Snake ID: {}", gameinfo.get_my_id());
		// Get my snake's head
		let head = gameinfo.get_my_head();
		println!("Head at: {}", head);
		println!("{}", grid);


		// Make this smarter lol, too tired
		let mut movement = Movement::Right;
		let val = grid.get_value(&head.get_right());
		if val == GridObject::EMPTY || val == GridObject::FOOD { movement = Movement::Right }
		let val = grid.get_value(&head.get_left());
		if val == GridObject::EMPTY || val == GridObject::FOOD { movement = Movement::Left }
		let val = grid.get_value(&head.get_up());
		if val == GridObject::EMPTY || val == GridObject::FOOD { movement = Movement::Up }
		let val = grid.get_value(&head.get_down());
		if val == GridObject::EMPTY || val == GridObject::FOOD { movement = Movement::Down }
		// selects move that is either to empty or food cell
		Self {
			movement: movement,
			shout: None,
		}
	}

	pub fn as_option_string(input: &str) -> Option<String> {
		Some(serde_json::to_string(&Move::new(&input)).unwrap())
	}

	pub fn get_json_string(&self) -> String {
		serde_json::to_string(self).unwrap()
	}
}

impl fmt::Display for Move {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let shout = if self.shout.is_some() {
			self.shout.as_ref().unwrap()
		} else {
			""
		};
        write!(f, "{} | {}", self.movement, shout)
    }
}