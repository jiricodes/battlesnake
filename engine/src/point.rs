//! Module to handle Point
//!

// External
use serde::{Deserialize, Serialize};

// Std
use std::cmp::PartialEq;
use std::convert::From;
use std::fmt;
use std::ops::{Add, AddAssign, Sub, SubAssign};
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

	/// Squared Euclidean distance to other point
	pub fn distance_squared(&self, other: &Self) -> f32 {
		((other.x - self.x).pow(2) + (other.y - self.y).pow(2)) as f32
	}

	/// Euclidean distance to other point
	pub fn distance(&self, other: &Self) -> f32 {
		self.distance_squared(other).sqrt()
	}

	/// Manhattan distance to other point
	pub fn manhattan_distance(&self, other: &Self) -> usize {
		let point = *other - *self;
		(point.x.abs() + point.y.abs()) as usize
	}

	/// Getter for neighbouring Points
	pub fn get_neighbours(&self) -> [Point; 4] {
		[
			*self + Direction::Right,
			*self + Direction::Left,
			*self + Direction::Up,
			*self + Direction::Down,
		]
	}

	/// Get a closest (euclidean distance) point from a vector of Points
	pub fn find_closest_euclidean(&self, others: &Vec<Point>) -> Option<Self> {
		let mut min_d = std::f32::MAX;
		let mut ret = Option::None;
		for p in others {
			let d = self.distance_squared(p);
			if d < min_d {
				min_d = d;
				ret = Some(*p);
			}
		}
		ret
	}

	/// Get a closest (manhattan distance) point from a vector of Points
	pub fn find_closest_manhattan(&self, others: &Vec<Point>) -> Option<Self> {
		let mut min_d = std::usize::MAX;
		let mut ret = Option::None;
		for p in others {
			let d = self.manhattan_distance(p);
			if d < min_d {
				min_d = d;
				ret = Some(*p);
			}
		}
		ret
	}

	/// Tries to get direction to other point
	pub fn direction_to_other(&self, other: &Self) -> Result<Direction> {
		Direction::try_from(*other - *self)
	}
}

impl Default for Point {
	/// Creates a Point with coordinates (0, 0)
	fn default() -> Self {
		Self { x: 0, y: 0 }
	}
}

impl fmt::Display for Point {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{{{}, {}}}", self.x, self.y)
	}
}

impl Sub<&Point> for Point {
	type Output = Self;

	fn sub(self, other: &Self) -> Self::Output {
		Self {
			x: self.x - other.x,
			y: self.y - other.y,
		}
	}
}

impl<T: Into<Point>> Sub<T> for Point {
	type Output = Point;

	fn sub(self, other: T) -> Self::Output {
		let p: Point = other.into();
		Self::Output {
			x: self.x - p.x,
			y: self.y - p.y,
		}
	}
}

impl Add<&Point> for Point {
	type Output = Self;

	fn add(self, other: &Self) -> Self::Output {
		Self::Output {
			x: self.x + other.x,
			y: self.y + other.y,
		}
	}
}

impl<T: Into<Point>> Add<T> for Point {
	type Output = Self;

	fn add(self, other: T) -> Self::Output {
		let p: Point = other.into();
		Self::Output {
			x: self.x + p.x,
			y: self.y + p.y,
		}
	}
}

impl From<Direction> for Point {
	fn from(dir: Direction) -> Self {
		match dir {
			Direction::Right => Point { x: 1, y: 0 },
			Direction::Left => Point { x: -1, y: 0 },
			Direction::Up => Point { x: 0, y: 1 },
			Direction::Down => Point { x: 0, y: -1 },
		}
	}
}

impl From<&Direction> for Point {
	fn from(dir: &Direction) -> Self {
		match *dir {
			Direction::Right => Point { x: 1, y: 0 },
			Direction::Left => Point { x: -1, y: 0 },
			Direction::Up => Point { x: 0, y: 1 },
			Direction::Down => Point { x: 0, y: -1 },
		}
	}
}

impl<T: Into<Point>> AddAssign<T> for Point {
	fn add_assign(&mut self, other: T) {
		let p: Point = other.into();
		*self = *self + p;
	}
}

