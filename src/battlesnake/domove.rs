use super::astar::Astar;
use super::grid::GameGrid;
use super::grid::GridObject;
use super::heuristic::{HeurMethod, Heuristic};
use super::input::GameInfo;
use super::point::Point;

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

        // Create grid and fill it with snake bodies, hazard and food - Should be split
        let mut grid = GameGrid::new(gameinfo.get_board_dimensions());
        grid.set_snakes(gameinfo.get_snake_bodies());
        let hazards = gameinfo.get_hazards();
        grid.set_hazards(&hazards);
        let head_collision = gameinfo.get_head_collision_hazard();
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
        println!("{}", grid);

        // If length is under 8 the snake cannot trap itself
        // so lets just head towards closest food
        let mut heur = Heuristic::new(HeurMethod::Battlesnake);
        let mut move_point = Point::new(0, 0);
        let mut path = None;
        let mut astar = Astar::new();
        'pathsearch: for apple in &food {
            heur.battlesnake_init(grid.get_width() , grid.get_height(), &hazards, &head_collision, gameinfo.get_my_health(), apple);
            if astar.solve(head, *apple, &grid, &heur) {
                if astar.get_cost() <= gameinfo.get_my_health() as f32 {
                    path = Some(astar.get_path());
                    break 'pathsearch;
                }
            }
        }
        // Run the algo again, but ignore hazard
        // if path.is_none() {
        //     grid.ignore_hazard();
        //     for apple in &food {
        //         path = Astar::solve(head, *apple, &grid, Heuristic::new(HeurMethod::Manhattan));
        //         if path.is_some() {
        //             break;
        //         }
        //     }
        // }
        if path.is_some() {
            move_point = path.unwrap()[0];
        } else {
            println!("A* found no path, moving first free space");
            // Otherwise
            // if solo game -> we do hamilton
            // else -> super trooper algo?

            // Supersimple, based on empty
            move_point = head.get_right();
            let turns = head.get_neighbours();
            for point in &turns {
                let val = grid.get_value(point);
                if val == GridObject::Empty || val == GridObject::Food {
                    move_point = *point;
                }
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
