use serde::{Serialize, Deserialize};
use super::input::GameInfo;
use super::grid::GameGrid;
use super::grid::GridObject;

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
		let gameinfo = GameInfo::new(&input);
		let mut grid = GameGrid::new(gameinfo.get_board_dimensions());
		grid.set_snakes(gameinfo.get_snake_bodies());
		let head = gameinfo.get_my_head();
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
}