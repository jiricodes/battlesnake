use serde::{Serialize, Deserialize};

// #[derive(Serialize, Deserialize, Debug)]
// struct Point {
// 	x: i32,
// 	y: i32,
// }

// #[derive(Serialize, Deserialize, Debug)]
// struct GameData {
// 	id: String,
// 	timeout: i32,
// }

// #[derive(Serialize, Deserialize, Debug)]
// struct Board {
// 	height: i32,
// 	width: i32,
// 	food: Vec<Point>,
// 	hazards: Vec<Point>,
// 	snakes: Vec<Snake>,
// }

// impl Board {

// }

// enum GridObjects {
// 	EMPTY,
// 	FOOD,
// 	SNAKE,
// 	HAZARD,
// 	OUTOFBOUND
// }

// struct GameGrid {
// 	height: usize,
// 	width: usize,
// 	data: Vec<GridObjects>,
// }

// impl GameGrid {
// 	pub fn new(height: usize, width: usize) -> Self {
// 		Self {
// 			height: height,
// 			width: width,
// 			data: vec![GridObjects::EMPTY; height * width],
// 		}
// 	}

// 	pub fn reset(&mut self) {
// 		self.data = vec![GridObjects::EMPTY; self.height * self.width];
// 	}

// 	fn get_index(&self, pos: &Point) -> Option<usize> {

// 	}

// 	fn is_in_bounds(&self, pos: &Point) -> bool {
// 		pos.x < self.width as i32 && pos.y < self.height as i32
// 	}
// }

// #[derive(Serialize, Deserialize, Debug)]
// struct Snake {
// 	id: String,
// 	name: String,
// 	health: i32,
// 	body: Vec<Point>,
// 	latency: i32,
// 	head: Point,
// 	length: i32,
// 	shout: String,
// 	squad: String
// }

// #[derive(Serialize, Deserialize, Debug)]
// struct GameInfo {
// 	game: GameData,
// 	turn: i32,
// 	board: Board,
// 	you: Snake,
// }

// impl GameInfo {
// 	pub fn new()
// }

// fn main() {
//     let snake_props = Snakeprops::new();
// 	let serialized = serde_json::to_string(&snake_props).unwrap();
// 	dbg!(serialized);

// 	let data = r#"{
// 					"game":{
// 						"id":"8e6bd531-d3d9-48f9-8687-b2ad665a567f",
// 						"timeout":500
// 					},
// 					"turn":2,
// 					"board":{
// 						"height":11,
// 						"width":11,
// 						"food":[
// 							{"x":8,"y":8},
// 							{"x":5,"y":5}
// 						],
// 						"hazards":[],
// 						"snakes":[
// 							{
// 								"id":"aca5a9fe-8cb9-4265-aedf-7f5d9ed02764",
// 								"name":"test",
// 								"health":99,
// 								"body":[
// 									{"x":9,"y":10},
// 									{"x":9,"y":9},
// 									{"x":9,"y":9}
// 								],
// 								"latency":0,
// 								"head":{"x":9,"y":10},
// 								"length":3,
// 								"shout":"",
// 								"squad":""
// 							},
// 							{
// 								"id":"aca5a9fe-8cb9-4265-aedf-7f5d9ed02764",
// 								"name":"otherone",
// 								"health":99,
// 								"body":[
// 									{"x":9,"y":10},
// 									{"x":9,"y":9},
// 									{"x":9,"y":9}
// 								],
// 								"latency":0,
// 								"head":{"x":9,"y":10},
// 								"length":3,
// 								"shout":"",
// 								"squad":""
// 							}
// 						]
// 					},
// 					"you":{
// 						"id":"aca5a9fe-8cb9-4265-aedf-7f5d9ed02764",
// 						"name":"test",
// 						"health":99,
// 						"body":[
// 							{"x":9,"y":10},
// 							{"x":9,"y":9},
// 							{"x":9,"y":9}
// 						],
// 						"latency":0,
// 						"head":{"x":9,"y":10},
// 						"length":3,
// 						"shout":"",
// 						"squad":""}
// 				}"#;
// 	let gamestate: GameState = serde_json::from_str(data).unwrap();
// 	dbg!(&gamestate);

// 	// println!("Board {:?}", deser["board"]);	
// }
