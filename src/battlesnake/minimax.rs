use log::*;
use std::cmp::{max, min, Eq, Ord, Ordering, PartialEq, PartialOrd};
use std::collections::BinaryHeap;
use std::sync::Mutex;
use std::time::{Duration, SystemTime};

use rayon::prelude::*;

use super::board::{Board, CauseOfDeath};
use super::domove::Move;
use super::input::GameInfo;
use super::Direction;

struct State {
    board: Board,
    root: Option<Direction>,
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

// Copied from Bookworm
pub fn cartesian_product<T: Copy>(lists: &[Vec<T>]) -> Vec<Vec<T>> {
    lists.iter().fold(vec![vec![]], |product, list| {
        list.iter()
            .flat_map(|item| {
                product
                    .iter()
                    .map(|prev_tuple| {
                        let mut new_tuple = prev_tuple.clone();
                        new_tuple.push(*item);
                        new_tuple
                    })
                    .collect::<Vec<Vec<T>>>()
            })
            .collect::<Vec<Vec<T>>>()
    })
}

//assume no NaN
#[inline]
fn min_f32(a: f32, b: f32) -> f32 {
    match a.partial_cmp(&b).unwrap() {
        Ordering::Less => a,
        _ => b,
    }
}

pub fn heuristic(board: &Board, snake_index: usize) -> f32 {
    0.0
}

pub fn get_move(gameinfo: &GameInfo, time_budget: Duration) -> Move {
    let time_start = SystemTime::now();
    let initial_board = Board::from_api(gameinfo);
    let hazards = gameinfo.get_hazards();
    let mut cnt_explored: usize = 0;
    let mut decision = initial_board.snakes[0].get_default_move();

    let mut queue: BinaryHeap<State> = BinaryHeap::new();
    queue.push(State {
        board: initial_board,
        root: None,
        depth: 0,
        h: 1.0,
    });
    'minimax: while let Some(first) = queue.pop() {
        if let Some(dir) = first.root {
            decision = dir;
        }

        if SystemTime::now().duration_since(time_start).unwrap() >= time_budget {
            info!("Time Budget Ran Out. Explored {}", cnt_explored);
            break 'minimax;
        }

        let all_snakes_moves = first.board.get_all_moves();

        // prune possible moves

        let worst_outcomes: Mutex<[Option<State>; 4]> = Mutex::new([None, None, None, None]);

        let possible_moves = cartesian_product(&all_snakes_moves);
        cnt_explored += possible_moves.len();

        possible_moves.par_iter().for_each(|moves| {
            let mut new_board = first.board.clone();
            let dead_snakes = new_board.advance_snakes(moves, &hazards);
            let my_move = *moves.get(0).unwrap();
            let my_move_index = my_move.as_index();

            if let Some(&cod) = dead_snakes.get(&0) {
                worst_outcomes.lock().unwrap()[my_move_index] = Some(State {
                    board: new_board,
                    root: Some(first.root.unwrap_or(my_move)),
                    depth: first.depth + 1,
                    h: match cod {
                        CauseOfDeath::HeadToHead => -1000.0,
                        CauseOfDeath::OutOfHealth => -1100.0,
                        _ => -3000.0,
                    },
                });
            } else {
                let next_h_score = heuristic(&new_board, 0);
                let is_new_worst = worst_outcomes
                    .lock()
                    .unwrap()
                    .get(my_move_index)
                    .unwrap()
                    .as_ref()
                    .map(|worst_outcome| next_h_score < worst_outcome.h)
                    .unwrap_or(true);

                if is_new_worst {
                    worst_outcomes.lock().unwrap()[my_move_index] = Some(State {
                        board: new_board,
                        root: Some(first.root.unwrap_or(my_move)),
                        depth: first.depth + 1,
                        h: min_f32(next_h_score, first.h),
                    });
                }
            };
        });

        for worst_outcome in worst_outcomes.lock().unwrap().iter_mut() {
            if let Some(state) = worst_outcome.take() {
                if state.h >= 0.0 {
                    // if log_enabled!(Debug) && frontier_board.depth == 1 {
                    //     debug!("Depth 1 option: dir={:?} score={}\n{}", frontier_board.root_dir, frontier_board.h_score, draw_board(&frontier_board.board));
                    // }
                    queue.push(state);
                }
            }
        }
    }

    Move {
        movement: decision,
        shout: None,
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn simple() {
        unimplemented!();
    }
}
