//! Heuristic module to support modularity of A* search algo

use super::point::Point;

#[derive(Clone, Copy, Debug)]
pub enum HeurMethod {
    Manhattan,
    Euclidean,
}

pub struct Heuristic {
    method: HeurMethod,
    pub get_func: fn(&Point, &Point) -> f32,
}

impl Heuristic {
    pub fn new(heur_method: HeurMethod) -> Self {
        Self {
            method: heur_method,
            get_func: match heur_method {
                HeurMethod::Manhattan => Self::manhattan,
                HeurMethod::Euclidean => Self::euclidean,
            },
        }
    }
    fn manhattan(start: &Point, end: &Point) -> f32 {
        start.manhattan_distance(end) as f32
    }

    fn euclidean(start: &Point, end: &Point) -> f32 {
        start.distance(end)
    }

    pub fn get_value(&self, start: &Point, end: &Point) -> f32 {
        (self.get_func)(start, end)
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
