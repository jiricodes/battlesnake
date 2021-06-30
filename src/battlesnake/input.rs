use super::point::Point;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct GameData {
    pub id: String,
    pub timeout: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Board {
    pub height: i32,
    pub width: i32,
    pub food: Vec<Point>,
    pub hazards: Vec<Point>,
    pub snakes: Vec<ApiSnake>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ApiSnake {
    pub id: String,
    pub name: String,
    pub health: i32,
    pub body: Vec<Point>,
    // latency: String,
    pub head: Point,
    pub length: i32,
    pub shout: String,
    // squad: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GameInfo {
    pub game: GameData,
    pub turn: i32,
    pub board: Board,
    pub you: ApiSnake,
}

impl GameInfo {
    pub fn new(data: &str) -> Self {
        // dbg!(&data);
        let new_object: GameInfo = serde_json::from_str(data).unwrap();
        // dbg!(&new_object);
        return new_object;
    }

    pub fn get_board_dimensions(&self) -> (usize, usize) {
        (self.board.height as usize, self.board.width as usize)
    }

    pub fn get_my_head(&self) -> Point {
        self.you.head
    }

    pub fn get_my_health(&self) -> i32 {
        self.you.health
    }

    pub fn get_my_index(&self) -> usize {
        for (i, s) in self.board.snakes.iter().enumerate() {
            if s.id == self.you.id {
                return i;
            }
        }
        return 101;
    }

    pub fn get_snake_bodies(&self) -> Vec<Vec<Point>> {
        let mut snakes_bodies: Vec<Vec<Point>> = Vec::new();
        // add snakes
        for snake in self.board.snakes.iter() {
            let to_add = snake.body.to_vec();
            snakes_bodies.push(to_add);
        }
        snakes_bodies
    }

    pub fn get_food(&self) -> Vec<Point> {
        self.board.food.clone()
    }

    pub fn get_hazards(&self) -> &Vec<Point> {
        &self.board.hazards
    }

    pub fn get_head_collision_hazard(&self) -> Vec<Point> {
        let mut result: Vec<Point> = Vec::new();
        for snake in self.board.snakes.iter() {
            if snake.length >= self.you.length && snake.id != self.you.id {
                let ngbs = snake.head.get_neighbours();
                for n in &ngbs {
                    if *n != snake.body[1] {
                        result.push(*n);
                    }
                }
            }
        }
        return result;
    }

    pub fn get_my_id(&self) -> &str {
        self.you.id.as_ref()
    }

    pub fn get_turn(&self) -> i32 {
        self.turn
    }

    pub fn get_game_id(&self) -> String {
        self.game.id.clone()
    }

    pub fn get_my_length(&self) -> i32 {
        self.you.length
    }

    pub fn get_heads(&self) -> Vec<Point> {
        let mut heads: Vec<Point> = Vec::new();
        for snake in self.board.snakes.iter() {
            heads.push(snake.head);
        }
        heads
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_input() {
        let data = r#"{
			"game":{
				"id":"00ca670e-d1e2-485f-9c39-17b531c9f377",
				"ruleset":{
					"name":"solo",
					"version":"v1.0.17"},
				"timeout":500},
				"turn":0,
				"board":{
					"height":7,
					"width":7,
					"snakes":[
						{
							"id":"gs_gvjGj8FFftxTwh4T9fhx498D",
							"name":"DefaultAWS",
							"latency":"",
							"health":100,
							"body":[
								{"x":5,"y":5},
								{"x":5,"y":5},
								{"x":5,"y":5}],
							"head":{"x":5,"y":5},
							"length":3,
							"shout":""
						}],
					"food":[
						{"x":6,"y":4},
						{"x":3,"y":3}],
					"hazards":[]},
				"you":{
					"id":"gs_gvjGj8FFftxTwh4T9fhx498D",
					"name":"DefaultAWS",
					"latency":"",
					"health":100,
					"body":[
						{"x":5,"y":5},
						{"x":5,"y":5},
						{"x":5,"y":5}],
					"head":{"x":5,"y":5},
					"length":3,
					"shout":""}
			}"#;
        let gameinfo = GameInfo::new(&data);
        dbg!(gameinfo);
    }
}
