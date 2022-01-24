//! Module to handle API for Board Object.
//!
//! These structures are created per [BattleSnake API documentation](https://docs.battlesnake.com/references/api#board)

/// The game board is represented by a standard 2D grid, oriented with (0,0) in the bottom left. The Y-Axis is positive in the up direction, and X-Axis is positive to the right.
pub struct Board {
    /// The number of rows in the y-axis of the game board.
    height: i32,
    /// The number of columns in the x-axis of the game board.
    width: i32,
    /// Array of coordinates representing food locations on the game board.
    food: Vec<Point>,
    /// Array of coordinates representing hazardous locations on the game board.
    hazards: Vec<Point>,
    /// Array of Battlesnake Objects representing all Battlesnakes remaining on the game board (including yourself if you haven't been eliminated).
    snakes: Vec<Battlesnake>,
}
