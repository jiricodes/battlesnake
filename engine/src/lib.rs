pub mod api;
pub mod path;
pub mod point;
pub mod snake;
pub mod utils;

use api::Movement;
use api::{GameMode, GameState};

pub fn make_move(state: &api::GameState) -> Movement {
    match state.gamemode() {
        GameMode::Solo => solo(state),
        GameMode::Wrapped => wrapped(state),
        _ => Movement::default(),
    }
}

fn solo(state: &api::GameState) -> Movement {
    Movement::default()
}

fn wrapped(state: &api::GameState) -> Movement {
    Movement::default()
}
