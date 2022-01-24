//! Module to handle API for Game Object.
//!
//! These structures are created per [BattleSnake API documentation](https://docs.battlesnake.com/references/api)

use super::super::utils::status::{Error, Result};
use serde::{Deserialize, Serialize};

/// Wrapper for royale.shrinkEveryNTurns
#[derive(Serialize, Deserialize, Debug)]
struct RoyaleSettings {
    /// In Royale mode, the number of turns between generating new hazards (shrinking the safe board space).
    #[serde(rename = "shrinkEveryNTurns")]
    shrink_n: i32,
}

/// Wrapper for squad settings
#[derive(Serialize, Deserialize, Debug)]
struct SquadSettings {
    /// In Squad mode, allow members of the same squad to move over each other without dying.
    #[serde(rename = "allowBodyCollisions")]
    allow_collision: bool,
    /// In Squad mode, all squad members are eliminated when one is eliminated.
    #[serde(rename = "sharedElimination")]
    shared_elimination: bool,
    /// In Squad mode, all squad members share health.
    #[serde(rename = "sharedHealth")]
    shared_health: bool,
    /// In Squad mode, all squad members share length.
    #[serde(rename = "sharedLength")]
    shared_length: bool,
}

/// Various game type specific settings. [Details](https://docs.battlesnake.com/references/api#rulesetsettings).
#[derive(Serialize, Deserialize, Debug)]
struct RulesetSettings {
    /// Percentage chance of spawning a new food every round.
    #[serde(rename = "foodSpawnChance")]
    food_spawn_chance: u32,
    /// Minimum food to keep on the board every turn
    #[serde(rename = "minimumFood")]
    min_food: u32,
    /// Health damage a snake will take when ending its turn in a hazard. This stacks on top of the regular 1 damage a snake takes per turn.
    #[serde(rename = "hazardDamagePerTurn")]
    hazard_dmg: u32,
    /// Royale settings
    royale: RoyaleSettings,
    /// Squad settings
    squad: SquadSettings,
}

/// Ruleset struct. [Details](https://docs.battlesnake.com/references/api#ruleset).
#[derive(Serialize, Deserialize, Debug)]
struct RuleSet {
    /// Name of the ruleset, e.g. solo, royale
    name: String,
    /// The release version of the [Rules](https://github.com/BattlesnakeOfficial/rules) module used in this game.
    version: String,
    /// A collection of specific settings being used by the current game that control how the rules are applied.
    settings: Option<RulesetSettings>,
}

/// Game Object. [Details](https://docs.battlesnake.com/references/api#game).
#[derive(Serialize, Deserialize, Debug)]
pub struct GameContext {
    /// A unique identifier for this Game.
    id: String,
    /// Information about the ruleset being used to run this game.
    ruleset: RuleSet,
    /// How much time (milliseconds) your snake has to respond to requests for this Game.
    timeout: u32,
    /// The source of this game, e.g. "league" or "custom".
    source: String,
}

impl GameContext {
    pub fn from_json(json_data: &str) -> Result<Self> {
        match serde_json::from_str(json_data) {
            Ok(val) => Ok(val),
            Err(e) => Err(Error::from(e)),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn gamecontext_fails() {
        let data = "fail";
        assert!(GameContext::from_json(&data).is_err());
    }
}
