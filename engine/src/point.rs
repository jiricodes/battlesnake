//! Module to handle Point
//!

// External
use serde::{Deserialize, Serialize};

// Std
use std::cmp::PartialEq;
use std::fmt;
use std::slice::Iter;

// Local
use super::utils::status::{Error, ErrorKind, Result};

/// Point Struct for 2D grids
#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq, Hash, Eq)]
pub struct Point {
	x: i16,
	y: i16,
}

impl Point {
	/// Constructor
	pub fn new(x: i16, y: i16) -> Self {
		Self { x, y }
	}
}

impl Default for Point {
	fn default() -> Self {
		Self { x: 0, y: 0 }
	}
}

impl fmt::Display for Point {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{{{}, {}}}", self.x, self.y)
	}
}

/// General 4-way direction enum for grid usage
#[derive(Deserialize, Serialize, Copy, Clone, PartialEq, Debug, Eq, Hash)]
#[serde(rename_all(serialize = "lowercase", deserialize = "lowercase"))]
pub enum Direction {
	Right,
	Left,
	Up,
	Down,
}

impl Direction {
	/// Returns iterator over all 4 directions.
	pub fn iterator() -> Iter<'static, Direction> {
		static ALL_DIRECTIONS: [Direction; 4] = [
			Direction::Right,
			Direction::Left,
			Direction::Up,
			Direction::Down,
		];
		ALL_DIRECTIONS.iter()
	}
}

impl fmt::Display for Direction {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let move_text = match self {
			Direction::Right => "right",
			Direction::Left => "left",
			Direction::Up => "up",
			Direction::Down => "down",
		};
		write!(f, "{}", move_text)
	}
}

impl TryFrom<Point> for Direction {
	type Error = Error;
	fn try_from(point: Point) -> Result<Self> {
		match point {
			Point { x: 1, y: 0 } => Ok(Direction::Right),
			Point { x: -1, y: 0 } => Ok(Direction::Left),
			Point { x: 0, y: 1 } => Ok(Direction::Up),
			Point { x: 0, y: -1 } => Ok(Direction::Down),
			_ => Err(Error::Engine(ErrorKind::FailedConversion)),
		}
	}
}

#[cfg(test)]
mod test {
	use super::*;
	#[test]
	fn all_directions() {
		let mut all_dirs = Direction::iterator();
		assert_eq!(all_dirs.next().unwrap(), &Direction::Right);
		assert_eq!(all_dirs.next().unwrap(), &Direction::Left);
		assert_eq!(all_dirs.next().unwrap(), &Direction::Up);
		assert_eq!(all_dirs.next().unwrap(), &Direction::Down);
		assert!(all_dirs.next().is_none());
	}

	#[test]
	fn dir_from_point() {
		assert_eq!(
			Direction::try_from(Point { x: 1, y: 0 }).unwrap(),
			Direction::Right
		);
		assert_eq!(
			Direction::try_from(Point { x: -1, y: 0 }).unwrap(),
			Direction::Left
		);
		assert_eq!(
			Direction::try_from(Point { x: 0, y: 1 }).unwrap(),
			Direction::Up
		);
		assert_eq!(
			Direction::try_from(Point { x: 0, y: -1 }).unwrap(),
			Direction::Down
		);
		assert!(Direction::try_from(Point { x: -1, y: -1 }).is_err());
		assert!(Direction::try_from(Point { x: -31, y: 91 }).is_err());
		assert!(Direction::try_from(Point { x: 10, y: 11 }).is_err());
		assert!(Direction::try_from(Point { x: 5, y: -1 }).is_err());
	}
}
