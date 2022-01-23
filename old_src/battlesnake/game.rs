//! Game state for minmax, ab pruning and iterative deepening eventualy - Possibly with EG for game state evaluation? Or even MCTS?
//! - for now consider using HEALTH as move indicator
//! - the gamestate should be evaluate self somehow
//! 
use super::GameGrid;

pub struct GameState {
    grid: GameGrid,
    me: Option<Snake>,
    oponents: Option<Vec<Snake>>,
    remaining: Time,
}