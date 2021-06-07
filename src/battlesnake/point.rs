use serde::{Deserialize, Serialize};
use std::fmt;
use std::ops::{Add, Sub};

use super::domove::Movement;

#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq, Hash, Eq)]
pub struct Point {
    x: i32,
    y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
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
            y: self.y,
        }
    }

    pub fn get_left(&self) -> Point {
        Point {
            x: self.x - 1,
            y: self.y,
        }
    }

    pub fn get_up(&self) -> Point {
        Point {
            x: self.x,
            y: self.y + 1,
        }
    }

    pub fn get_down(&self) -> Point {
        Point {
            x: self.x,
            y: self.y - 1,
        }
    }

    pub fn get_neighbours(&self) -> Vec<Point> {
        vec![
            self.get_right(),
            self.get_left(),
            self.get_up(),
            self.get_down(),
        ]
    }

    pub fn distance_squared(&self, other: &Self) -> f32 {
        ((other.get_x() - self.x).pow(2) + (other.get_y() - self.y).pow(2)) as f32
    }
    pub fn distance(&self, other: &Self) -> f32 {
        self.distance_squared(other).sqrt()
    }

	pub fn manhattan_distance(&self, other: &Self) -> i32 {
		let point = *other - *self;
		point.x.abs() + point.y.abs()
	}

    pub fn find_closest(&self, others: Vec<Self>) -> Self {
        let mut dist = f32::MAX;
        let mut best = &Point::new(-1, -1);
        for point in others.iter() {
            let current = self.distance_squared(point);
            if dist > current {
                best = point;
                dist = current;
            }
        }
        best.clone()
    }

    pub fn get_neighbour_direction(self, neighbour: Self) -> Option<Movement> {
        let p = neighbour - self;
        match p {
            Point { x: 1, y: 0 } => Some(Movement::Right),
            Point { x: -1, y: 0 } => Some(Movement::Left),
            Point { x: 0, y: 1 } => Some(Movement::Up),
            Point { x: 0, y: -1 } => Some(Movement::Down),
            _ => None,
        }
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{{}, {}}}", self.x, self.y)
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn closest() {
        let point = Point::new(0, 0);
        let other_points = vec![
            Point::new(-1, -10),
            Point::new(-3, 3),
            Point::new(-10, -10),
            Point::new(-1, -1),
        ];
        assert!(Point::new(-1, -1) == point.find_closest(other_points));
    }

    #[test]
    fn directions() {
        let point = Point::new(10, 10);
        let right = Point { x: 11, y: 10 };
        let left = Point { x: 9, y: 10 };
        let up = Point { x: 10, y: 11 };
        let down = Point { x: 10, y: 9 };
        let bad = Point { x: 15, y: 15 };
        assert!(point.get_neighbour_direction(right).unwrap() == Movement::Right);
        assert!(point.get_neighbour_direction(left).unwrap() == Movement::Left);
        assert!(point.get_neighbour_direction(up).unwrap() == Movement::Up);
        assert!(point.get_neighbour_direction(down).unwrap() == Movement::Down);
        assert!(point.get_neighbour_direction(bad) == None);
    }
}
