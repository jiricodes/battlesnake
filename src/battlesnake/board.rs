use std::cmp::{Eq, Ord, Ordering, PartialEq, PartialOrd};
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::convert::TryFrom;

use super::GameInfo;
use super::Path;
use super::Point;
use super::Snake;
use super::{Direction, ALL_DIRECTIONS};

#[derive(Debug, Clone)]
pub struct Board {
    pub snakes: Vec<Snake>,
    pub food: Vec<Point>,
    bound: Point,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum CauseOfDeath {
    HeadToHead,
    OtherCollision,
    SelfCollision,
    OutOfBounds,
    OutOfHealth,
}

/// A* star queue nodes (point to explore, f_score)
#[derive(Eq)]
struct OpenNode(Point, usize);

impl Ord for OpenNode {
    fn cmp(&self, other: &Self) -> Ordering {
        self.1.cmp(&other.1).reverse() //we want a min heap
    }
}

impl PartialOrd for OpenNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for OpenNode {
    fn eq(&self, other: &Self) -> bool {
        self.1 == other.1
    }
}

// Static heuristics weight, assuming all tiles are hazard
const H_WEIGHT: usize = 3 + 15;

impl Board {
    pub fn from_api(input: &GameInfo) -> Self {
        let mut snakes: Vec<Snake> = Vec::new();
        snakes.push(Snake::try_from(&input.you).unwrap());
        for snake in input.board.snakes.iter() {
            if snake.id != input.you.id {
                snakes.push(Snake::try_from(snake).unwrap());
            }
        }
        let dim = input.get_board_dimensions();
        Self {
            snakes,
            food: input.get_food(),
            bound: Point::new(dim.0 as i32 - 1, dim.1 as i32 - 1),
        }
    }

    pub fn get_pruned_moves(&self, p: &Point, n: usize) -> Vec<Direction> {
        ALL_DIRECTIONS
            .iter()
            .cloned()
            .filter(|dir| {
                let new = *p + *dir;
                self.is_inbounds(&new)
                    && self.snakes.iter().all(|snake| {
                        if new.manhattan_distance(&snake.head()) as usize > snake.size() {
                            true
                        } else if let Some(i) = snake.is_collision(&new) {
                            snake.size() >= n && i >= snake.size() - n
                        } else {
                            true
                        }
                    })
            })
            .collect()
    }

    pub fn get_all_moves(&self) -> Vec<Vec<Direction>> {
        let mut ret: Vec<Vec<Direction>> = Vec::new();
        for snake in self.snakes.iter() {
            // iterate over all directions and remove the conflicting ones
            let mut new: Vec<Direction> = self.get_pruned_moves(&snake.head(), 1);
            if new.is_empty() {
                new.push(snake.get_default_move());
            }
            ret.push(new);
        }
        ret
    }

    // pass hazard and check if head in hazard then reduce health
    pub fn advance_snakes(
        &mut self,
        moves: &[Direction],
        hazards: &Vec<Point>,
    ) -> HashMap<usize, CauseOfDeath> {
        let mut eaten_food: HashSet<usize> = HashSet::new();

        for i in 0..self.snakes.len() {
            let dir = moves
                .get(i)
                .cloned()
                .unwrap_or_else(|| self.snakes[i].get_default_move());
            self.snakes[i].advance(&dir);
            if let Some(food) = self.check_food(&self.snakes[i].head()) {
                self.snakes[i].feed();
                eaten_food.insert(food);
            }

            // process hazards
            if hazards.contains(&self.snakes[i].head()) {
                self.snakes[i].reduce_health(15);
            }
        }

        let dead_snakes = self
            .snakes
            .iter()
            .enumerate()
            .filter_map(|(i, snake)| {
                let snake_head = snake.head();
                if !snake.has_health() {
                    return Some((i, CauseOfDeath::OutOfHealth));
                }
                if !self.is_inbounds(&snake_head) {
                    return Some((i, CauseOfDeath::OutOfBounds));
                }
                for (other_i, other_snake) in self.snakes.iter().enumerate() {
                    if i != other_i {
                        if let Some(col) = other_snake.is_collision(&snake_head) {
                            if col > 0 {
                                return Some((i, CauseOfDeath::OtherCollision));
                            } else if snake.size() <= other_snake.size() {
                                return Some((i, CauseOfDeath::HeadToHead));
                            }
                        }
                    } else if let Some(col) = other_snake.is_collision(&snake_head) {
                        if col != 0 {
                            return Some((i, CauseOfDeath::SelfCollision));
                        }
                    }
                }
                None
            })
            .collect::<HashMap<usize, CauseOfDeath>>();

        if !eaten_food.is_empty() {
            self.food = self
                .food
                .iter()
                .enumerate()
                .filter_map(|(i, f)| {
                    if eaten_food.contains(&i) {
                        None
                    } else {
                        Some(*f)
                    }
                })
                .collect();
        }

        if !dead_snakes.is_empty() {
            self.snakes = self
                .snakes
                .iter()
                .enumerate()
                .filter_map(|(i, s)| {
                    if dead_snakes.contains_key(&i) {
                        None
                    } else {
                        Some(s.clone())
                    }
                })
                .collect();
        }

        dead_snakes
    }

