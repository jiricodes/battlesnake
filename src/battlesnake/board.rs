use std::convert::TryFrom;

use super::input::GameInfo;
use super::point::Point;
use super::snake::Snake;
use super::Direction;

#[derive(Debug)]
pub struct Board {
    pub snakes: Vec<Snake>,
    pub food: Vec<Point>,
    bound: Point,
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
        let ret: Vec<Vec<Direction>> = Vec::new();
        for snake in self.snakes {
            // iterate over all directions and remove the conflicting ones
        }
        unimplemented!();
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
    }
}
