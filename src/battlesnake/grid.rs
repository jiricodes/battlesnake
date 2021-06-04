use super::point::Point;
use std::fmt;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum GridObject {
	EMPTY,
	FOOD,
	SNAKE(usize),
	HAZARD,
	OUTOFBOUND
}

impl fmt::Display for GridObject {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let symbol = match self {
			GridObject::EMPTY => "◦",
			GridObject::FOOD => "⚕",
			GridObject::SNAKE(n) => match n {
				0 => "■",
				1 => "⌀",
				2 => "●",
				3 => "⍟",
				4 => "◘",
				5 => "☺",
				6 => "□",
				7 => "☻",
				_ => "S",
			},
			GridObject::HAZARD => "H",
			GridObject::OUTOFBOUND => "X",
		};
		write!(f, "{}", symbol)
	}
}

pub struct GameGrid {
	height: usize,
	width: usize,
	data: Vec<GridObject>,
}

impl GameGrid {
	pub fn new(dimensions: (usize, usize)) -> Self {
		Self {
			height: dimensions.0,
			width: dimensions.1,
			data: vec![GridObject::EMPTY; dimensions.0 * dimensions.1],
		}
	}

	pub fn reset(&mut self) {
		self.data = vec![GridObject::EMPTY; self.height * self.width];
	}

	fn get_index(&self, pos: &Point) -> Option<usize> {
		if !self.is_in_bounds(&pos)	{
			None
		}
		else {
			Some(pos.get_y() as usize * self.width + pos.get_x() as usize)
		}
	}

	pub fn get_value(&self, pos: &Point) -> GridObject {
		match self.get_index(&pos) {
			Some(i) => self.data[i],
			None => GridObject::OUTOFBOUND
		}
	}

	fn is_in_bounds(&self, pos: &Point) -> bool {
		0 <= pos.get_x() && pos.get_x() < self.width as i32 && 0 <= pos.get_y() && pos.get_y() < self.height as i32
	}

	// invalid points are ignored
	pub fn set_snakes(&mut self, snakes: Vec<Vec<Point>>) {
		for (p, snake) in snakes.iter().enumerate() {
			for point in snake.iter() {
				match self.get_index(&point) {
					Some(i) => { self.data[i] = GridObject::SNAKE(p); },
					None => { continue; }
				}
			}
		}
	}
}

impl fmt::Display for GameGrid {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let mut grid_text = String::new();
		let mut line = String::new();
		for (i, cell) in self.data.iter().rev().enumerate() {
			line.insert_str(0, &format!("{}", cell));
			if (i + 1) % self.width == 0 {
				line.push_str("\n");
				grid_text.push_str(&line);
				line.clear();
			}
		}
		write!(f, "{}", grid_text)
	}
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn grid_is_in_bounds() {
		let grid = GameGrid::new((10, 10));
		assert_eq!(grid.is_in_bounds(&Point::new(5, 5)), true);
		assert_eq!(grid.is_in_bounds(&Point::new(0, 0)), true);
		assert_eq!(grid.is_in_bounds(&Point::new(9, 9)), true);
		assert_eq!(grid.is_in_bounds(&Point::new(-1, 5)), false);
		assert_eq!(grid.is_in_bounds(&Point::new(1, -5)), false);
		assert_eq!(grid.is_in_bounds(&Point::new(10, 5)), false);
		assert_eq!(grid.is_in_bounds(&Point::new(1, 15)), false);
	}

	#[test]
	fn grid_get_value() {
		let grid = GameGrid::new((10, 10));
		assert_eq!(grid.get_value(&Point::new(5, 5)), GridObject::EMPTY);
		assert_eq!(grid.get_value(&Point::new(-1, 5)), GridObject::OUTOFBOUND);
	}
}