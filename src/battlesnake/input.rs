use serde::{Serialize, Deserialize};
use super::point::Point;

#[derive(Serialize, Deserialize, Debug)]
struct GameData {
	id: String,
	timeout: i32,
}

#[derive(Serialize, Deserialize, Debug)]
struct Board {
	height: i32,
	width: i32,
	food: Vec<Point>,
	hazards: Vec<Point>,
	snakes: Vec<Snake>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Snake {
	id: String,
	name: String,
	health: i32,
	body: Vec<Point>,
	latency: i32,
	head: Point,
	length: i32,
	shout: String,
	squad: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GameInfo {
	game: GameData,
	turn: i32,
	board: Board,
	you: Snake,
}

impl GameInfo {
	pub fn new(data: &str) -> Self {
		let new_object: GameInfo = serde_json::from_str(data).unwrap();
		return new_object;
	}

	pub fn get_board_dimensions(&self) -> (usize, usize) {
		(self.board.height as usize, self.board.width as usize)
	}

	pub fn get_my_head(&self) -> Point {
		self.you.head
	}

	pub fn get_snake_bodies(&self) -> Vec<Point> {
		let mut snakes_bodies = Vec::new();
		// add snakes
		for snake in self.board.snakes.iter() {
				let mut to_add = snake.body.to_vec();
				snakes_bodies.append(&mut to_add);
		}
		snakes_bodies
	}
}
