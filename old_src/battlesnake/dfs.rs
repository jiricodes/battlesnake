use super::Point;
use super::{GameGrid, GridObject};

pub struct Dfs {
    solution: Option<Vec<Point>>,
    cost: f32,
    length: usize,
}

impl Dfs {
    pub fn new() -> Self {
        Self {
            solution: None,
            cost: 0.0,
            length: 0,
        }
    }

    pub fn get_atleast_len(&mut self, start: &Point, grid: &GameGrid, atleast: i32) -> bool {
        let mut atleast = atleast;
        let ngbs = start.get_neighbours();
        for n in &ngbs {
            if grid.is_accessible(n) {
                if self.solution.is_some() && !self.solution.as_ref().unwrap().contains(n) {
                    self.solution.as_mut().unwrap().push(*n);
                } else if self.solution.is_none() {
                    self.solution = Some(vec![*n]);
                } else {
                    continue;
                }
                let step = match grid.get_value(n) {
                    GridObject::Empty => 1.0,
                    GridObject::Food => 1.0,
                    GridObject::Hazard => 15.0,
                    _ => 101.0,
                };
                if grid.get_value(n) == GridObject::Food {
                    atleast += 1;
                }
                if atleast - 1 <= 0 {
                    self.cost += step;
                    self.length = self.solution.as_ref().unwrap().len();
                    return true;
                } else {
                    if self.get_atleast_len(n, grid, atleast - 1) {
                        self.cost += step;
                        self.length = self.solution.as_ref().unwrap().len();
                        return true;
                    } else {
                        let _ = self.solution.as_mut().unwrap().pop();
                    }
                }
            }
        }
        false
    }

    pub fn get_path(&self) -> Vec<Point> {
        self.solution.as_ref().unwrap().clone()
    }

    pub fn get_length(&self) -> usize {
        self.length
    }

    pub fn get_cost(&self) -> f32 {
        self.cost
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn dfs_simple() {
        let grid = GameGrid::new((11, 11));
        let start = Point::new(0, 0);
        let atleast = 10;
        let mut dfs = Dfs::new();
        let ret = dfs.get_atleast_len(&start, &grid, atleast);
        assert!(ret == true);
        dbg!(dfs.get_path());
        dbg!(dfs.get_length());
        dbg!(dfs.get_cost());
    }

    #[test]
    fn no_way() {
        // {10 0} {10 1} {9 1} {8 1} {8 0} {7 0} {7 1} {7 2} {6 2} {6 3} {6 3}
        let mut grid = GameGrid::new((11, 11));
        grid.set_snakes(vec![vec![
            Point::new(10, 1),
            Point::new(9, 1),
            Point::new(8, 1),
            Point::new(8, 0),
            Point::new(7, 0),
            Point::new(7, 1),
            Point::new(7, 2),
            Point::new(6, 2),
            Point::new(6, 3),
            Point::new(6, 2),
        ]]);
        let start = Point::new(10, 0);
        let atleast = 10;
        let mut dfs = Dfs::new();
        let ret = dfs.get_atleast_len(&start, &grid, atleast);
        assert!(ret == false);
        dbg!(dfs.get_path());
        dbg!(dfs.get_length());
        dbg!(dfs.get_cost());
    }

    #[test]
    fn way_out() {
        let mut grid = GameGrid::new((11, 11));
        grid.set_snakes(vec![
            vec![
                Point::new(10, 9),
                Point::new(10, 8),
                Point::new(10, 7),
                Point::new(10, 6),
                Point::new(10, 5),
                Point::new(9, 5),
                Point::new(8, 5),
                Point::new(7, 5),
                Point::new(7, 6),
                Point::new(8, 6),
                Point::new(9, 6),
                Point::new(9, 6),
            ],
            vec![
                Point::new(8, 9),
                Point::new(8, 8),
                Point::new(7, 8),
                Point::new(6, 8),
                Point::new(6, 7),
                Point::new(6, 6),
                Point::new(5, 6),
                Point::new(5, 7),
                Point::new(4, 7),
                Point::new(3, 7),
                Point::new(2, 7),
            ],
        ]);
        let start = Point::new(9, 9);
        let atleast = 12;
        let mut dfs = Dfs::new();
        let ret = dfs.get_atleast_len(&start, &grid, atleast);
        assert!(ret == true);
        let start = Point::new(10, 10);
        let atleast = 12;
        let mut dfs = Dfs::new();
        let ret = dfs.get_atleast_len(&start, &grid, atleast);
        assert!(ret == true);
        let start = Point::new(8, 10);
        let atleast = 12;
        let mut dfs = Dfs::new();
        let ret = dfs.get_atleast_len(&start, &grid, atleast);
        assert!(ret == true);
        let start = Point::new(7, 9);
        let atleast = 12;
        let mut dfs = Dfs::new();
        let ret = dfs.get_atleast_len(&start, &grid, atleast);
        assert!(ret == true);
    }
}
