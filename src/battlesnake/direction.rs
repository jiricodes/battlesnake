//! General 4-way direction enum for grid usage
//!
use std::convert::TryFrom;
use std::fmt;

use serde::{Deserialize, Serialize};

use super::Point;

pub const ALL_DIRECTIONS: [Direction; 4] = [
    Direction::Right,
    Direction::Left,
    Direction::Up,
    Direction::Down,
];

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
#[serde(rename_all(serialize = "lowercase", deserialize = "lowercase"))]
pub enum Direction {
    Right,
    Left,
    Up,
    Down,
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
    type Error = &'static str;

    fn try_from(point: Point) -> Result<Self, Self::Error> {
        match point {
            Point { x: 1, y: 0 } => Ok(Direction::Right),
            Point { x: -1, y: 0 } => Ok(Direction::Left),
            Point { x: 0, y: 1 } => Ok(Direction::Up),
            Point { x: 0, y: -1 } => Ok(Direction::Down),
            _ => Err("Point is not valid Direction"),
        }
    }
}
