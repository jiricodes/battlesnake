//! Compute the shortest path from start position to end position using the [A* algorithm](https://en.wikipedia.org/wiki/A*_search_algorithm)
//! Designed for battlesnake
//! v0.0.1
//! Eventually should consider snake length, smoothness and available space through heuristic perhaps

// default
use std::cmp::{Ordering, Reverse};
use std::collections::BinaryHeap;
use std::collections::HashMap;

// custom
use super::Heuristic;
use super::Point;
use super::{GameGrid, GridObject};

#[derive(Eq, Clone, Debug)]
struct AstarNode {
    point: Point,
    parents: Vec<Point>,
    gscore: i32,
}

impl AstarNode {
    pub fn new(point: Point) -> Self {
        Self {
            point,
            parents: Vec::new(),
            gscore: 0,
        }
    }

    pub fn get_children(&self, grid: &GameGrid, heur: &Heuristic) -> Vec<(i32, Point)> {
        let mut result: Vec<(i32, Point)> = Vec::new();
        let neighbours = self.point.get_neighbours();
        for p in &neighbours {
            if grid.is_accessible(p) && !self.parents.contains(p) {
                result.push((heur.get_value(&self.point, p) as i32, *p));
            }
        }
        result
    }

    pub fn is_end(&self, end: &Point) -> bool {
        self.point == *end
    }

    // returns complete path, without starting node
    pub fn get_path(&self) -> Vec<Point> {
        let mut path: Vec<Point> = self.parents[1..].to_vec();
        path.push(self.point);
        path
    }

    pub fn extend_with_child(&self, child: &Point, step: i32) -> Self {
        let mut new = self.clone();
        new.parents.push(self.point);
        new.point = *child;
        new.gscore += step;
        // dbg!(&new);
        new
    }

    pub fn get_gscore(&self) -> i32 {
        self.gscore
    }
}

impl Ord for AstarNode {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.gscore == other.gscore {
            Ordering::Equal
        } else if self.gscore < other.gscore {
            Ordering::Greater
        } else {
            Ordering::Less
        }
    }
}

impl PartialOrd for AstarNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for AstarNode {
    fn eq(&self, other: &Self) -> bool {
        self.gscore == other.gscore
    }
}

#[derive(Debug)]
struct AstarQueue {
    heap: BinaryHeap<AstarNode>,
    min_paths: HashMap<Point, i32>,
}

impl AstarQueue {
    pub fn new() -> Self {
        Self {
            heap: BinaryHeap::new(),
            min_paths: HashMap::new(),
        }
    }

    pub fn enqueue(&mut self, node: AstarNode) {
        self.heap.push(node);
    }

    pub fn dequeue(&mut self) -> Option<AstarNode> {
        self.heap.pop()
    }

    pub fn len(&self) -> usize {
        self.heap.len()
    }

    pub fn point_already_reached(&mut self, point: &Point, fscore: i32) -> bool {
        match self.min_paths.get(point) {
            Some(val) => {
                if *val <= fscore {
                    return true;
                } else {
                    self.min_paths.insert(*point, fscore);
                    return false;
                }
            }
            None => {
                self.min_paths.insert(*point, fscore);
                return false;
            }
        }
    }
}

pub struct Astar {
    solution: Option<(f32, Vec<Point>)>,
    closest_empty: Option<(f32, Vec<Point>)>,
}

impl Astar {
    pub fn new() -> Self {
        Self {
            solution: None,
            closest_empty: None,
        }
    }

    pub fn solve(&mut self, start: Point, end: Point, grid: &GameGrid, heur: &Heuristic) -> bool {
        self.solution = None;
        let mut queue = AstarQueue::new();
        queue.enqueue(AstarNode::new(start));
        while queue.len() > 0 {
            // dbg!(&queue);
            let current = queue.dequeue().unwrap();
            if current.is_end(&end) {
                self.solution = Some((current.get_gscore() as f32, current.get_path()));
                return true;
            }
            let children = current.get_children(grid, &heur);
            for (h, child) in &children {
                let step = match grid.get_value(child) {
                    GridObject::Collisionchance(v) => v,
                    GridObject::Hazard => 15,
                    _ => 1,
                };
                if !queue.point_already_reached(child, current.get_gscore() + *h) {
                    queue.enqueue(current.extend_with_child(child, step));
                }
            }
        }
        false
    }

    pub fn get_path(&self) -> Vec<Point> {
        self.solution.as_ref().unwrap().1.clone()
    }

    pub fn get_cost(&self) -> f32 {
        self.solution.as_ref().unwrap().0
    }
}

#[cfg(test)]
mod test {
    use super::super::heuristic::HeurMethod;
    use super::*;

    #[test]
    fn basic_manhattan() {
        let start = Point::new(0, 0);
        let end = Point::new(9, 9);
        let grid = GameGrid::new((10, 10));
        let heur = Heuristic::new(HeurMethod::Manhattan);
        let mut astar = Astar::new();
        let ret = astar.solve(start, end, &grid, &heur);
        assert!(ret == true);
        let path = astar.get_path();
        dbg!(path);
    }

    #[test]
    fn basic_battlesnake() {
        let start = Point::new(0, 0);
        let end = Point::new(9, 9);
        let grid = GameGrid::new((10, 10));
        let mut heur = Heuristic::new(HeurMethod::Battlesnake);
        heur.battlesnake_init(
            10,
            10,
            &vec![Point::new(1, 1)],
            &vec![Point::new(5, 5)],
            100,
            &end,
        );
        let mut astar = Astar::new();
        let ret = astar.solve(start, end, &grid, &heur);
        assert!(ret == true);
        let cost = astar.get_cost();
        dbg!(cost);
        let path = astar.get_path();
        dbg!(path);
    }
}
