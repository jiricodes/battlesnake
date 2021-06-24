use std::convert::TryFrom;

use serde::{Deserialize, Serialize};

use super::input::ApiSnake;
use super::path::Path;
use super::point::Point;
use super::Direction;

#[derive(Serialize, Deserialize, Debug)]
pub struct SnakeProps {
    apiversion: String,
    author: String,
    color: String,
    head: String,
    tail: String,
    version: String,
}

impl SnakeProps {
    pub fn new() -> Self {
        Self {
            apiversion: String::from("1"),
            author: String::from("jiricodes"),
            color: String::from("#622BAA"),
            head: String::from("evil"),
            tail: String::from("rattle"),
            version: String::from("0.0.1"),
        }
    }

    pub fn get_string(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}

#[derive(Debug, Clone)]
pub struct Snake {
    pub health: u8,
    pub body: Path,
}

impl Snake {
    pub fn get_default_move(&self) -> Direction {
        if let Some(neck) = self.neck() {
            if let Ok(direction) = Direction::try_from(self.head() - neck) {
                return direction;
            }
        }
        Direction::Up
    }

    pub fn neck(&self) -> Option<Point> {
        self.body.get_node(1)
    }

    pub fn head(&self) -> Point {
        self.body.first().unwrap()
    }

    pub fn advance(&mut self, dir: &Direction) {
        self.reduce_health(1);
        self.body.slide_front(dir);
    }

    pub fn feed(&mut self) {
        self.health = 100;
        self.body.extend_back(&Point::zero());
    }

    pub fn reduce_health(&mut self, val: u8) {
        if self.health >= val {
            self.health -= val;
        } else {
            self.health = 0;
        }
    }

    pub fn has_health(&self) -> bool {
        self.health != 0
    }

    pub fn is_collision(&self, p: &Point) -> Option<usize> {
        self.body.nodes.iter().position(|x| x == p)
    }

    pub fn size(&self) -> usize {
        self.body.nodes.len()
    }
}

impl TryFrom<&ApiSnake> for Snake {
    type Error = &'static str;

    fn try_from(input: &ApiSnake) -> Result<Self, Self::Error> {
        if input.body.is_empty() {
            return Err("Empty ApiSnake Body");
        }
        let body = Path::from(&input.body);
        Ok(Snake {
            health: input.health as u8,
            body,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn basics() {
        let mut snake = Snake {
            health: 50,
            body: Path::from_vec(vec![
                Point { x: 9, y: 3 },
                Point { x: 9, y: 4 },
                Point { x: 9, y: 5 },
                Point { x: 8, y: 5 },
                Point { x: 7, y: 5 },
                Point { x: 7, y: 6 },
            ]),
        };
        assert_eq!(snake.size(), 6);
        assert_eq!(snake.get_default_move(), Direction::Down);
        assert_eq!(snake.neck(), Some(Point { x: 9, y: 4 }));
        assert_eq!(snake.head(), Point { x: 9, y: 3 });
        snake.advance(&Direction::Down);
        assert_eq!(snake.size(), 6);
        assert_eq!(
            snake.body.nodes,
            vec![
                Point { x: 9, y: 2 },
                Point { x: 9, y: 3 },
                Point { x: 9, y: 4 },
                Point { x: 9, y: 5 },
                Point { x: 8, y: 5 },
                Point { x: 7, y: 5 }
            ]
        );
        assert_eq!(snake.health, 49);
        assert_eq!(snake.has_health(), true);
        snake.reduce_health(9);
        assert_eq!(snake.health, 40);
        snake.reduce_health(45);
        assert_eq!(snake.health, 0);
        assert_eq!(snake.has_health(), false);
        snake.feed();
        assert_eq!(snake.health, 100);
        assert_eq!(snake.has_health(), true);
        assert_eq!(snake.size(), 7);
        assert_eq!(
            snake.body.nodes,
            vec![
                Point { x: 9, y: 2 },
                Point { x: 9, y: 3 },
                Point { x: 9, y: 4 },
                Point { x: 9, y: 5 },
                Point { x: 8, y: 5 },
                Point { x: 7, y: 5 },
                Point { x: 7, y: 5 }
            ]
        );
        assert_eq!(snake.is_collision(&Point { x: 9, y: 4 }), Some(2));
        assert_eq!(snake.is_collision(&Point { x: 7, y: 5 }), Some(5));
        assert_eq!(snake.is_collision(&Point { x: 7, y: 6 }), None);
    }
}
