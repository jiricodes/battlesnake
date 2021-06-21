use log::*;
use std::cmp::{max, min, Eq, Ord, Ordering, PartialEq, PartialOrd};
use std::collections::BinaryHeap;
use std::time::{Duration, SystemTime};

use super::board::Board;
use super::domove::Move;
use super::input::GameInfo;
use super::Direction;

struct State {
    board: Board,
    parent: Option<Direction>,
    depth: usize,
    h: f32,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.h.partial_cmp(&other.h)
    }
}

impl Eq for State {}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.h == other.h
    }
}

pub fn get_move(gameinfo: &GameInfo, time_budget: Duration) -> Move {
    let time_start = SystemTime::now();
    let initial_board = Board::from_api(gameinfo);
    let mut cnt_explored: usize = 0;
    let mut decision = initial_board.snakes[0].get_default_move();

    let mut queue: BinaryHeap<State> = BinaryHeap::new();
    queue.push(State {
        board: initial_board,
        parent: None,
        depth: 0,
        h: 1.0,
    });
    'minimax: while let Some(first) = queue.pop() {
        if let Some(dir) = first.parent {
            decision = dir;
        }

        if SystemTime::now().duration_since(time_start).unwrap() >= time_budget {
            info!("Time Budget Ran Out. Explored {}", cnt_explored);
            break 'minimax;
        }
    }

    Move {
        movement: decision,
        shout: None,
    }
}