    pub fn get_my_death (
        &mut self,
    ) -> Option <CauseOfDeath> {
        let snake = self.snakes[0].clone();
        let snake_head = snake.head();
        if !snake.has_health() {
            return Some(CauseOfDeath::OutOfHealth);
        }
        if !self.is_inbounds(&snake_head) {
            return Some(CauseOfDeath::OutOfBounds);
        }
        for (other_i, other_snake) in self.snakes.iter().enumerate() {
            if other_i != 0 {
                if let Some(col) = other_snake.is_collision(&snake_head) {
                    if col > 0 {
                        return Some(CauseOfDeath::OtherCollision);
                    } else if snake.size() <= other_snake.size() {
                        return Some(CauseOfDeath::HeadToHead);
                    }
                }
            } else if let Some(col) = other_snake.is_collision(&snake_head) {
                if col != 0 {
                    return Some(CauseOfDeath::SelfCollision);
                }
            }
        }
        None
    }

    fn check_food(&self, pos: &Point) -> Option<usize> {
        self.food.iter().position(|food| food == pos)
    }

    fn is_inbounds(&self, pos: &Point) -> bool {
        pos.is_not_negative() && (pos.x <= self.bound.x && pos.y <= self.bound.y)
    }

    pub fn astar(&self, s: Point, e: Point, hazards: &Vec<Point>) -> Option<(usize, Path)> {
        // keeps open points and the f_cost
        let mut openset: BinaryHeap<OpenNode> = BinaryHeap::new();
        // keeps min cost per point <point, (g_cost, turns, parent)
        let mut closedset: HashMap<Point, (usize, usize, Option<Point>)> = HashMap::new();

        openset.push(OpenNode(s, s.manhattan_distance(&e) * H_WEIGHT));
        closedset.insert(s, (0, 0, None));

        while let Some(OpenNode(point, _)) = openset.pop() {
            let (mut g_score, turn) = closedset
                .get(&point)
                .map(|(g, t, ..)| (*g, *t + 1))
                .unwrap_or((0, 0));
            if point == e {
                let mut nodes = vec![point];
                while let Some((_, _, Some(parent))) = closedset.get(nodes.last().unwrap()) {
                    nodes.push(*parent);
                }
                if hazards.contains(&point) {
                    g_score -= 15;
                }
                return Some((g_score, Path::from_vec(nodes)));
            }

            let to_explore = self
                .get_pruned_moves(&point, turn)
                .iter()
                .map(|x| point + x)
                .collect::<Vec<_>>();

            for n in to_explore {
                let new_g_score = if hazards.contains(&n) {
                    g_score + 16
                } else {
                    g_score + 1
                };
                let old_g_score = closedset.get(&n);
                if old_g_score.is_none() || new_g_score < (*old_g_score.unwrap()).0 {
                    closedset.insert(n, (new_g_score, turn, Some(point)));
                    openset.push(OpenNode(
                        n,
                        new_g_score + n.manhattan_distance(&e) * H_WEIGHT,
                    ))
                }
            }
        }
        None
    }
}

#[cfg(test)]
mod test {
    use super::super::game_logger::GameStateLog;
    use super::*;

