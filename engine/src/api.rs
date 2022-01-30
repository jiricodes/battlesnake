// External
use serde::{Deserialize, Serialize};

// Local
mod battlesnake;
pub use battlesnake::Battlesnake;

mod board;
use board::Board;

mod gamectx;
use gamectx::GameContext;

mod movement;
pub use movement::Movement;

use super::utils::status::{Error, Result};

#[derive(Serialize, Deserialize, Debug)]
pub struct GameState {
    game: GameContext,
    turn: u32,
    board: Board,
    you: Battlesnake,
}

impl GameState {
    pub fn from_json(json_data: &str) -> Result<Self> {
        match serde_json::from_str(json_data) {
            Ok(val) => Ok(val),
            Err(e) => Err(Error::from(e)),
        }
    }

    pub fn gamemode(&self) -> GameMode {
        self.game.gamemode()
    }
}

pub enum GameMode {
    Solo,
    Standard,
    Royale,
    Wrapped,
    Constrictor,
    Squad,
    Custom,
}

impl From<&str> for GameMode {
    fn from(f: &str) -> Self {
        match f {
            "solo" => Self::Solo,
            "standard" => Self::Standard,
            "royale" => Self::Royale,
            "wrapped" => Self::Wrapped,
            "constrictor" => Self::Constrictor,
            "squad" => Self::Squad,
            _ => Self::Custom,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn serialize() {
        let data = r###"{
            "game": {
                "id": "a418a0fd-c1b9-4772-9892-a64afcfff2f2",
                "ruleset": {
                    "name": "solo",
                    "version": "v1.0.25",
                    "settings": {
                        "foodSpawnChance": 15,
                        "minimumFood": 1,
                        "hazardDamagePerTurn": 0,
                        "royale": {
                            "shrinkEveryNTurns": 0
                        },
                        "squad": {
                            "allowBodyCollisions": false,
                            "sharedElimination": false,
                            "sharedHealth": false,
                            "sharedLength": false
                        }
                    }
                },
                "timeout": 500,
                "source": "custom"
            },
            "turn": 0,
            "board": {
                "height": 11,
                "width": 11,
                "snakes": [
                    {
                        "id": "gs_3QT6BxYxGbY3RyBJvtrmR6D7",
                        "name": "tester-replit",
                        "latency": "",
                        "health": 100,
                        "body": [
                            {
                                "x": 5,
                                "y": 9
                            },
                            {
                                "x": 5,
                                "y": 9
                            },
                            {
                                "x": 5,
                                "y": 9
                            }
                        ],
                        "head": {
                            "x": 5,
                            "y": 9
                        },
                        "length": 3,
                        "shout": "",
                        "squad": "",
                        "customizations": {
                            "color": "#888888",
                            "head": "villain",
                            "tail": "skinny-jeans"
                        }
                    }
                ],
                "food": [
                    {
                        "x": 4,
                        "y": 10
                    },
                    {
                        "x": 5,
                        "y": 5
                    }
                ],
                "hazards": []
            },
            "you": {
                "id": "gs_3QT6BxYxGbY3RyBJvtrmR6D7",
                "name": "tester-replit",
                "latency": "",
                "health": 100,
                "body": [
                    {
                        "x": 5,
                        "y": 9
                    },
                    {
                        "x": 5,
                        "y": 9
                    },
                    {
                        "x": 5,
                        "y": 9
                    }
                ],
                "head": {
                    "x": 5,
                    "y": 9
                },
                "length": 3,
                "shout": "",
                "squad": "",
                "customizations": {
                    "color": "#888888",
                    "head": "villain",
                    "tail": "skinny-jeans"
                }
            }
        }"###;
        let game = GameState::from_json(&data);
        dbg!(&game);
        assert!(game.is_ok());
    }
}
