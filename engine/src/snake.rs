use super::path::Path;
use super::point::{Direction, Point};

#[derive(Debug, Clone)]
pub struct Snake {
	pub health: u8,
	pub body: Path,
}

impl Snake {
	pub fn new(health: u8, body: Vec<Point>) -> Self {
		Self {
			health,
			body: Path::from(body),
		}
	}
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn basics() {
		let p = Point::new(1, 1);
		let snake = Snake::new(100, vec![p - p, p, p + p]);
		dbg!(snake);
	}
}
