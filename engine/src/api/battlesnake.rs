//! Module to handle API for Battlesnake.
//!
//! These structures are created per [BattleSnake API documentation](https://docs.battlesnake.com/references/api#battlesnake)

// External
use serde::{Deserialize, Serialize};

// Local
use super::super::point::Point;

/// Struct for Battlesnake object
#[derive(Serialize, Deserialize, Debug)]
pub struct Battlesnake {
    /// Unique identifier for this Battlesnake in the context of the current Game.
    id: String,
    /// Name given to this Battlesnake by its author
    name: String,
    /// Health value of this Battlesnake, between 0 and 100 inclusively.
    health: u8,
    /// Array of coordinates representing this Battlesnake's location on the game board. This array is ordered from head to tail.
    body: Vec<Point>,
    /// The previous response time of this Battlesnake, in milliseconds. "0" means the Battlesnake timed out and failed to respond.
    latency: String,
    /// Coordinates for this Battlesnake's head. Equivalent to the first element of the body array.
    head: Point,
    /// Length of this Battlesnake from head to tail. Equivalent to the length of the body array.
    length: i32,
    /// Message shouted by this Battlesnake on the previous turn.
    shout: String,
    /// The squad that the Battlesnake belongs to. Used to identify squad members in Squad Mode games.
    squad: String,
}

impl Battlesnake {
    pub fn is_empty(&self) -> bool {
        self.body.is_empty()
    }

    pub fn body_ref(&self) -> &Vec<Point> {
        &self.body
    }

    pub fn health(&self) -> u8 {
        self.health
    }
}
