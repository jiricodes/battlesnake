///! Grid implementation for Wrapped game mode
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum GridObject {
	Empty,
	Food,
	Snake(usize),
	Hazard,
	Collisionchance(i32),
	Outofbounds,
}

impl GridObject {
	pub fn is_snake(&self) -> bool {
		match self {
			GridObject::Snake(_) => true,
			_ => false,
		}
	}

	pub fn is_considerable(&self) -> bool {
		match self {
			GridObject::Snake(_) => false,
			GridObject::Collisionchance(_) => false,
			GridObject::Outofbounds => false,
			_ => true,
		}
	}

	pub fn is_accessible(&self) -> bool {
		match self {
			GridObject::Snake(_) => false,
			GridObject::Outofbounds => false,
			_ => true,
		}
	}
}

impl fmt::Display for GridObject {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let symbol = match self {
			GridObject::Empty => "◦",
			GridObject::Food => "⚕",
			GridObject::Snake(n) => match n {
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
			GridObject::Hazard => "H",
			GridObject::Outofbounds => "X",
			GridObject::Collisionchance(_) => "!",
		};
		write!(f, "{}", symbol)
	}
}

pub struct Grid {
	height: u8,
	width: u8,
	data: Vec<GridObject>,
}
