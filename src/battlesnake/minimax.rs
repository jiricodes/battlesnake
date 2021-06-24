use log::*;
use std::cmp::{Eq, Ord, Ordering, PartialEq, PartialOrd};
use std::collections::BinaryHeap;
use std::sync::Mutex;
use std::time::{Duration, SystemTime};

use rayon::prelude::*;

use super::{Board, CauseOfDeath};
use super::Move;
use super::GameInfo;
use super::Direction;
use super::Point;

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

#[inline]
fn max_f32(a: f32, b: f32) -> f32 {
    match a.partial_cmp(&b).unwrap() {
        Ordering::Greater => a,
        _ => b,
    }
}

pub fn heuristic(board: &Board, snake_index: usize, hazards: &Vec<Point>) -> f32 {
    // floodfill / area dominance
    // A* 1.0 - (cost / hp)
    // aggression

    let snake_len = board.snakes[snake_index].size();

    let mut ttl = 0;
    for snake in board.snakes.iter() {
        ttl += snake.size();
    }
    let avg_len = ttl as f32 / board.snakes.len() as f32;
    let len_rat = if snake_len >= avg_len { 1.0snake_len as f32 / avg_len;
    let mut aval: f32 = if board.food.is_empty() {1.0} else {0.1};
    for food in board.food.iter() {
        let res = board.astar(board.snakes[snake_index].head(), *food, hazards);
        if res.is_some() {
            let (g_score, _) = res.unwrap();
            println!("{} {} -> rat {} -> {}")
            aval = max_f32(aval, 1.0 - (g_score as f32 / board.snakes[snake_index].health as f32));
            }
    }

    let mut aggression: f32 = 0.0;
    for snake in board.snakes.iter() {
        if snake.size() < snake_len {
            if snake.head() == board.snakes[snake_index].head() {
                aggression = 1.1;
            } else {
                aggression = 1.0 / snake.head().manhattan_distance(&board.snakes[snake_index].head()) as f32;
            }
        }
    }
    //finally get the ratio. should implement here a weighted ratio
    aval * aggression
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
            info!(
                "Time Budget Ran Out. Explored {} Depth {}",
                cnt_explored, first.depth
            );
            // println!(
            //     "Time Budget Ran Out. Explored {} Depth {}",
            //     cnt_explored, first.depth
            // );
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
                        CauseOfDeath::HeadToHead => -1.0,
                        CauseOfDeath::OutOfHealth => -2.0,
                        _ => -3.0,
                    },
                });
            } else {
                let next_h_score = heuristic(&new_board, 0, hazards);
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
                // if state.depth == 2 {
                //     println!("Depth 1 option: dir={:?} score={}", state.root, state.h);
                // }
                if state.h >= 0.0 {
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
    use super::super::game_logger::GameStateLog;
    use super::*;

    #[test]
    fn test_test() {
        let data = GameInfo::new(
            r#"
        {
            "game": {
                "id": "66a99167-b263-4c9f-988e-087f5df286be",
                "ruleset": {
                    "name": "royale",
                    "version": ""
                },
                "timeout": 500
            },
            "turn": 85,
            "board": {
                "width": 11,
                "height": 11,
                "snakes": [
                    {
                        "id": "gs_bDM7cytwR9pKPcWFwFMY6BcW",
                        "name": "Go  Giddy",
                        "body": [
                            {
                                "x": 10,
                                "y": 5
                            },
                            {
                                "x": 9,
                                "y": 5
                            },
                            {
                                "x": 8,
                                "y": 5
                            },
                            {
                                "x": 7,
                                "y": 5
                            },
                            {
                                "x": 6,
                                "y": 5
                            },
                            {
                                "x": 6,
                                "y": 4
                            },
                            {
                                "x": 7,
                                "y": 4
                            },
                            {
                                "x": 8,
                                "y": 4
                            },
                            {
                                "x": 9,
                                "y": 4
                            }
                        ],
                        "head": {
                            "x": 10,
                            "y": 5
                        },
                        "length": 9,
                        "health": 88,
                        "shout": "",
                        "squad": ""
                    },
                    {
                        "id": "gs_MPSPTppCcQWCYCHy6TrvHhmK",
                        "name": "DDT",
                        "body": [
                            {
                                "x": 7,
                                "y": 0
                            },
                            {
                                "x": 6,
                                "y": 0
                            },
                            {
                                "x": 6,
                                "y": 1
                            },
                            {
                                "x": 5,
                                "y": 1
                            },
                            {
                                "x": 5,
                                "y": 0
                            },
                            {
                                "x": 4,
                                "y": 0
                            },
                            {
                                "x": 4,
                                "y": 1
                            },
                            {
                                "x": 4,
                                "y": 2
                            },
                            {
                                "x": 3,
                                "y": 2
                            },
                            {
                                "x": 2,
                                "y": 2
                            }
                        ],
                        "head": {
                            "x": 7,
                            "y": 0
                        },
                        "length": 10,
                        "health": 5,
                        "shout": "Out of my way!",
                        "squad": ""
                    },
                    {
                        "id": "gs_VVTvwpQRmtB8xKH6xxTKgxD4",
                        "name": "spaceworm",
                        "body": [
                            {
                                "x": 7,
                                "y": 6
                            },
                            {
                                "x": 6,
                                "y": 6
                            },
                            {
                                "x": 5,
                                "y": 6
                            },
                            {
                                "x": 5,
                                "y": 5
                            },
                            {
                                "x": 5,
                                "y": 4
                            },
                            {
                                "x": 4,
                                "y": 4
                            },
                            {
                                "x": 4,
                                "y": 3
                            },
                            {
                                "x": 3,
                                "y": 3
                            },
                            {
                                "x": 3,
                                "y": 4
                            },
                            {
                                "x": 3,
                                "y": 5
                            }
                        ],
                        "head": {
                            "x": 7,
                            "y": 6
                        },
                        "length": 10,
                        "health": 87,
                        "shout": "",
                        "squad": ""
                    }
                ],
                "food": [
                    {
                        "x": 2,
                        "y": 7
                    },
                    {
                        "x": 10,
                        "y": 7
                    },
                    {
                        "x": 8,
                        "y": 9
                    },
                    {
                        "x": 0,
                        "y": 0
                    }
                ],
                "hazards": [
                    {
                        "x": 0,
                        "y": 0
                    },
                    {
                        "x": 0,
                        "y": 1
                    },
                    {
                        "x": 0,
                        "y": 2
                    },
                    {
                        "x": 0,
                        "y": 3
                    },
                    {
                        "x": 0,
                        "y": 4
                    },
                    {
                        "x": 0,
                        "y": 5
                    },
                    {
                        "x": 0,
                        "y": 6
                    },
                    {
                        "x": 0,
                        "y": 7
                    },
                    {
                        "x": 0,
                        "y": 8
                    },
                    {
                        "x": 0,
                        "y": 9
                    },
                    {
                        "x": 0,
                        "y": 10
                    },
                    {
                        "x": 1,
                        "y": 0
                    },
                    {
                        "x": 1,
                        "y": 1
                    },
                    {
                        "x": 1,
                        "y": 10
                    },
                    {
                        "x": 2,
                        "y": 0
                    },
                    {
                        "x": 2,
                        "y": 1
                    },
                    {
                        "x": 2,
                        "y": 10
                    },
                    {
                        "x": 3,
                        "y": 0
                    },
                    {
                        "x": 3,
                        "y": 1
                    },
                    {
                        "x": 3,
                        "y": 10
                    },
                    {
                        "x": 4,
                        "y": 0
                    },
                    {
                        "x": 4,
                        "y": 1
                    },
                    {
                        "x": 4,
                        "y": 10
                    },
                    {
                        "x": 5,
                        "y": 0
                    },
                    {
                        "x": 5,
                        "y": 1
                    },
                    {
                        "x": 5,
                        "y": 10
                    },
                    {
                        "x": 6,
                        "y": 0
                    },
                    {
                        "x": 6,
                        "y": 1
                    },
                    {
                        "x": 6,
                        "y": 10
                    },
                    {
                        "x": 7,
                        "y": 0
                    },
                    {
                        "x": 7,
                        "y": 1
                    },
                    {
                        "x": 7,
                        "y": 10
                    },
                    {
                        "x": 8,
                        "y": 0
                    },
                    {
                        "x": 8,
                        "y": 1
                    },
                    {
                        "x": 8,
                        "y": 10
                    },
                    {
                        "x": 9,
                        "y": 0
                    },
                    {
                        "x": 9,
                        "y": 1
                    },
                    {
                        "x": 9,
                        "y": 10
                    },
                    {
                        "x": 10,
                        "y": 0
                    },
                    {
                        "x": 10,
                        "y": 1
                    },
                    {
                        "x": 10,
                        "y": 10
                    }
                ]
            },
            "you": {
                "id": "gs_bDM7cytwR9pKPcWFwFMY6BcW",
                "name": "Go  Giddy",
                "body": [
                    {
                        "x": 10,
                        "y": 5
                    },
                    {
                        "x": 9,
                        "y": 5
                    },
                    {
                        "x": 8,
                        "y": 5
                    },
                    {
                        "x": 7,
                        "y": 5
                    },
                    {
                        "x": 6,
                        "y": 5
                    },
                    {
                        "x": 6,
                        "y": 4
                    },
                    {
                        "x": 7,
                        "y": 4
                    },
                    {
                        "x": 8,
                        "y": 4
                    },
                    {
                        "x": 9,
                        "y": 4
                    }
                ],
                "head": {
                    "x": 10,
                    "y": 5
                },
                "length": 9,
                "health": 88,
                "shout": "",
                "squad": ""
            }
        }
        "#,
        );
        // https://play.battlesnake.com/g/66a99167-b263-4c9f-988e-087f5df286be/?turn=0
        let res = get_move(&data, Duration::from_millis(280));
        GameStateLog::from_api(&data).print();
        dbg!(res);
    }
}
