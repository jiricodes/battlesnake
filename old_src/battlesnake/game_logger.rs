//! Tool for logging games
//!
//! TO-DO
//! list of currently ongoing games
//! separate file for each game
//! .input.log -> logs input data
//! .custom.log -> should be utilized for dbg etc...
//!
use chrono::{DateTime, Local};
use colored::{ColoredString, Colorize};
use log::*;

use std::fmt;
use std::collections::{HashMap};
use std::time::{Duration, SystemTime};

use super::GameInfo;
use super::Point;

const LOGGER: Logger = Logger;

struct Logger;
impl log::Log for Logger {
    fn enabled(&self, _metadata: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        let now: DateTime<Local> = Local::now();
        println!(
            "{} [{}] {}",
            now.format("%Y-%m-%d %H:%M:%S%.6f"),
            record.level(),
            record.args()
        );
    }

    fn flush(&self) {}
}

pub fn init_logger() {
    if set_logger(&LOGGER).is_ok() {
        log::set_max_level(LevelFilter::Debug);
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum GridObjectLog {
    Empty,
    Food,
    Snake(usize),
}

impl fmt::Display for GridObjectLog {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let symbol = match self {
            GridObjectLog::Empty => "◦",
            GridObjectLog::Food => "⚕",
            GridObjectLog::Snake(n) => match n {
                0 => "■",
                1 => "⌀",
                2 => "●",
                3 => "⍟",
                4 => "◘",
                5 => "☺",
                6 => "□",
                7 => "☻",
                _ => "S",
            },
        };
        write!(f, "{}", symbol)
    }
}

pub struct GameGridLog {
    height: usize,
    width: usize,
    data: Vec<GridObjectLog>,
    snake_heads: Vec<usize>,
    hazards: Vec<usize>,
}

impl GameGridLog {
    fn new(dimensions: (usize, usize)) -> Self {
        Self {
            height: dimensions.0,
            width: dimensions.1,
            data: vec![GridObjectLog::Empty; dimensions.0 * dimensions.1],
            snake_heads: Vec::new(),
            hazards: Vec::new(),
        }
    }

    pub fn from_api(gameinfo: &GameInfo) -> Self {
        let dim = gameinfo.get_board_dimensions();
        let mut grid = GameGridLog::new(dim);
        grid.set_snakes(gameinfo.get_snake_bodies());
        grid.set_food(&gameinfo.get_food());
        let hazards = gameinfo.get_hazards();
        for h in hazards {
            if let Some(i) = grid.get_index(h) {
                grid.hazards.push(i);
            }
        }
        let snake_heads = gameinfo.get_heads();
        for h in snake_heads.iter() {
            if let Some(i) = grid.get_index(h) {
                grid.snake_heads.push(i);
            }
        }
        grid
    }

    fn is_in_bounds(&self, pos: &Point) -> bool {
        0 <= pos.get_x()
            && pos.get_x() < self.width as i32
            && 0 <= pos.get_y()
            && pos.get_y() < self.height as i32
    }

    fn get_index(&self, pos: &Point) -> Option<usize> {
        if !self.is_in_bounds(&pos) {
            None
        } else {
            Some(pos.get_y() as usize * self.width + pos.get_x() as usize)
        }
    }

    fn set_snakes(&mut self, snakes: Vec<Vec<Point>>) {
        for (p, snake) in snakes.iter().enumerate() {
            for point in snake.iter() {
                match self.get_index(&point) {
                    Some(i) => {
                        self.data[i] = GridObjectLog::Snake(p);
                    }
                    None => {
                        continue;
                    }
                }
            }
        }
    }

    fn set_food(&mut self, food: &Vec<Point>) {
        for p in food {
            match self.get_index(&p) {
                Some(i) => {
                    if self.data[i] == GridObjectLog::Empty {
                        self.data[i] = GridObjectLog::Food;
                    }
                }
                None => {
                    continue;
                }
            }
        }
    }

    pub fn print(&self) {
        let l = self.data.len() - 1;
        let mut line: Vec<ColoredString> = Vec::new();
        for (i, cell) in self.data.iter().rev().enumerate() {
            let mut val = if self.snake_heads.contains(&(l - i)) {
                format!("{}", cell).green()
            } else {
                format!("{}", cell).white()
            };
            if self.hazards.contains(&(l - i)) {
                val = val.on_color("red");
            }
            line.push(val);
            if (i + 1) % self.width == 0 {
                for c in line.iter().rev() {
                    print!("{}", c);
                }
                println!();
                line.clear();
            }
        }
    }
}

pub struct GameStateLog {
    grid: GameGridLog,
    snake_legend: Vec<(usize, String, i32)>,
    game_legend: i32,
}

impl GameStateLog {
    pub fn from_api(data: &GameInfo) -> Self {
        Self {
            grid: GameGridLog::from_api(data),
            snake_legend: data
                .board
                .snakes
                .iter()
                .enumerate()
                .map(|(i, snake)| return (i, snake.name.clone(), snake.health))
                .collect(),
            game_legend: data.get_turn(),
        }
    }

    pub fn print(&self) {
        println!("Turn {}", self.game_legend);
        for snake in self.snake_legend.iter() {
            println!(
                "[ {:3} | {} ] {}",
                snake.2,
                GridObjectLog::Snake(snake.0),
                snake.1
            );
        }
        self.grid.print();
        println!();
    }
}

pub struct SessionStats {
    games: usize,
    wins: usize,
    total_turns: usize,
    running_games: HashMap<String, (usize, SystemTime)>,
    timeout: Duration,
}

impl SessionStats {
    /// timeout in seconds
    pub fn new(timeout: u64) -> Self {
        Self {
            games: 0,
            wins: 0,
            total_turns: 0,
            running_games: HashMap::new(),
            timeout: Duration::from_secs(timeout),
        }
    }

    pub fn start_game(&mut self, id: String) {
        self.running_games.insert(id, (0, SystemTime::now()));
    }

    pub fn update_game(&mut self, id: &String, turns: usize) {
        if let Some(game) = self.running_games.get_mut(id) {
            (*game).0 = turns;
        }
    }

    pub fn end_game(&mut self, id: &String, win: bool) {
        if let Some(game) = self.running_games.get(id) {
            self.total_turns += game.0;
            self.games += 1;
            if win {
                self.wins += 1;
            }
            self.running_games.remove(id);
        }  
    }

    pub fn garbage_collect(&mut self) {
        let mut to_end: Vec<String> = Vec::new();
        let now = SystemTime::now();
        for (key, (_, start)) in self.running_games.iter() {
            if now.duration_since(*start).unwrap_or(Duration::from_millis(1)) > self.timeout {
                to_end.push((*key).clone());
            }
        }
        while !to_end.is_empty() {
            self.end_game(&to_end.pop().unwrap(), false);
        }
    }

    pub fn get_running_len(&self) -> usize {
        self.running_games.len()
    }
    
    pub fn set_timeout(&mut self, new_timeout: Duration) {
        self.timeout = new_timeout;
    }

    pub fn get_timeout(&mut self) -> u64 {
        self.timeout.as_secs()
    }
}

impl fmt::Display for SessionStats {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let games = if self.games != 0 {
            self.games
        } else {
            1
        };
        write!(f, "Games {}, Wins {}, Win ratio {:.2}%, Average Turns {}. In progress {} games.",
        self.games, self.wins, (self.wins as f32 / games as f32) * 100.00, self.total_turns / games, self.get_running_len())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::thread;

    #[test]
    fn print() {
        let gameinfo = GameInfo::new(
            r#"{
            "game": {
                "id": "2c2d43ec-0fdb-4bf4-9a00-8f1d243238d4",
                "ruleset": {
                    "name": "royale",
                    "version": ""
                },
                "timeout": 500
            },
            "turn": 32,
            "board": {
                "width": 11,
                "height": 11,
                "snakes": [
                    {
                        "id": "gs_9Xdgwh9wPKktBtXphrMt4d67",
                        "name": "Eel In Snake's Clothing",
                        "body": [
                            {
                                "x": 3,
                                "y": 7
                            },
                            {
                                "x": 4,
                                "y": 7
                            },
                            {
                                "x": 4,
                                "y": 8
                            },
                            {
                                "x": 5,
                                "y": 8
                            },
                            {
                                "x": 6,
                                "y": 8
                            }
                        ],
                        "head": {
                            "x": 3,
                            "y": 7
                        },
                        "length": 5,
                        "health": 88,
                        "shout": "",
                        "squad": ""
                    },
                    {
                        "id": "gs_fhDCrB9BmRfgWBgXj9cBCxqR",
                        "name": "Go  Giddy",
                        "body": [
                            {
                                "x": 9,
                                "y": 3
                            },
                            {
                                "x": 9,
                                "y": 4
                            },
                            {
                                "x": 9,
                                "y": 5
                            },
                            {
                                "x": 8,
                                "y": 5
                            },
                            {
                                "x": 7,
                                "y": 5
                            },
                            {
                                "x": 7,
                                "y": 6
                            },
                            {
                                "x": 7,
                                "y": 6
                            }
                        ],
                        "head": {
                            "x": 9,
                            "y": 3
                        },
                        "length": 7,
                        "health": 100,
                        "shout": "",
                        "squad": ""
                    },
                    {
                        "id": "gs_KTTWwyytWTBjgXkyh8gDp9VD",
                        "name": "Untimely Neglected Wearable",
                        "body": [
                            {
                                "x": 7,
                                "y": 3
                            },
                            {
                                "x": 7,
                                "y": 2
                            },
                            {
                                "x": 8,
                                "y": 2
                            },
                            {
                                "x": 9,
                                "y": 2
                            },
                            {
                                "x": 9,
                                "y": 1
                            },
                            {
                                "x": 8,
                                "y": 1
                            },
                            {
                                "x": 7,
                                "y": 1
                            }
                        ],
                        "head": {
                            "x": 7,
                            "y": 3
                        },
                        "length": 7,
                        "health": 96,
                        "shout": "",
                        "squad": ""
                    },
                    {
                        "id": "gs_8fMbQg9DHB9fMGxR7Hv39P9Q",
                        "name": "Danger Noodle - A*/Flood",
                        "body": [
                            {
                                "x": 6,
                                "y": 4
                            },
                            {
                                "x": 6,
                                "y": 3
                            },
                            {
                                "x": 6,
                                "y": 2
                            },
                            {
                                "x": 5,
                                "y": 2
                            },
                            {
                                "x": 4,
                                "y": 2
                            }
                        ],
                        "head": {
                            "x": 6,
                            "y": 4
                        },
                        "length": 5,
                        "health": 80,
                        "shout": "",
                        "squad": ""
                    }
                ],
                "food": [
                    {
                        "x": 10,
                        "y": 6
                    }
                ],
                "hazards": [
                    {
                        "x": 10,
                        "y": 0
                    },
                    {
                        "x": 10,
                        "y": 1
                    },
                    {
                        "x": 10,
                        "y": 2
                    },
                    {
                        "x": 10,
                        "y": 3
                    },
                    {
                        "x": 10,
                        "y": 4
                    },
                    {
                        "x": 10,
                        "y": 5
                    },
                    {
                        "x": 10,
                        "y": 6
                    },
                    {
                        "x": 10,
                        "y": 7
                    },
                    {
                        "x": 10,
                        "y": 8
                    },
                    {
                        "x": 10,
                        "y": 9
                    },
                    {
                        "x": 10,
                        "y": 10
                    }
                ]
            },
            "you": {
                "id": "gs_fhDCrB9BmRfgWBgXj9cBCxqR",
                "name": "Go  Giddy",
                "body": [
                    {
                        "x": 9,
                        "y": 3
                    },
                    {
                        "x": 9,
                        "y": 4
                    },
                    {
                        "x": 9,
                        "y": 5
                    },
                    {
                        "x": 8,
                        "y": 5
                    },
                    {
                        "x": 7,
                        "y": 5
                    },
                    {
                        "x": 7,
                        "y": 6
                    },
                    {
                        "x": 7,
                        "y": 6
                    }
                ],
                "head": {
                    "x": 9,
                    "y": 3
                },
                "length": 7,
                "health": 100,
                "shout": "",
                "squad": ""
            }
        }"#,
        );
        let game = GameStateLog::from_api(&gameinfo);
        game.print();
    }

    #[test]
    fn session_basic() {
        let mut session = SessionStats::new(1);
        session.start_game("2c2d43ec-0fdb-4bf4-9a00-8f1d243238d4".to_string());
        session.start_game("2c2d43ec-0fdb-4bf4-9a00-8f1d243238d5".to_string());
        session.start_game("2c2d43ec-0fdb-4bf4-9a00-8f1d243238d6".to_string());

        session.update_game(&"2c2d43ec-0fdb-4bf4-9a00-8f1d243238d4".to_string(), 70);
        session.update_game(&"2c2d43ec-0fdb-4bf4-9a00-8f1d243238d5".to_string(), 120);
        session.update_game(&"2c2d43ec-0fdb-4bf4-9a00-8f1d243238d6".to_string(), 80);
        assert_eq!(session.get_running_len(), 3);
        session.end_game(&"2c2d43ec-0fdb-4bf4-9a00-8f1d243238d6".to_string(), false);
        assert_eq!(session.get_running_len(), 2);
        assert_eq!(session.games, 1);
        assert_eq!(session.total_turns, 80);
        session.end_game(&"2c2d43ec-0fdb-4bf4-9a00-8f1d243238d4".to_string(), true);
        assert_eq!(session.get_running_len(), 1);
        assert_eq!(session.games, 2);
        assert_eq!(session.total_turns, 150);
        thread::sleep(Duration::from_secs(2));
        session.start_game("2c2d43ec-0fdb-4bf4-9a00-8f1d243238d7".to_string());
        assert_eq!(session.get_running_len(), 2);
        session.garbage_collect();
        assert_eq!(session.get_running_len(), 1);
        println!("{}", session);
    }
}
