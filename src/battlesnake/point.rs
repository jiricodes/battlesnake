use serde::{Deserialize, Serialize};
use std::cmp::PartialEq;
use std::convert::From;
use std::fmt;
use std::ops::{Add, Sub};

use super::Direction;

#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq, Hash, Eq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn zero() -> Self {
        Self { x: 0, y: 0 }
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

    pub fn manhattan_distance(&self, other: &Self) -> usize {
        let point = *other - *self;
        (point.x.abs() + point.y.abs()) as usize
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

    // needs rework to use try_from for direction
    pub fn get_neighbour_direction(self, neighbour: Self) -> Option<Direction> {
        let p = neighbour - self;
        match p {
            Point { x: 1, y: 0 } => Some(Direction::Right),
            Point { x: -1, y: 0 } => Some(Direction::Left),
            Point { x: 0, y: 1 } => Some(Direction::Up),
            Point { x: 0, y: -1 } => Some(Direction::Down),
            _ => None,
        }
    }

    pub fn is_neighbour(self, other: Point) -> bool {
        self.manhattan_distance(&other) == 1
    }

    pub fn is_not_negative(&self) -> bool {
        self.x >= 0 && self.y >= 0
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

impl Sub<&Point> for Point {
    type Output = Self;

    fn sub(self, other: &Self) -> Self::Output {
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

impl Add<Direction> for Point {
    type Output = Self;

    fn add(self, dir: Direction) -> Self {
        self + Point::from(dir)
    }
}

impl Add<&Direction> for Point {
    type Output = Self;

    fn add(self, dir: &Direction) -> Self {
        self + Point::from(dir)
    }
}

impl Sub<Direction> for Point {
    type Output = Self;

    fn sub(self, dir: Direction) -> Self {
        self - Point::from(dir)
    }
}

impl Sub<&Direction> for Point {
    type Output = Self;

    fn sub(self, dir: &Direction) -> Self {
        self - Point::from(dir)
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
        assert_eq!(Point::new(-1, -1), point.find_closest(other_points));
    }

    #[test]
    fn directions() {
        let point = Point::new(10, 10);
        let right = Point { x: 11, y: 10 };
        let left = Point { x: 9, y: 10 };
        let up = Point { x: 10, y: 11 };
        let down = Point { x: 10, y: 9 };
        let bad = Point { x: 15, y: 15 };
        assert!(point.get_neighbour_direction(right).unwrap() == Direction::Right);
        assert!(point.get_neighbour_direction(left).unwrap() == Direction::Left);
        assert!(point.get_neighbour_direction(up).unwrap() == Direction::Up);
        assert!(point.get_neighbour_direction(down).unwrap() == Direction::Down);
        assert!(point.get_neighbour_direction(bad) == None);
        assert_eq!(point + Direction::Right, right);
        assert_eq!(point + Direction::Left, left);
        assert_eq!(point + Direction::Up, up);
        assert_eq!(point + Direction::Down, down);
        assert_eq!(point + &Direction::Right, right);
        assert_eq!(point + &Direction::Left, left);
        assert_eq!(point + &Direction::Up, up);
        assert_eq!(point + &Direction::Down, down);
    }
}
