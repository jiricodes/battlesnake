enum Direction {
    Right,
    Left,
    Up,
    Down,
}

// implement iterator?
// random selection?

struct MCTSnode {
    state: GameState,
    wins:   usize,
    losses: usize,
    draws: usize,
    children: (Direction, Option(MCTSnode)) // if none, then unexplored
}