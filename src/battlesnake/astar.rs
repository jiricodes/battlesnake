//! Compute the shortest path from start position to end position using the [A* algorithm](https://en.wikipedia.org/wiki/A*_search_algorithm)
//! Designed for battlesnake
//! v0.0.1 
//! Eventually should consider snake length, smoothness and available space through heuristic perhaps

use std::collections::HashMap;
use std::collections::BinaryHeap;
use std::cmp::{Ordering, Reverse};

use super::point::Point;
use super::heuristic::Heuristic;
use super::grid::GameGrid;

pub struct Astar<'g> {
	start: Point,
	end: Point,
	grid: &'g GameGrid,
	heur: Heuristic,
	solution: Option<Vec<Point>>,
}

#[derive(Eq, Clone, Debug)]
struct AstarNode {
	point: Point,
	parents: Vec<Point>,
	cost: i32
}

impl AstarNode {
	pub fn new(point: Point) -> Self {
		Self {
			point,
			parents: Vec::new(),
			cost: 0
		}
	}

	pub fn get_children(&self, grid: &GameGrid, heur: &Heuristic) -> Vec<(i32, Point)> {
		let mut result: Vec<(i32, Point)> = Vec::new();
		let neighbours = self.point.get_neighbours();
		for p in &neighbours {
			if grid.is_accessible(p) && !self.parents.contains(p){
				result.push((heur.get_value(p), *p));
			}
		}
		result
	}

	pub fn is_end(&self, end: &Point) -> bool {
		self.point == *end
	}

	// returns complete path, without starting node
	pub fn get_path(&self) -> Vec<Point> {
		let mut path: Vec<Point> = self.parents.to_vec();
		path.push(self.point);
		path
	}
}

impl Ord for AstarNode {
    fn cmp(&self, other: &Self) -> Ordering {
		if self.cost == other.cost {
			Ordering::Equal
		}
		else if self.cost < other.cost {
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
        self.cost == other.cost
    }
}

struct AstarQueue {
	heap: BinaryHeap<AstarNode>,
}

impl AstarQueue {
	pub fn new() -> Self{
		Self {
			heap: BinaryHeap::new(),
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
}

impl<'g> Astar<'g> {
	fn new(start: Point, end: Point, grid: &'g GameGrid, heur: Heuristic) -> Self {
		Self {
			start,
			end,
			grid,
			heur,
			solution: None
		}
	}
	pub fn solve(start: Point, end: Point, grid: &'g GameGrid, heur: Heuristic) -> Option<Vec<Point>> {

		let mut queue = AstarQueue::new();
		queue.enqueue(AstarNode::new(start));
		while queue.len() > 0 {
			let current = queue.dequeue().unwrap();
			if current.is_end(&end) {
				return Some(current.get_path());
			}
			let children = current.get_children(grid, &heur);


		}
		None
	}
}