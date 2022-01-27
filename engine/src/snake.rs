//! Battlesnake entity module

// Std
use std::convert::TryFrom;

// Local
use super::api::Battlesnake;
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

	/// Retrieves a move to direction away from neck.
	///
	/// `get_default_move()` substracts neck postion from head position,
	/// then tries to convert it to `Direction`. `ErrorKind::FailedConversion`
	/// is invoked if the conversion is not successful.
	/// In case of missing neck, the default direction is returned.
	///
	/// # Example
	/// ```
	/// # use battlesnake_engine::snake::Snake;
	/// # use battlesnake_engine::point::{Direction, Point};
	/// // Basic
	/// let snake = Snake::new(100, vec![Point::new(3, 0), Point::new(2, 0), Point::new(1, 0)]);
	/// assert_eq!(snake.get_default_move().unwrap(), Direction::Right);
	///
	/// // Missing neck (and head)
	/// let snake = Snake::new(100, Vec::new());
	///	assert_eq!(snake.get_default_move().unwrap(), Direction::default());
	///
	/// ```
	pub fn get_default_move(&self) -> Result<Direction> {
		let neck = self.neck();
		let head = self.head();
		if neck.is_some() && head.is_some() {
			Direction::try_from(head.unwrap() - neck.unwrap())
		} else {
			Ok(Direction::default())
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

	/// Slides the snake forward and reduces health accordingly
	pub fn advance(&mut self, dir: &Direction, cost: Option<u8>) -> Result<()> {
		self.body.slide_front(dir);
		self.reduce_health(cost.unwrap_or(1))
	}

	/// Feed the snake refilling health to 100 and extending by duplicating tail
	pub fn feed(&mut self) {
		self.health = 100;
		self.body.extend_back(&Point::default());
	}

	/// Reduce health by cost.
	///
	/// `reduce_health()` takes a cost argument that is deducted
	/// from current `self.health`. `self.health` cannot go below
	/// zero, so the negative value is truncated at `0` and
	/// `Error::Engine(ErrorKind::OutOfHealth)` is returned in such case.
	///
	/// # Examples
	/// ```
	/// # use battlesnake_engine::snake::Snake;
	/// # use battlesnake_engine::point::{Direction, Point};
	/// let mut snake = Snake::new(100, vec![Point::new(3, 0), Point::new(2, 0), Point::new(1, 0)]);
	/// assert!(snake.reduce_health(50).is_ok());
	/// assert!(snake.reduce_health(100).is_err());
	///
	/// ```
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

	/// Check if snake health > 0
	pub fn has_health(&self) -> bool {
		self.health != 0
	}

	/// Checks if given point is a collision with the snake, returning index of the collision
	pub fn is_collision(&self, p: &Point) -> Option<usize> {
		self.body.nodes.iter().position(|x| x == p)
	}

	/// Length of the snake
	pub fn size(&self) -> usize {
		self.body.nodes.len()
	}
}

impl TryFrom<&Battlesnake> for Snake {
	type Error = Error;

	fn try_from(input: &Battlesnake) -> Result<Self> {
		if input.is_empty() {
			return Err(Error::Engine(ErrorKind::FailedSnakeConversion));
		}
		Ok(Snake {
			health: input.health(),
			body: Path::from(input.body_ref()),
		})
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

	#[test]
	fn extended_basics() {
		let mut snake = Snake {
			health: 50,
			body: Path::from(vec![
				Point::new(9, 3),
				Point::new(9, 4),
				Point::new(9, 5),
				Point::new(8, 5),
				Point::new(7, 5),
				Point::new(7, 6),
			]),
		};
		assert_eq!(snake.size(), 6);
		assert_eq!(snake.get_default_move().unwrap(), Direction::Down);
		assert_eq!(snake.neck(), Some(Point::new(9, 4)));
		assert_eq!(snake.head(), Some(Point::new(9, 3)));
		snake.advance(&Direction::Down, Option::None);
		assert_eq!(snake.size(), 6);
		assert_eq!(
			snake.body.nodes,
			vec![
				Point::new(9, 2),
				Point::new(9, 3),
				Point::new(9, 4),
				Point::new(9, 5),
				Point::new(8, 5),
				Point::new(7, 5)
			]
		);
		assert_eq!(snake.health, 49);
		assert_eq!(snake.has_health(), true);
		assert!(snake.reduce_health(9).is_ok());
		assert_eq!(snake.health, 40);
		assert!(snake.reduce_health(45).is_err());
		assert_eq!(snake.health, 0);
		assert!(!snake.has_health());
		snake.feed();
		assert_eq!(snake.health, 100);
		assert!(snake.has_health());
		assert_eq!(snake.size(), 7);
		assert_eq!(
			snake.body.nodes,
			vec![
				Point::new(9, 2),
				Point::new(9, 3),
				Point::new(9, 4),
				Point::new(9, 5),
				Point::new(8, 5),
				Point::new(7, 5),
				Point::new(7, 5)
			]
		);
		assert_eq!(snake.is_collision(&Point::new(9, 4)), Some(2));
		assert_eq!(snake.is_collision(&Point::new(7, 5)), Some(5));
		assert_eq!(snake.is_collision(&Point::new(7, 6)), Option::None);
	}
}
