//! Heuristic module to support modularity of A* search algo

use super::grid::{GameGrid, GridObject};
use super::point::Point;

#[derive(Clone, Copy, Debug)]
pub enum HeurMethod {
    Manhattan,
    Euclidean,
    Battlesnake,
}

pub struct Heuristic {
    method: HeurMethod,
    pub get_func: fn(&Heuristic, &Point, &Point) -> f32,
    width: usize,
    height: usize,
    data: Vec<f32>,
}

impl Heuristic {
    pub fn new(heur_method: HeurMethod) -> Self {
        Self {
            method: heur_method,
            get_func: match heur_method {
                HeurMethod::Manhattan => Self::manhattan,
                HeurMethod::Euclidean => Self::euclidean,
                HeurMethod::Battlesnake => Self::battlesnake,
            },
            width: 0,
            height: 0,
            data: Vec::new(),
        }
    }
    fn manhattan(&self, start: &Point, end: &Point) -> f32 {
        start.manhattan_distance(end) as f32
    }

    fn euclidean(&self, start: &Point, end: &Point) -> f32 {
        start.distance(end)
    }

    fn is_in_bounds(&self, p: &Point) -> bool {
        let x = p.get_x();
        let y = p.get_y();
        x > 0 && x < self.width as i32 && y > 0 && y < self.height as i32
    }

    pub fn battlesnake_init(
        &mut self,
        width: usize,
        height: usize,
        hazards: &Vec<Point>,
        collision: &Vec<Point>,
        health: i32,
        target: &Point,
    ) {
        self.data.clear();
        self.width = width;
        self.height = height;
        let x = target.get_x();
        let y = target.get_y();
        for i in 0..(self.height * self.width) {
            let v: i32 = (x - (i % self.width) as i32).abs() + (y - (i / self.width) as i32).abs();
            self.data.push(v as f32);
        }
        for p in hazards {
            if self.is_in_bounds(p) {
                let i = (p.get_y() as usize * self.width) + p.get_x() as usize;
                self.data[i] += 15.0;
            }
        }
        for p in collision {
            if self.is_in_bounds(p) {
                let i = (p.get_y() as usize * self.width) + p.get_x() as usize;
                self.data[i] += health as f32;
            }
        }
        // dbg!(&self.data);
    }

    fn battlesnake(&self, start: &Point, end: &Point) -> f32 {
        let i = (start.get_y() as usize * self.width) + start.get_x() as usize;
        self.data[i]
    }

    pub fn get_value(&self, start: &Point, end: &Point) -> f32 {
        (self.get_func)(self, start, end)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn manhattan_00() {
        let heur = Heuristic::new(HeurMethod::Manhattan);
        let h: usize = 5;
        let w: usize = 5;
        let start = Point::new(0, 0);
        let expected: Vec<i32> = vec![
            0, 1, 2, 3, 4, 1, 2, 3, 4, 5, 2, 3, 4, 5, 6, 3, 4, 5, 6, 7, 4, 5, 6, 7, 8,
        ];
        for (i, val) in expected.iter().enumerate() {
            let end = Point::new((i % w) as i32, (i / h) as i32);
            assert!(expected[i] as f32 == heur.get_value(&start, &end));
        }
    }
    #[test]
    fn manhattan_inner() {
        let heur = Heuristic::new(HeurMethod::Manhattan);
        let h: usize = 10;
        let w: usize = 10;
        let start = Point::new(7, 3);
        let expected: Vec<i32> = vec![
            10, 9, 8, 7, 6, 5, 4, 3, 4, 5, 9, 8, 7, 6, 5, 4, 3, 2, 3, 4, 8, 7, 6, 5, 4, 3, 2, 1, 2,
            3, 7, 6, 5, 4, 3, 2, 1, 0, 1, 2, 8, 7, 6, 5, 4, 3, 2, 1, 2, 3, 9, 8, 7, 6, 5, 4, 3, 2,
            3, 4, 10, 9, 8, 7, 6, 5, 4, 3, 4, 5, 11, 10, 9, 8, 7, 6, 5, 4, 5, 6, 12, 11, 10, 9, 8,
            7, 6, 5, 6, 7, 13, 12, 11, 10, 9, 8, 7, 6, 7, 8,
        ];
        for (i, val) in expected.iter().enumerate() {
            let end = Point::new((i % w) as i32, (i / h) as i32);
            assert!(expected[i] as f32 == heur.get_value(&start, &end));
        }
    }
    #[test]
    fn manhattan_outter() {
        let heur = Heuristic::new(HeurMethod::Manhattan);
        let h: usize = 10;
        let w: usize = 10;
        let start = Point::new(-13, 19);
        let expected: Vec<i32> = vec![
            32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 30, 31,
            32, 33, 34, 35, 36, 37, 38, 39, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 28, 29, 30, 31,
            32, 33, 34, 35, 36, 37, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 26, 27, 28, 29, 30, 31,
            32, 33, 34, 35, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 24, 25, 26, 27, 28, 29, 30, 31,
            32, 33, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32,
        ];
        for (i, val) in expected.iter().enumerate() {
            let end = Point::new((i % w) as i32, (i / h) as i32);
            assert!(expected[i] as f32 == heur.get_value(&start, &end));
        }
    }
}
