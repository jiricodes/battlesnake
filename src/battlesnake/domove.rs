use super::grid::GameGrid;
use super::grid::GridObject;
use super::input::GameInfo;
use serde::{Deserialize, Serialize};

use std::fmt;

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
#[serde(rename_all(serialize = "lowercase", deserialize = "lowercase"))]
pub enum Movement {
    Right,
    Left,
    Up,
    Down,
}

impl fmt::Display for Movement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let move_text = match self {
            Movement::Right => "right",
            Movement::Left => "left",
            Movement::Up => "up",
            Movement::Down => "down",
        };
        write!(f, "{}", move_text)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Move {
    #[serde(rename = "move")]
    movement: Movement,
    #[serde(skip_serializing_if = "Option::is_none")]
    shout: Option<String>,
}

impl Move {
    pub fn new(input: &str) -> Self {
        // Parse game information
        let gameinfo = GameInfo::new(&input);

        // Create grid and fill it with snake bodies and food
        let mut grid = GameGrid::new(gameinfo.get_board_dimensions());
        grid.set_snakes(gameinfo.get_snake_bodies());
        let food = gameinfo.get_food();
        grid.set_food(&food);

        // Log my snakes id
        println!("Turn: {}", gameinfo.get_turn());
        println!("Snake ID: {}", gameinfo.get_my_id());
        // Get my snake's head and length
        let head = gameinfo.get_my_head();
        println!("Head at: {}", head);
        let my_len = gameinfo.get_my_length();
        println!("Length: {}", my_len);
        // Check closest FOOD
        let closest_food = head.find_closest(food);
        println!("Closest Food: {}", closest_food);
        println!("{}", grid);

        // If length is under 8 the snake cannot trap itself
        // so lets just head towards closest food

        // Otherwise
        // if solo game -> we do hamilton
        // else -> super trooper algo?

        // Supersimple, based on empty
        let mut move_point = head.get_right();
        let turns = head.get_neighbours();
        for point in &turns {
            let val = grid.get_value(point);
            if val == GridObject::EMPTY || val == GridObject::FOOD {
                move_point = *point;
            }
        }

        // selects move that is either to empty or food cell
        Self {
            movement: head.get_neighbour_direction(move_point).unwrap(),
            shout: None,
        }
    }

    pub fn as_option_string(input: &str) -> Option<String> {
        Some(serde_json::to_string(&Move::new(&input)).unwrap())
    }

    pub fn get_json_string(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let shout = if self.shout.is_some() {
            self.shout.as_ref().unwrap()
        } else {
            ""
        };
        write!(f, "{} | {}", self.movement, shout)
    }
}
