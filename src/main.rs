#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
use rocket::response::content::Json;
use rocket::http::Status;

mod battlesnake;
use battlesnake::SnakeProps;
use battlesnake::Move;

use std::time::Instant;

#[get("/")]
fn index() -> Json<String> {
	println!("\nReceived Index");
	let snake = SnakeProps::new();
	// let datastr = snake.get_string();
	Json(snake.get_string())
}

#[post("/move", data = "<data>")]
fn domove(data: String) -> Json<String> {
	let start = Instant::now();
	println!("\nReceived Move");
	let movement = Move::new(&data);
	println!("Move: {}", &movement);
	println!("----\nAsnwered in {}\n---\n", start.elapsed().as_millis());
	Json(movement.get_json_string())
}

#[post("/start")]
fn start() -> Status {
	println!("Received START");
	Status::Ok
}

#[post("/end")]
fn end() -> Status {
	println!("Received END");
	Status::Ok
}

fn main() {
	rocket::ignite().mount("/", routes![index, start, end, domove]).launch();
}