// External Dependencies
#[macro_use]
extern crate actix_web;

use actix_web::{get, middleware::Logger, post, web, App, HttpResponse, HttpServer, Responder};
use env_logger;

// Local Dependencies
mod snakeprops;
use battlesnake_engine::api::{GameState, Movement};
use battlesnake_engine::make_move;
use snakeprops::SnakeProps;

#[get("/")]
async fn index(data: web::Data<SnakeProps<'_>>) -> impl Responder {
    dbg!("Received GET /");
    HttpResponse::Ok().body(data.json())
}

#[post("/move")]
async fn domove(gamestate: web::Json<GameState>) -> impl Responder {
    dbg!("Received POST /move");
    let movement = make_move(&gamestate);
    let response = match movement.json() {
        Ok(val) => val,
        Err(e) => {
            dbg!(e);
            Movement::default().json().unwrap()
        }
    };
    HttpResponse::Ok().body(response)
}

#[post("/start")]
async fn start(gamestate: web::Json<GameState>) -> impl Responder {
    dbg!("Received POST /start");
    dbg!(gamestate);
    HttpResponse::Ok()
}

#[post("/end")]
async fn end(gamestate: web::Json<GameState>) -> impl Responder {
    dbg!("Received POST /end");
    dbg!(gamestate);
    HttpResponse::Ok()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // toggle actix_logger
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("trace"));

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
            .wrap(Logger::default())
    })
    .bind("0.0.0.0:6969")?
    .run()
    .await
}
