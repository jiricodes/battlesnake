use log::*;
use std::cmp::{Eq, Ord, Ordering, PartialEq, PartialOrd, max};
use std::collections::BinaryHeap;
use std::sync::Mutex;
use std::time::{Duration, SystemTime};

use rayon::prelude::*;

use super::Direction;
use super::GameInfo;
use super::Move;
use super::Point;
use super::{Board, CauseOfDeath};

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

    let n_snakes = board.snakes.len();

    if snake_index == 0 && n_snakes == 1 {
        return 2.0;
    }

    let snake = board.snakes.get(snake_index).unwrap();

    let areas = board.get_area(hazards);
    let snake_area = areas.get(snake_index).unwrap();
    let total_area = max(1, areas.iter().map(|a| a.0).sum());

    // Area Control
    // println!("MyArea: {} | Total {}", snake_area.0, total_area);
    let area_control_score = snake_area.0 as f32 / total_area as f32;

    let snake_hp = snake.health as usize;
    // let snake_mod_hp = if snake_hp > 20 { snake_hp - 20 } else {snake_hp};
    let snake_head = snake.head();
    let snake_len = snake.size();

    let food_score = if snake_hp == 0 {
        0.0
    } else {
        if let Some(f) = snake_area.2 {
            // println!("Food g_score: {}", board.astar(snake_head, f, hazards).unwrap().0 as f32);
            max_f32(0.0, (snake_hp as f32 - board.astar(snake_head, f, hazards).unwrap().0 as f32) / snake_hp as f32)
        } else {
            min_f32(1.0, 0.15 * snake_hp as f32 * n_snakes as f32 / total_area as f32)
        }
    };

    let htoh_score = board.snakes.iter().enumerate().filter(|(other_index, other)| {
        *other_index == snake_index || other.size() < snake_len || snake_head.manhattan_distance(&other.head()) > 2
    }).count() as f32 / n_snakes as f32;

    let mut total_dist = 1.0;
    let mut bigger_dist = 1.0;
    let mut smaller_dist = 1.0;
    for (i, other) in board.snakes.iter().enumerate() {
        if i == snake_index { continue; }
        let d = snake_head.manhattan_distance(&other.head()) as f32;
        total_dist += d;
        if other.size() < snake_len {
            smaller_dist += d;
        } else {
            bigger_dist += d;
        }
    }
    // println!("Smaller: {} | Bigger: {} | Total: {}", smaller_dist, bigger_dist, total_dist);
    let h2h_dist_score = (1.0 - smaller_dist / total_dist) * (bigger_dist / total_dist);

    let snakes_ratio = 1.0 / n_snakes as f32;

    
    let mut tot_lens = 0;
    for s in board.snakes.iter() {
        tot_lens += s.size();
    }
    let mut len_score = snake_len as f32 / tot_lens as f32;
    len_score = len_score * len_score;
    
    let health_score = snake_hp as f32 / 100.0;

    // let mut aggression: f32 = 0.1;
    // for snake in board.snakes.iter() {
    //     if snake.size() < snake_len {
    //         if snake.head() == board.snakes[snake_index].head() {
    //             aggression = 1.1;
    //         } else {
    //             aggression = 1.0
    //                 / snake
    //                     .head()
    //                     .manhattan_distance(&board.snakes[snake_index].head())
    //                     as f32;
    //         }
    //     }
    // }
    
    // println!("Area: {:10.6}\nFood: {:10.6}\nH2H: {:11.6}\nSrat: {:10.6}\nlrat: {:10.6}\nhp: {:12.6}\nprox: {:10.6}\n", area_control_score, food_score, htoh_score, snakes_ratio, len_score, health_score, h2h_dist_score);
    area_control_score * food_score * htoh_score * snakes_ratio * len_score * health_score * h2h_dist_score
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

        // if first.depth == 20 {
        //     break 'minimax;
        // }

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
                if state.depth < 3 {
                    println!("Depth {} option: dir={:?} score={}", state.depth, state.root, state.h);
                }
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
    
    #[test]
    fn test_heuristic() {
        let gameinfo = GameInfo::new(
            r#"{
            "game": {
                "id": "2c2d43ec-0fdb-4bf4-9a00-8f1d243238d4",
                "ruleset": {
                    "name": "royale",
                    "version": ""
                },
                "timeout": 500
            },
            "turn": 69,
            "board": {
                "width": 11,
                "height": 11,
                "snakes": [
                    {
                        "id": "gs_9Xdgwh9wPKktBtXphrMt4d67",
                        "name": "Eel In Snake's Clothing",
                        "body": [
                            {
                                "x": 0,
                                "y": 5
                            },
                            {
                                "x": 1,
                                "y": 5
                            },
                            {
                                "x": 1,
                                "y": 6
                            },
                            {
                                "x": 1,
                                "y": 7
                            },
                            {
                                "x": 0,
                                "y": 7
                            }
                        ],
                        "head": {
                            "x": 0,
                            "y": 5
                        },
                        "length": 5,
                        "health": 97,
                        "shout": "",
                        "squad": ""
                    },
                    {
                        "id": "gs_fhDCrB9BmRfgWBgXj9cBCxqR",
                        "name": "Go  Giddy",
                        "body": [
                            {
                                "x": 7,
                                "y": 2
                            },
                            {
                                "x": 7,
                                "y": 1
                            },
                            {
                                "x": 6,
                                "y": 1
                            },
                            {
                                "x": 6,
                                "y": 0
                            }
                        ],
                        "head": {
                            "x": 7,
                            "y": 2
                        },
                        "length": 4,
                        "health": 40,
                        "shout": "",
                        "squad": ""
                    },
                    {
                        "id": "gs_KTTWwyytWTBjgXkyh8gDp9VD",
                        "name": "Untimely Neglected Wearable",
                        "body": [
                            {
                                "x": 2,
                                "y": 3
                            },
                            {
                                "x": 1,
                                "y": 3
                            },
                            {
                                "x": 0,
                                "y": 3
                            },
                            {
                                "x": 0,
                                "y": 2
                            }
                        ],
                        "head": {
                            "x": 2,
                            "y": 3
                        },
                        "length": 4,
                        "health": 95,
                        "shout": "",
                        "squad": ""
                    },
                    {
                        "id": "gs_8fMbQg9DHB9fMGxR7Hv39P9Q",
                        "name": "Danger Noodle - A*/Flood",
                        "body": [
                            {
                                "x": 5,
                                "y": 4
                            },
                            {
                                "x": 5,
                                "y": 5
                            },
                            {
                                "x": 5,
                                "y": 6
                            },
                            {
                                "x": 4,
                                "y": 6
                            },
                            {
                                "x": 3,
                                "y": 6
                            }
                        ],
                        "head": {
                            "x": 5,
                            "y": 4
                        },
                        "length": 5,
                        "health": 99,
                        "shout": "",
                        "squad": ""
                    }
                ],
                "food": [
                    {
                        "x": 9,
                        "y": 1
                    }
                ],
                "hazards": []
            },
            "you": {
                "id": "gs_fhDCrB9BmRfgWBgXj9cBCxqR",
                        "name": "Go  Giddy",
                        "body": [
                            {
                                "x": 7,
                                "y": 2
                            },
                            {
                                "x": 7,
                                "y": 1
                            },
                            {
                                "x": 6,
                                "y": 1
                            },
                            {
                                "x": 6,
                                "y": 0
                            }
                        ],
                        "head": {
                            "x": 7,
                            "y": 2
                        },
                        "length": 4,
                        "health": 40,
                        "shout": "",
                        "squad": ""
            }
        }"#,
        );
        let board = Board::from_api(&gameinfo);
        let hazards = gameinfo.get_hazards();
        GameStateLog::from_api(&gameinfo).print();
        let heur = heuristic(&board, 0, &hazards);
        dbg!(heur);
        let gameinfo = GameInfo::new(
            r#"{
            "game": {
                "id": "2c2d43ec-0fdb-4bf4-9a00-8f1d243238d4",
                "ruleset": {
                    "name": "royale",
                    "version": ""
                },
                "timeout": 500
            },
            "turn": 32,
            "board": {
                "width": 11,
                "height": 11,
                "snakes": [
                    {
                        "id": "gs_9Xdgwh9wPKktBtXphrMt4d67",
                        "name": "Eel In Snake's Clothing",
                        "body": [
                            {
                                "x": 0,
                                "y": 5
                            },
                            {
                                "x": 1,
                                "y": 5
                            },
                            {
                                "x": 1,
                                "y": 6
                            },
                            {
                                "x": 1,
                                "y": 7
                            },
                            {
                                "x": 0,
                                "y": 7
                            }
                        ],
                        "head": {
                            "x": 0,
                            "y": 5
                        },
                        "length": 5,
                        "health": 97,
                        "shout": "",
                        "squad": ""
                    },
                    {
                        "id": "gs_fhDCrB9BmRfgWBgXj9cBCxqR",
                        "name": "Go  Giddy",
                        "body": [
                            {
                                "x": 8,
                                "y": 1
                            },
                            {
                                "x": 7,
                                "y": 1
                            },
                            {
                                "x": 6,
                                "y": 1
                            },
                            {
                                "x": 6,
                                "y": 0
                            }
                        ],
                        "head": {
                            "x": 8,
                            "y": 1
                        },
                        "length": 4,
                        "health": 40,
                        "shout": "",
                        "squad": ""
                    },
                    {
                        "id": "gs_KTTWwyytWTBjgXkyh8gDp9VD",
                        "name": "Untimely Neglected Wearable",
                        "body": [
                            {
                                "x": 2,
                                "y": 3
                            },
                            {
                                "x": 1,
                                "y": 3
                            },
                            {
                                "x": 0,
                                "y": 3
                            },
                            {
                                "x": 0,
                                "y": 2
                            }
                        ],
                        "head": {
                            "x": 2,
                            "y": 3
                        },
                        "length": 4,
                        "health": 95,
                        "shout": "",
                        "squad": ""
                    },
                    {
                        "id": "gs_8fMbQg9DHB9fMGxR7Hv39P9Q",
                        "name": "Danger Noodle - A*/Flood",
                        "body": [
                            {
                                "x": 5,
                                "y": 4
                            },
                            {
                                "x": 5,
                                "y": 5
                            },
                            {
                                "x": 5,
                                "y": 6
                            },
                            {
                                "x": 4,
                                "y": 6
                            },
                            {
                                "x": 3,
                                "y": 6
                            }
                        ],
                        "head": {
                            "x": 5,
                            "y": 4
                        },
                        "length": 5,
                        "health": 99,
                        "shout": "",
                        "squad": ""
                    }
                ],
                "food": [
                    {
                        "x": 9,
                        "y": 1
                    }
                ],
                "hazards": []
            },
            "you": {
                "id": "gs_fhDCrB9BmRfgWBgXj9cBCxqR",
                        "name": "Go  Giddy",
                        "body": [
                            {
                                "x": 8,
                                "y": 1
                            },
                            {
                                "x": 7,
                                "y": 1
                            },
                            {
                                "x": 6,
                                "y": 1
                            },
                            {
                                "x": 6,
                                "y": 0
                            }
                        ],
                        "head": {
                            "x": 8,
                            "y": 1
                        },
                        "length": 4,
                        "health": 40,
                        "shout": "",
                        "squad": ""
            }
        }"#,
        );
        let board = Board::from_api(&gameinfo);
        let hazards = gameinfo.get_hazards();
        GameStateLog::from_api(&gameinfo).print();
        let heur = heuristic(&board, 0, &hazards);
        dbg!(heur);
    }
    
}
