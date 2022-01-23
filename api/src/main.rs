// External Dependencies
#[macro_use]
extern crate actix_web;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

// Std
use std::sync::RwLock;

// Local Dependencies
mod snakeprops;
use snakeprops::SnakeProps;

#[get("/")]
async fn index(data: web::Data<SnakeProps<'_>>) -> impl Responder {
	dbg!("Received GET /");
	HttpResponse::Ok().body(data.json())
}

#[post("/move")]
async fn domove(data: String) -> impl Responder {
	dbg!("Received POST /move");
	HttpResponse::Ok()
}

#[post("/start")]
async fn start(data: String) -> impl Responder {
	dbg!("Received POST /start");
	HttpResponse::Ok()
}

#[post("/end")]
async fn end(data: String) -> impl Responder {
	dbg!("Received POST /end");
	HttpResponse::Ok()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
	// Change Snake Properties
	let mut snakeprops = SnakeProps::default();
	snakeprops.set_color("#622BAA");
	snakeprops.set_head("villain");
	snakeprops.set_tail("skinny-jeans");
	// Start the HTTP server
	HttpServer::new(move || {
		App::new()
			.data(snakeprops)
			.service(index)
			.service(domove)
			.service(start)
			.service(end)
	})
	.bind("0.0.0.0:6969")?
	.run()
	.await
}
