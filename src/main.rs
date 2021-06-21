#[macro_use]
extern crate actix_web;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

use std::io;
use std::time::{Duration, SystemTime};

use log::*;
// use log::Level::Debug;

mod battlesnake;
use battlesnake::init_logger;
use battlesnake::Move;
use battlesnake::SnakeProps;

// use std::time::Instant;

#[get("/")]
async fn index() -> impl Responder {
    debug!("Received Index");
    let snake = SnakeProps::new();
    // let datastr = snake.get_string();
    HttpResponse::Ok().body(snake.get_string())
}

#[post("/move")]
async fn domove(data: String) -> impl Responder {
    // debug!("Received Move");
    let start_time = SystemTime::now();
    let movement = Move::new(&data);
    let duration = SystemTime::now()
        .duration_since(start_time)
        .unwrap()
        .as_millis();
    info!("Handled /move [{}] in {}ms", movement.movement, duration);
    HttpResponse::Ok().body(movement.get_json_string())
}

#[post("/start")]
async fn start() -> impl Responder {
    debug!("Received START");
    HttpResponse::Ok()
}

#[post("/end")]
async fn end(data: String) -> impl Responder {
    debug!("Received END");
    dbg!(data);
    HttpResponse::Ok()
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    init_logger();
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(domove)
            .service(start)
            .service(end)
    })
    .bind("0.0.0.0:6970")?
    .run()
    .await
}