    #[test]
    fn from_api() {
        let gameinfo = GameInfo::new(
            r#"{
            "game": {
                "id": "5e6c3d1b-6404-403a-9803-09bd2b99224f",
                "ruleset": {
                    "name": "royale",
                    "version": ""
                },
                "timeout": 500
            },
            "turn": 4,
            "board": {
                "width": 11,
                "height": 11,
                "snakes": [
                    {
                        "id": "gs_SJWHXg6r4QtxBJQ4qQcT6tSH",
                        "name": "Barry",
                        "body": [
                            {
                                "x": 2,
                                "y": 6
                            },
                            {
                                "x": 2,
                                "y": 7
                            },
                            {
                                "x": 1,
                                "y": 7
                            }
                        ],
                        "head": {
                            "x": 2,
                            "y": 6
                        },
                        "length": 3,
                        "health": 96,
                        "shout": "",
                        "squad": ""
                    },
                    {
                        "id": "gs_kcVF7X8vkHkPdKPckKwfKRvF",
                        "name": "Go  Giddy",
                        "body": [
                            {
                                "x": 6,
                                "y": 6
                            },
                            {
                                "x": 7,
                                "y": 6
                            },
                            {
                                "x": 8,
                                "y": 6
                            },
                            {
                                "x": 9,
                                "y": 6
                            }
                        ],
                        "head": {
                            "x": 6,
                            "y": 6
                        },
                        "length": 4,
                        "health": 98,
                        "shout": "",
                        "squad": ""
                    },
                    {
                        "id": "gs_6Tfrmk6tdw87PDDvjq4Yk9GM",
                        "name": "ninemo",
                        "body": [
                            {
                                "x": 0,
                                "y": 8
                            },
                            {
                                "x": 0,
                                "y": 7
                            },
                            {
                                "x": 0,
                                "y": 6
                            },
                            {
                                "x": 0,
                                "y": 5
                            },
                            {
                                "x": 0,
                                "y": 5
                            }
                        ],
                        "head": {
                            "x": 0,
                            "y": 8
                        },
                        "length": 5,
                        "health": 100,
                        "shout": "",
                        "squad": ""
                    },
                    {
                        "id": "gs_xd7wttW7PBxPBK3WWGK793xM",
                        "name": "CertnSnake",
                        "body": [
                            {
                                "x": 9,
                                "y": 5
                            },
                            {
                                "x": 9,
                                "y": 4
                            },
                            {
                                "x": 9,
                                "y": 3
                            }
                        ],
                        "head": {
                            "x": 9,
                            "y": 5
                        },
                        "length": 3,
                        "health": 96,
                        "shout": "",
                        "squad": ""
                    }
                ],
                "food": [
                    {
                        "x": 10,
                        "y": 0
                    },
                    {
                        "x": 5,
                        "y": 5
                    },
                    {
                        "x": 6,
                        "y": 10
                    },
                    {
                        "x": 5,
                        "y": 7
                    }
                ],
                "hazards": []
            },
            "you": {
                "id": "gs_kcVF7X8vkHkPdKPckKwfKRvF",
                "name": "Go  Giddy",
                "body": [
                    {
                        "x": 6,
                        "y": 6
                    },
                    {
                        "x": 7,
                        "y": 6
                    },
                    {
                        "x": 8,
                        "y": 6
                    },
                    {
                        "x": 9,
                        "y": 6
                    }
                ],
                "head": {
                    "x": 6,
                    "y": 6
                },
                "length": 4,
                "health": 98,
                "shout": "",
                "squad": ""
            }
        }"#,
        );
        let board = Board::from_api(&gameinfo);
        assert_eq!(board.snakes[0].health, 98);
        assert_eq!(board.snakes[1].health, 96);
        assert_eq!(board.snakes[2].health, 100);
        assert_eq!(board.snakes[3].health, 96);
        assert_eq!(
            board.snakes[0].body.nodes,
            vec![
                Point::new(6, 6),
                Point::new(7, 6),
                Point::new(8, 6),
                Point::new(9, 6)
            ]
        );
        assert_eq!(
            board.snakes[1].body.nodes,
            vec![Point::new(2, 6), Point::new(2, 7), Point::new(1, 7)]
        );
        assert_eq!(
            board.snakes[2].body.nodes,
            vec![
                Point::new(0, 8),
                Point::new(0, 7),
                Point::new(0, 6),
                Point::new(0, 5),
                Point::new(0, 5)
            ]
        );
        assert_eq!(
            board.snakes[3].body.nodes,
            vec![Point::new(9, 5), Point::new(9, 4), Point::new(9, 3)]
        );
        assert_eq!(
            board.food,
            vec![
                Point { x: 10, y: 0 },
                Point { x: 5, y: 5 },
                Point { x: 6, y: 10 },
                Point { x: 5, y: 7 }
            ]
        );
    }

