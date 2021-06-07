//! Compute the shortest path from start position to end position using the [A* algorithm](https://en.wikipedia.org/wiki/A*_search_algorithm)
//! Designed for battlesnake
//! v0.0.1 
//! Eventually should consider snake length, smoothness and available space through heuristic perhaps

use std::collections::HashMap;
use std::collections::BinaryHeap;

use super::point::Point;
use super::heuristic::Heuristic;
use super::grid::GameGrid;

pub struct Astar {
	start: Point,
	end: Point,
	grid: &GameGrid,
	heur: Heuristic,
	solution: Option<Vec<Point>>,
}

impl Astar {
	fn new(start: Point, end: Point, grid: &GameGrid, heur: Heuristic) -> Self {
		Self {
			start,
			end,
			grid,
			heur,
			None
		}
	}
	pub fn solve(start: Point, end: Point, grid: &GameGrid, heur: Heuristic) -> Option<Vec<Point>> {
		None
	}
}