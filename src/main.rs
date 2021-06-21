#[macro_use]
extern crate actix_web;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

use std::io;

mod battlesnake;
use battlesnake::Move;
use battlesnake::SnakeProps;

// use std::time::Instant;

#[get("/")]
async fn index() -> impl Responder {
    println!("\nReceived Index");
    let snake = SnakeProps::new();
    // let datastr = snake.get_string();
    HttpResponse::Ok().body(snake.get_string())
}

#[post("/move")]
async fn domove(data: String) -> impl Responder {
    // let start: Instant = Instant::now();
    println!("\nReceived Move");
    let movement = Move::new(&data);
    // println!("Move: {}", &movement);
    // println!("----\nAsnwered in {}\n---\n", start.elapsed().as_millis());
    HttpResponse::Ok().body(movement.get_json_string())
}

#[post("/start")]
async fn start() -> impl Responder {
    println!("Received START");
    HttpResponse::Ok()
}

#[post("/end")]
async fn end(data: String) -> impl Responder {
    println!("Received END");
    dbg!(data);
    HttpResponse::Ok()
}

#[actix_web::main]
async fn main() -> io::Result<()> {
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
