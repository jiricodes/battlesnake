use super::point::Point;
use std::fmt;

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

pub struct GameGrid {
    height: usize,
    width: usize,
    data: Vec<GridObject>,
    ignore_hazard: bool,
}

impl GameGrid {
    pub fn new(dimensions: (usize, usize)) -> Self {
        Self {
            height: dimensions.0,
            width: dimensions.1,
            data: vec![GridObject::Empty; dimensions.0 * dimensions.1],
            ignore_hazard: false,
        }
    }

    pub fn reset(&mut self) {
        self.data = vec![GridObject::Empty; self.height * self.width];
    }

    fn get_index(&self, pos: &Point) -> Option<usize> {
        if !self.is_in_bounds(&pos) {
            None
        } else {
            Some(pos.get_y() as usize * self.width + pos.get_x() as usize)
        }
    }

    pub fn get_value(&self, pos: &Point) -> GridObject {
        match self.get_index(&pos) {
            Some(i) => self.data[i],
            None => GridObject::Outofbounds,
        }
    }

    pub fn get_value_at_index(&self, i: usize) -> GridObject {
        if i < self.width * self.height {
            self.data[i]
        } else {
            GridObject::Outofbounds
        }
    }

    pub fn is_accessible(&self, pos: &Point) -> bool {
        let val = self.get_value(pos);
        val == GridObject::Empty
            || val == GridObject::Food
            || (val == GridObject::Hazard && self.ignore_hazard)
    }

    fn is_in_bounds(&self, pos: &Point) -> bool {
        0 <= pos.get_x()
            && pos.get_x() < self.width as i32
            && 0 <= pos.get_y()
            && pos.get_y() < self.height as i32
    }

    // invalid points are ignored
    pub fn set_snakes(&mut self, snakes: Vec<Vec<Point>>) {
        for (p, snake) in snakes.iter().enumerate() {
            for point in snake.iter() {
                match self.get_index(&point) {
                    Some(i) => {
                        self.data[i] = GridObject::Snake(p);
                    }
                    None => {
                        continue;
                    }
                }
            }
        }
    }

    pub fn set_food(&mut self, food: &Vec<Point>) {
        for p in food {
            match self.get_index(&p) {
                Some(i) => {
                    self.data[i] = GridObject::Food;
                }
                None => {
                    continue;
                }
            }
        }
    }

    pub fn set_hazards(&mut self, hazard: &Vec<Point>) {
        for p in hazard {
            match self.get_index(&p) {
                Some(i) => {
                    if !self.data[i].is_snake() {
                        self.data[i] = GridObject::Hazard;
                    }
                }
                None => {
                    continue;
                }
            }
        }
    }

    pub fn set_collision_chance(&mut self, collision: &Vec<Point>, hp: i32) {
        for p in collision {
            match self.get_index(&p) {
                Some(i) => {
                    if !self.data[i].is_snake() {
                        self.data[i] = GridObject::Collisionchance(hp);
                    }
                }
                None => {
                    continue;
                }
            }
        }
    }

    pub fn ignore_hazard(&mut self) {
        self.ignore_hazard = true;
    }

    pub fn get_width(&self) -> usize {
        self.width
    }

    pub fn get_height(&self) -> usize {
        self.height
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
        assert_eq!(grid.get_value(&Point::new(5, 5)), GridObject::Empty);
        assert_eq!(grid.get_value(&Point::new(-1, 5)), GridObject::Outofbounds);
    }
}
