use serde::{Serialize, Deserialize};
use std::fmt;

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub struct Point {
	x: i32,
	y: i32,
}

impl Point {
	pub fn new(x: i32, y:i32) -> Self {
		Self {
			x,
			y,
		}
	}

	pub fn get_x(&self) -> i32 {
		self.x
	}

	pub fn get_y(&self) -> i32 {
		self.y
	}

	pub fn get_right(&self) -> Point {
		Point {
			x: self.x + 1,
			y: self.y
		}
	}

	pub fn get_left(&self) -> Point {
		Point {
			x: self.x - 1,
			y: self.y
		}
	}

	pub fn get_up(&self) -> Point {
		Point {
			x: self.x,
			y: self.y + 1
		}
	}

	pub fn get_down(&self) -> Point {
		Point {
			x: self.x,
			y: self.y - 1
		}
	}

}

impl fmt::Display for Point {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{{}, {}}}", self.x, self.y)
    }
}