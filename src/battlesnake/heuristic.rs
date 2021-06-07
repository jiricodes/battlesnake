//! Heuristic module to support modularity of A* search algo

use super::point::Point;

pub struct Heuristic {
    height: usize,
    width: usize,
    start: Point,
    data: Vec<i32>,
}

impl Heuristic {
    pub fn manhattan(height: usize, width: usize, start: Point) -> Self {
        let mut data: Vec<i32> = Vec::with_capacity(height * width);
        for i in 0..height * width {
            let x = i % width;
            let y = i / height;
            let val = (start.get_x() - x as i32).abs() + (start.get_y() - y as i32).abs();
            data.push(val);
        }
        Self {
            height,
            width,
            start,
            data,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn manhattan_00() {
        let h: usize = 5;
        let w: usize = 5;
        let start = Point::new(0, 0);
        let result = Heuristic::manhattan(h, w, start);
        let expected: Vec<i32> = vec![
            0, 1, 2, 3, 4, 1, 2, 3, 4, 5, 2, 3, 4, 5, 6, 3, 4, 5, 6, 7, 4, 5, 6, 7, 8,
        ];
        assert!(result.data == expected);
    }
    #[test]
    fn manhattan_inner() {
        let h: usize = 10;
        let w: usize = 10;
        let start = Point::new(7, 3);
        let result = Heuristic::manhattan(h, w, start);
        let expected: Vec<i32> = vec![
            10, 9, 8, 7, 6, 5, 4, 3, 4, 5, 9, 8, 7, 6, 5, 4, 3, 2, 3, 4, 8, 7, 6, 5, 4, 3, 2, 1, 2,
            3, 7, 6, 5, 4, 3, 2, 1, 0, 1, 2, 8, 7, 6, 5, 4, 3, 2, 1, 2, 3, 9, 8, 7, 6, 5, 4, 3, 2,
            3, 4, 10, 9, 8, 7, 6, 5, 4, 3, 4, 5, 11, 10, 9, 8, 7, 6, 5, 4, 5, 6, 12, 11, 10, 9, 8,
            7, 6, 5, 6, 7, 13, 12, 11, 10, 9, 8, 7, 6, 7, 8,
        ];
        assert!(result.data == expected);
    }
    #[test]
    fn manhattan_outter() {
        let h: usize = 10;
        let w: usize = 10;
        let start = Point::new(-13, 19);
        let result = Heuristic::manhattan(h, w, start);
        let expected: Vec<i32> = vec![
            32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 30, 31,
            32, 33, 34, 35, 36, 37, 38, 39, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 28, 29, 30, 31,
            32, 33, 34, 35, 36, 37, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 26, 27, 28, 29, 30, 31,
            32, 33, 34, 35, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 24, 25, 26, 27, 28, 29, 30, 31,
            32, 33, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32,
        ];
        assert!(result.data == expected);
    }
}
