//! Module to handle API for Game Object.
//! 
//! These structures are created per [BattleSnake API documentation](https://docs.battlesnake.com/references/api)

/// Wrapper for royale.shrinkEveryNTurns
struct RoyaleSettings {
    /// In Royale mode, the number of turns between generating new hazards (shrinking the safe board space).
    shrink_n: i32,
}

/// Wrapper for squad settings
struct SquadSettings {
    /// In Squad mode, allow members of the same squad to move over each other without dying.
    allow_collision: bool,
    /// In Squad mode, all squad members are eliminated when one is eliminated.
    shared_elimination: bool,
    /// In Squad mode, all squad members share health.
    shared_health: bool,
    /// In Squad mode, all squad members share length.
    shared_length: bool
}
/// Various game type specific settings. [Details](https://docs.battlesnake.com/references/api#rulesetsettings).
struct RulesetSettings {
    /// Percentage chance of spawning a new food every round.
    food_spawn_chance: i32,
    /// Minimum food to keep on the board every turn
    min_food: i32,
    /// Health damage a snake will take when ending its turn in a hazard. This stacks on top of the regular 1 damage a snake takes per turn.
    hazard_dmg: i32,
    /// Royale settings
    royale: RoyaleSettings,
    /// Squad settings
    squad: SquadSettings,

}

/// Ruleset struct. [Details](https://docs.battlesnake.com/references/api#ruleset).
struct RuleSet {
    /// Name of the ruleset, e.g. solo, royale
    name: String,
    /// The release version of the [Rules](https://github.com/BattlesnakeOfficial/rules) module used in this game.
    version: String
    /// A collection of specific settings being used by the current game that control how the rules are applied.
    settings: Option<RulesetSettings>
}

/// Game Object. [Details](https://docs.battlesnake.com/references/api#game).
pub struct GameContext {
    /// A unique identifier for this Game.
    id: String,
    /// Information about the ruleset being used to run this game.
    ruleset: RuleSet,
    /// How much time (milliseconds) your snake has to respond to requests for this Game.
    timeout: i32,
    /// The source of this game, e.g. "league" or "custom".
    source: String,
}