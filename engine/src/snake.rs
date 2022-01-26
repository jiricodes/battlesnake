//! Battlesnake entity module
use super::path::Path;
use super::point::{Direction, Point};
use super::utils::status::{Error, ErrorKind, Result};

/// Struct representing a snake
#[derive(Debug, Clone)]
pub struct Snake {
	health: u8,
	body: Path,
}

impl Snake {
	/// Constructor
	pub fn new(health: u8, body: Vec<Point>) -> Self {
		Self {
			health,
			body: Path::from(body),
		}
	}

	/// Retrieves head position (first node)
	pub fn head(&self) -> Option<Point> {
		self.body.first()
	}

	/// Retrieves second node aka neck
	fn neck(&self) -> Option<Point> {
		self.body.get_node(1)
	}

	pub fn advance(&mut self, dir: &Direction, cost: Option<u8>) -> Result<()> {
		self.body.slide_front(dir);
		self.reduce_health(cost.unwrap_or(1))
	}

	pub fn feed(&mut self) {
		self.health = 100;
		self.body.extend_back(&Point::default());
	}

	pub fn reduce_health(&mut self, cost: u8) -> Result<()> {
		match cost < self.health {
			true => {
				self.health -= cost;
				Ok(())
			}
			false => {
				self.health = 0;
				Err(Error::Engine(ErrorKind::OutOfHealth))
			}
		}
	}
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn basics() {
		let mut snake = Snake::new(
			100,
			vec![Point::new(0, 0), Point::new(1, 0), Point::new(2, 0)],
		);
		assert_eq!(snake.head().unwrap(), Point::new(0, 0));
		assert_eq!(snake.neck().unwrap(), Point::new(1, 0));
		assert!(snake.advance(&Direction::Up, Option::None).is_ok());
		assert_eq!(snake.head().unwrap(), Point::new(0, 1));
		assert_eq!(snake.neck().unwrap(), Point::new(0, 0));
		assert_eq!(snake.health, 99);
		assert!(snake.advance(&Direction::Up, Some(101)).is_err());
		assert_eq!(snake.head().unwrap(), Point::new(0, 2));
		assert_eq!(snake.neck().unwrap(), Point::new(0, 1));
		assert_eq!(snake.health, 0);
	}
}
