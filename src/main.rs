#[macro_use]
extern crate actix_web;
#[macro_use]
extern crate clap;
#[macro_use]
extern crate lazy_static;

// Depts
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use clap::{App as ClApp, Arg as ClArg};
use log::*;

// Std
use std::io;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Mutex;
use std::time::{Duration, SystemTime};

// Battlesnake
mod battlesnake;
use battlesnake::get_move;
use battlesnake::init_logger;
use battlesnake::Board;
use battlesnake::GameInfo;
use battlesnake::SessionStats;
use battlesnake::SnakeProps;

// Vars
static TIME_BUDGET: AtomicU64 = AtomicU64::new(280);
// lazy_static! {
//     static ref SESSION_STATS: Mutex<SessionStats> = Mutex::new(SessionStats::new(1200));
// }

#[get("/")]
async fn index() -> impl Responder {
    debug!("Received Index");
    let snake = SnakeProps::new();
    // let datastr = snake.get_string();
    HttpResponse::Ok().body(snake.get_string())
}

// #[get("/stats")]
// async fn get_stats() -> impl Responder {
//     debug!("Received GET /stats");
//     // let session_stats = SESSION_STATS.lock().unwrap();
//     HttpResponse::Ok().body(session_stats.get_string())
// }

#[post("/move")]
async fn domove(data: String) -> impl Responder {
    let start_time = SystemTime::now();
    let game_data = GameInfo::new(&data);
    // let mut session_stats = SESSION_STATS.lock().unwrap();
    // session_stats.update_game(&game_data.get_game_id(), game_data.get_turn() as usize);
    let movement = get_move(
        &game_data,
        Duration::from_millis(TIME_BUDGET.load(Ordering::SeqCst)),
    );
    let duration = SystemTime::now()
        .duration_since(start_time)
        .unwrap()
        .as_millis();
    info!("Handled /move at turn {} [{}] in {}ms", game_data.get_turn(), movement.movement, duration);
    HttpResponse::Ok().body(movement.get_json_string())
}

#[post("/start")]
async fn start(data: String) -> impl Responder {
    debug!("Received START");
    let game_data = GameInfo::new(&data);
    // let mut session_stats = SESSION_STATS.lock().unwrap();
    // session_stats.garbage_collect();
    // session_stats.start_game(game_data.get_game_id());
    // debug!("{}", session_stats);
    HttpResponse::Ok()
}

#[post("/end")]
async fn end(data: String) -> impl Responder {
    debug!("Received END");
    let game_data = GameInfo::new(&data);
    let result = Board::from_api(&game_data).get_my_death();
    let mut win = false;
    if result.is_some() {
        info!("Death by {:?}", result.unwrap());
    } else {
        info!("Victory!");
        win = true;
    }
    // let mut session_stats = SESSION_STATS.lock().unwrap();
    // session_stats.end_game(&game_data.get_game_id(), win);
    // session_stats.garbage_collect();
    // debug!("{}", session_stats);
    HttpResponse::Ok()
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    // Initialize logger
    init_logger();

    // Process arguments
    let arguments = ClApp::new("battlesnake")
    .author("hello@jiricodes.com")
    .version(crate_version!())
    .arg(
        ClArg::with_name("ip_address")
        .short("i")
                .long("ip-address")
                .takes_value(true)
                .help("Listen IP address"),
    ).arg(
            ClArg::with_name("port")
                .short("p")
                .long("port")
                .takes_value(true)
                .help("Listen port"),
    ).arg(
        ClArg::with_name("time_budget").short("t")
        .long("time-budget")
        .takes_value(true)
        .help("Time Budget for the algorithm")
    ).arg(
        ClArg::with_name("stats_game_timeout")
        .short("s")
        .long("stats-game-timeout")
        .takes_value(true)
        .help("Sets timeout in seconds for session statistics' games. This is required due to battlesnake API invoking game end when not winning snake. Default 1200s (20 minutes)")
    ).get_matches();

    // Set Time Budget if argument passed
    if let Ok(time_budget) = value_t!(arguments, "time_budget", u64) {
        TIME_BUDGET.store(time_budget as u64, Ordering::SeqCst);
        info!(
            "Time budget set to {} ms.",
            TIME_BUDGET.load(Ordering::SeqCst)
        );
    }
    // Set Stats timeout
    // if let Ok(stats_timeout) = value_t!(arguments, "stats_game_timeout", u64) {
    //     let mut session_stats = SESSION_STATS.lock().unwrap();
    //     session_stats.set_timeout(Duration::from_secs(stats_timeout));
    //     info!(
    //         "Stats Timeout set to {} seconds",
    //         session_stats.get_timeout()
    //     );
    //     std::mem::drop(session_stats);
    // }

    // Prep IP and Port
    let ip_address = arguments.value_of("ip_address").unwrap_or("0.0.0.0");
    let port = arguments.value_of("port").unwrap_or("6969");
    let address = format!("{}:{}", ip_address, port);
    // Start the HTTP server
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(domove)
            .service(start)
            .service(end)
            //.service(get_stats)
    })
    .bind(&address)?
    .run()
    .await
}