    #[test]
    fn available_moves() {
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
                "turn": 12,
                "board": {
                    "width": 11,
                    "height": 11,
                    "snakes": [
                        {
                            "id": "gs_9Xdgwh9wPKktBtXphrMt4d67",
                            "name": "Eel In Snake's Clothing",
                            "body": [
                                {
                                    "x": 7,
                                    "y": 7
                                },
                                {
                                    "x": 7,
                                    "y": 6
                                },
                                {
                                    "x": 7,
                                    "y": 5
                                },
                                {
                                    "x": 6,
                                    "y": 5
                                }
                            ],
                            "head": {
                                "x": 7,
                                "y": 7
                            },
                            "length": 4,
                            "health": 90,
                            "shout": "",
                            "squad": ""
                        },
                        {
                            "id": "gs_fhDCrB9BmRfgWBgXj9cBCxqR",
                            "name": "Go  Giddy",
                            "body": [
                                {
                                    "x": 5,
                                    "y": 5
                                },
                                {
                                    "x": 5,
                                    "y": 6
                                },
                                {
                                    "x": 5,
                                    "y": 7
                                },
                                {
                                    "x": 5,
                                    "y": 8
                                },
                                {
                                    "x": 5,
                                    "y": 8
                                }
                            ],
                            "head": {
                                "x": 5,
                                "y": 5
                            },
                            "length": 5,
                            "health": 100,
                            "shout": "",
                            "squad": ""
                        },
                        {
                            "id": "gs_KTTWwyytWTBjgXkyh8gDp9VD",
                            "name": "Untimely Neglected Wearable",
                            "body": [
                                {
                                    "x": 2,
                                    "y": 0
                                },
                                {
                                    "x": 2,
                                    "y": 1
                                },
                                {
                                    "x": 1,
                                    "y": 1
                                },
                                {
                                    "x": 1,
                                    "y": 0
                                },
                                {
                                    "x": 0,
                                    "y": 0
                                },
                                {
                                    "x": 0,
                                    "y": 0
                                }
                            ],
                            "head": {
                                "x": 2,
                                "y": 0
                            },
                            "length": 6,
                            "health": 100,
                            "shout": "",
                            "squad": ""
                        },
                        {
                            "id": "gs_8fMbQg9DHB9fMGxR7Hv39P9Q",
                            "name": "Danger Noodle - A*/Flood",
                            "body": [
                                {
                                    "x": 4,
                                    "y": 2
                                },
                                {
                                    "x": 4,
                                    "y": 3
                                },
                                {
                                    "x": 5,
                                    "y": 3
                                },
                                {
                                    "x": 5,
                                    "y": 4
                                },
                                {
                                    "x": 5,
                                    "y": 4
                                }
                            ],
                            "head": {
                                "x": 4,
                                "y": 2
                            },
                            "length": 5,
                            "health": 100,
                            "shout": "",
                            "squad": ""
                        }
                    ],
                    "food": [
                        {
                            "x": 1,
                            "y": 4
                        }
                    ],
                    "hazards": []
                },
                "you": {
                    "id": "gs_fhDCrB9BmRfgWBgXj9cBCxqR",
                    "name": "Go  Giddy",
                    "body": [
                        {
                            "x": 5,
                            "y": 5
                        },
                        {
                            "x": 5,
                            "y": 6
                        },
                        {
                            "x": 5,
                            "y": 7
                        },
                        {
                            "x": 5,
                            "y": 8
                        },
                        {
                            "x": 5,
                            "y": 8
                        }
                    ],
                    "head": {
                        "x": 5,
                        "y": 5
                    },
                    "length": 5,
                    "health": 100,
                    "shout": "",
                    "squad": ""
                }
            }"#,
        );
        let board = Board::from_api(&gameinfo);
        let all_moves = board.get_all_moves();
        let state_log = GameStateLog::from_api(&gameinfo);
        state_log.print();
        assert_eq!(
            all_moves,
            vec![
                vec![Direction::Right, Direction::Left,],
                vec![Direction::Right, Direction::Left, Direction::Up,],
                vec![Direction::Right,],
                vec![Direction::Right, Direction::Left, Direction::Down,]
            ]
        );
        let future_moves = board.get_pruned_moves(&Point { x: 6, y: 7 }, 2);
        assert_eq!(future_moves, vec![Direction::Up, Direction::Down]);
        let future_moves = board.get_pruned_moves(&Point { x: 6, y: 7 }, 3);
        assert_eq!(
            future_moves,
            vec![Direction::Left, Direction::Up, Direction::Down]
        );
    }

    #[test]
    fn advance() {
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
                                "x": 3,
                                "y": 7
                            },
                            {
                                "x": 4,
                                "y": 7
                            },
                            {
                                "x": 4,
                                "y": 8
                            },
                            {
                                "x": 5,
                                "y": 8
                            },
                            {
                                "x": 6,
                                "y": 8
                            }
                        ],
                        "head": {
                            "x": 3,
                            "y": 7
                        },
                        "length": 5,
                        "health": 88,
                        "shout": "",
                        "squad": ""
                    },
                    {
                        "id": "gs_fhDCrB9BmRfgWBgXj9cBCxqR",
                        "name": "Go  Giddy",
                        "body": [
                            {
                                "x": 9,
                                "y": 3
                            },
                            {
                                "x": 9,
                                "y": 4
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
                                "x": 7,
                                "y": 6
                            },
                            {
                                "x": 7,
                                "y": 6
                            }
                        ],
                        "head": {
                            "x": 9,
                            "y": 3
                        },
                        "length": 7,
                        "health": 100,
                        "shout": "",
                        "squad": ""
                    },
                    {
                        "id": "gs_KTTWwyytWTBjgXkyh8gDp9VD",
                        "name": "Untimely Neglected Wearable",
                        "body": [
                            {
                                "x": 7,
                                "y": 3
                            },
                            {
                                "x": 7,
                                "y": 2
                            },
                            {
                                "x": 8,
                                "y": 2
                            },
                            {
                                "x": 9,
                                "y": 2
                            },
                            {
                                "x": 9,
                                "y": 1
                            },
                            {
                                "x": 8,
                                "y": 1
                            },
                            {
                                "x": 7,
                                "y": 1
                            }
                        ],
                        "head": {
                            "x": 7,
                            "y": 3
                        },
                        "length": 7,
                        "health": 96,
                        "shout": "",
                        "squad": ""
                    },
                    {
                        "id": "gs_8fMbQg9DHB9fMGxR7Hv39P9Q",
                        "name": "Danger Noodle - A*/Flood",
                        "body": [
                            {
                                "x": 6,
                                "y": 4
                            },
                            {
                                "x": 6,
                                "y": 3
                            },
                            {
                                "x": 6,
                                "y": 2
                            },
                            {
                                "x": 5,
                                "y": 2
                            },
                            {
                                "x": 4,
                                "y": 2
                            }
                        ],
                        "head": {
                            "x": 6,
                            "y": 4
                        },
                        "length": 5,
                        "health": 80,
                        "shout": "",
                        "squad": ""
                    }
                ],
                "food": [
                    {
                        "x": 10,
                        "y": 6
                    }
                ],
                "hazards": [
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
                        "y": 2
                    },
                    {
                        "x": 10,
                        "y": 3
                    },
                    {
                        "x": 10,
                        "y": 4
                    },
                    {
                        "x": 10,
                        "y": 5
                    },
                    {
                        "x": 10,
                        "y": 6
                    },
                    {
                        "x": 10,
                        "y": 7
                    },
                    {
                        "x": 10,
                        "y": 8
                    },
                    {
                        "x": 10,
                        "y": 9
                    },
                    {
                        "x": 10,
                        "y": 10
                    }
                ]
            },
            "you": {
                "id": "gs_fhDCrB9BmRfgWBgXj9cBCxqR",
                "name": "Go  Giddy",
                "body": [
                    {
                        "x": 9,
                        "y": 3
                    },
                    {
                        "x": 9,
                        "y": 4
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
                        "x": 7,
                        "y": 6
                    },
                    {
                        "x": 7,
                        "y": 6
                    }
                ],
                "head": {
                    "x": 9,
                    "y": 3
                },
                "length": 7,
                "health": 100,
                "shout": "",
                "squad": ""
            }
        }"#,
        );
        let board = Board::from_api(&gameinfo);
        let hazards = gameinfo.get_hazards();
        //  [Right, Left, Down]
        //  [Left, Up, Down]
        //  [Right, Left, Up]
        //  [Right, Left, Up],

        // Expected: OK, OK, OK, OK
        let moves = vec![
            Direction::Right,
            Direction::Left,
            Direction::Right,
            Direction::Right,
        ];
        let mut new_board = board.clone();
        let res = new_board.advance_snakes(&moves, &hazards);
        let expected: HashMap<usize, CauseOfDeath> = HashMap::new();
        assert_eq!(res, expected);
        // Expected: HeadCollision, OK, HeadCollision, OK
        let moves = vec![
            Direction::Left,
            Direction::Left,
            Direction::Right,
            Direction::Right,
        ];
        let mut new_board = board.clone();
        let res = new_board.advance_snakes(&moves, &hazards);
        let mut expected: HashMap<usize, CauseOfDeath> = HashMap::new();
        expected.insert(2, CauseOfDeath::HeadToHead);
        expected.insert(0, CauseOfDeath::HeadToHead);
        assert_eq!(res, expected);
        // // Expected: CollisionOther, OK, CollisionOther, OK
        let moves = vec![
            Direction::Down,
            Direction::Left,
            Direction::Left,
            Direction::Right,
        ];
        let mut new_board = board.clone();
        let res = new_board.advance_snakes(&moves, &hazards);
        let mut expected: HashMap<usize, CauseOfDeath> = HashMap::new();
        expected.insert(2, CauseOfDeath::OtherCollision);
        expected.insert(0, CauseOfDeath::OtherCollision);
        assert_eq!(res, expected);
    }

    #[test]
    fn astar() {
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
                                "x": 3,
                                "y": 7
                            },
                            {
                                "x": 4,
                                "y": 7
                            },
                            {
                                "x": 4,
                                "y": 8
                            },
                            {
                                "x": 5,
                                "y": 8
                            },
                            {
                                "x": 6,
                                "y": 8
                            }
                        ],
                        "head": {
                            "x": 3,
                            "y": 7
                        },
                        "length": 5,
                        "health": 88,
                        "shout": "",
                        "squad": ""
                    },
                    {
                        "id": "gs_fhDCrB9BmRfgWBgXj9cBCxqR",
                        "name": "Go  Giddy",
                        "body": [
                            {
                                "x": 9,
                                "y": 3
                            },
                            {
                                "x": 9,
                                "y": 4
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
                                "x": 7,
                                "y": 6
                            },
                            {
                                "x": 7,
                                "y": 6
                            }
                        ],
                        "head": {
                            "x": 9,
                            "y": 3
                        },
                        "length": 7,
                        "health": 100,
                        "shout": "",
                        "squad": ""
                    },
                    {
                        "id": "gs_KTTWwyytWTBjgXkyh8gDp9VD",
                        "name": "Untimely Neglected Wearable",
                        "body": [
                            {
                                "x": 7,
                                "y": 3
                            },
                            {
                                "x": 7,
                                "y": 2
                            },
                            {
                                "x": 8,
                                "y": 2
                            },
                            {
                                "x": 9,
                                "y": 2
                            },
                            {
                                "x": 9,
                                "y": 1
                            },
                            {
                                "x": 8,
                                "y": 1
                            },
                            {
                                "x": 7,
                                "y": 1
                            }
                        ],
                        "head": {
                            "x": 7,
                            "y": 3
                        },
                        "length": 7,
                        "health": 96,
                        "shout": "",
                        "squad": ""
                    },
                    {
                        "id": "gs_8fMbQg9DHB9fMGxR7Hv39P9Q",
                        "name": "Danger Noodle - A*/Flood",
                        "body": [
                            {
                                "x": 6,
                                "y": 4
                            },
                            {
                                "x": 6,
                                "y": 3
                            },
                            {
                                "x": 6,
                                "y": 2
                            },
                            {
                                "x": 5,
                                "y": 2
                            },
                            {
                                "x": 4,
                                "y": 2
                            }
                        ],
                        "head": {
                            "x": 6,
                            "y": 4
                        },
                        "length": 5,
                        "health": 80,
                        "shout": "",
                        "squad": ""
                    }
                ],
                "food": [
                    {
                        "x": 10,
                        "y": 6
                    }
                ],
                "hazards": [
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
                        "y": 2
                    },
                    {
                        "x": 10,
                        "y": 3
                    },
                    {
                        "x": 10,
                        "y": 4
                    },
                    {
                        "x": 10,
                        "y": 5
                    },
                    {
                        "x": 10,
                        "y": 6
                    },
                    {
                        "x": 10,
                        "y": 7
                    },
                    {
                        "x": 10,
                        "y": 8
                    },
                    {
                        "x": 10,
                        "y": 9
                    },
                    {
                        "x": 10,
                        "y": 10
                    }
                ]
            },
            "you": {
                "id": "gs_fhDCrB9BmRfgWBgXj9cBCxqR",
                "name": "Go  Giddy",
                "body": [
                    {
                        "x": 9,
                        "y": 3
                    },
                    {
                        "x": 9,
                        "y": 4
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
                        "x": 7,
                        "y": 6
                    },
                    {
                        "x": 7,
                        "y": 6
                    }
                ],
                "head": {
                    "x": 9,
                    "y": 3
                },
                "length": 7,
                "health": 100,
                "shout": "",
                "squad": ""
            }
        }"#,
        );
        let board = Board::from_api(&gameinfo);
        let hazards = gameinfo.get_hazards();
        GameStateLog::from_api(&gameinfo).print();
        let path = board.astar(Point { x: 9, y: 3 }, Point { x: 10, y: 6 }, hazards);
        assert!(path.is_some());
        let (g_score, path) = path.unwrap();
        assert_eq!(g_score, 49);
        assert_eq!(
            path.nodes,
            vec![
                Point { x: 10, y: 6 },
                Point { x: 10, y: 5 },
                Point { x: 10, y: 4 },
                Point { x: 10, y: 3 },
                Point { x: 9, y: 3 },
            ]
        );
        let path = board.astar(Point { x: 6, y: 4 }, Point { x: 10, y: 6 }, hazards);
        assert!(path.is_some());
        let (g_score, path) = path.unwrap();
        assert_eq!(g_score, 6);
        assert_eq!(
            path.nodes,
            vec![
                Point { x: 10, y: 6 },
                Point { x: 9, y: 6 },
                Point { x: 8, y: 6 },
                Point { x: 7, y: 6 },
                Point { x: 6, y: 6 },
                Point { x: 6, y: 5 },
                Point { x: 6, y: 4 },
            ]
        );
        let path = board.astar(Point { x: 7, y: 3 }, Point { x: 10, y: 6 }, hazards);
        assert!(path.is_none());
    }
}
