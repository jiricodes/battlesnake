use super::point::Point;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct GameData {
    id: String,
    timeout: i32,
}

#[derive(Serialize, Deserialize, Debug)]
struct Board {
    height: i32,
    width: i32,
    food: Vec<Point>,
    hazards: Vec<Point>,
    snakes: Vec<Snake>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Snake {
    id: String,
    name: String,
    health: i32,
    body: Vec<Point>,
    // latency: String,
    head: Point,
    length: i32,
    shout: String,
    // squad: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GameInfo {
    game: GameData,
    turn: i32,
    board: Board,
    you: Snake,
}

impl GameInfo {
    pub fn new(data: &str) -> Self {
        dbg!(&data);
        let new_object: GameInfo = serde_json::from_str(data).unwrap();
        return new_object;
    }

    pub fn get_board_dimensions(&self) -> (usize, usize) {
        (self.board.height as usize, self.board.width as usize)
    }

    pub fn get_my_head(&self) -> Point {
        self.you.head
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

    pub fn get_my_id(&self) -> &str {
        self.you.id.as_ref()
    }

    pub fn get_turn(&self) -> i32 {
        self.turn
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