impl AddAssign<&Point> for Point {
	fn add_assign(&mut self, other: &Self) {
		*self = *self + other;
	}
}

impl<T: Into<Point>> SubAssign<T> for Point {
	fn sub_assign(&mut self, other: T) {
		let p: Point = other.into();
		*self = *self - p;
	}
}

impl SubAssign<&Point> for Point {
	fn sub_assign(&mut self, other: &Self) {
		*self = *self - other;
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

impl TryFrom<&Point> for Direction {
	type Error = Error;
	fn try_from(point: &Point) -> Result<Self> {
		match point {
			Point { x: 1, y: 0 } => Ok(Direction::Right),
			Point { x: -1, y: 0 } => Ok(Direction::Left),
			Point { x: 0, y: 1 } => Ok(Direction::Up),
			Point { x: 0, y: -1 } => Ok(Direction::Down),
			_ => Err(Error::Engine(ErrorKind::FailedConversion)),
		}
	}
}

impl Default for Direction {
	/// According to Battlesnake docs, default direction is `UP`.
	fn default() -> Self {
		Direction::Up
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
		assert_eq!(
			Direction::try_from(&Point { x: 1, y: 0 }).unwrap(),
			Direction::Right
		);
		assert_eq!(
			Direction::try_from(&Point { x: -1, y: 0 }).unwrap(),
			Direction::Left
		);
		assert_eq!(
			Direction::try_from(&Point { x: 0, y: 1 }).unwrap(),
			Direction::Up
		);
		assert_eq!(
			Direction::try_from(&Point { x: 0, y: -1 }).unwrap(),
			Direction::Down
		);
		assert!(Direction::try_from(&Point { x: -1, y: -1 }).is_err());
		assert!(Direction::try_from(&Point { x: -31, y: 91 }).is_err());
		assert!(Direction::try_from(&Point { x: 10, y: 11 }).is_err());
		assert!(Direction::try_from(&Point { x: 5, y: -1 }).is_err());
	}

	#[test]
	fn point_ops() {
		let a = Point { x: 5, y: 5 };
		let b = Point { x: 1, y: 2 };
		assert_eq!(a + b, Point { x: 6, y: 7 });
		assert_eq!(a + &b, Point { x: 6, y: 7 });
		assert_eq!(a - b, Point { x: 4, y: 3 });
		assert_eq!(a - &b, Point { x: 4, y: 3 });
		assert_eq!(a + Direction::Left, Point { x: 4, y: 5 });
		assert_eq!(a - Direction::Left, Point { x: 6, y: 5 });
		let mut c = Point::default();
		c += a;
		assert_eq!(c, a);
		c += &a;
		assert_eq!(c, Point { x: 10, y: 10 });
		c -= b;
		assert_eq!(c, Point { x: 9, y: 8 });
		c -= &b;
		assert_eq!(c, Point { x: 8, y: 6 });
		c += Direction::Up;
		assert_eq!(c, Point { x: 8, y: 7 });
		c -= &Direction::Up;
		assert_eq!(c, Point { x: 8, y: 6 });
	}

	#[test]
	fn point_distances() {
		let a = Point { x: 5, y: 5 };
		for d in Direction::iterator() {
			let b = a + d;
			assert_eq!(a.distance(&b), 1.0);
			assert_eq!(a.manhattan_distance(&b), 1);
		}
		let b = Point { x: 7, y: 3 };
		assert_eq!(a.distance_squared(&b), 8.0);
		assert_eq!(a.manhattan_distance(&b), 4);
		let b = Point { x: 7, y: 1 };
		assert_eq!(a.distance(&b), 4.472135955);
		assert_eq!(a.manhattan_distance(&b), 6);
	}

	#[test]
	fn neighbours() {
		let a = Point { x: 5, y: 5 };
		let expected = [
			Point { x: 6, y: 5 },
			Point { x: 4, y: 5 },
			Point { x: 5, y: 6 },
			Point { x: 5, y: 4 },
		];
		assert_eq!(a.get_neighbours(), expected);
	}
}
