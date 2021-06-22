use std::collections::{HashMap, HashSet};
use std::convert::TryFrom;

use super::input::GameInfo;
use super::point::Point;
use super::snake::Snake;
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

    pub fn get_all_moves(&self) -> Vec<Vec<Direction>> {
        let mut ret: Vec<Vec<Direction>> = Vec::new();
        for snake in self.snakes.iter() {
            // iterate over all directions and remove the conflicting ones
            let mut new: Vec<Direction> = Vec::new();
            for dir in ALL_DIRECTIONS.iter() {
                if snake.head() + dir != snake.neck().unwrap() {
                    new.push(*dir);
                } // should check for more pruning? e.g. snakes that are further than my_len / 2?
            }
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
                    } else if other_snake.is_collision(&snake_head).is_some() {
                        return Some((i, CauseOfDeath::SelfCollision));
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

    fn check_food(&self, pos: &Point) -> Option<usize> {
        self.food.iter().position(|food| food == pos)
    }

    fn is_inbounds(&self, pos: &Point) -> bool {
        pos.is_not_negative() && (pos.x <= self.bound.x && pos.y <= self.bound.y)
    }
}

#[cfg(test)]
mod test {
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
            }"#);
            let board = Board::from_api(&gameinfo);
            let all_moves = board.get_all_moves();
            dbg!(&all_moves);
            assert_eq!(all_moves, vec![
            vec![
                Direction::Right,
                Direction::Left,
                Direction::Down,
            ],
            vec![
                Direction::Right,
                Direction::Left,
                Direction::Up,
            ],
            vec![
                Direction::Right,
                Direction::Left,
                Direction::Down,
            ],
            vec![
                Direction::Right,
                Direction::Left,
                Direction::Down,
            ]]);
    }
}
