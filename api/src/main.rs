// External Dependencies
#[macro_use]
extern crate actix_web;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

// Local Dependencies
mod snakeprops;
use battlesnake_engine::api::GameState;
use snakeprops::SnakeProps;

#[get("/")]
async fn index(data: web::Data<SnakeProps<'_>>) -> impl Responder {
	dbg!("Received GET /");
	HttpResponse::Ok().body(data.json())
}

#[post("/move")]
async fn domove(gamestate: web::Json<GameState>) -> impl Responder {
	dbg!("Received POST /move");
	dbg!(gamestate);
	HttpResponse::Ok()
}

#[post("/start")]
async fn start(gamestate: String) -> impl Responder {
	dbg!("Received POST /start");
	dbg!(gamestate);
	HttpResponse::Ok()
}

#[post("/end")]
async fn end(gamestate: String) -> impl Responder {
	dbg!("Received POST /end");
	dbg!(gamestate);
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
