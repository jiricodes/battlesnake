#![allow(dead_code)]

use std::{env, fs, path::Path};

mod server;
use server::Server;

mod website_handler;
use website_handler::WebsiteHandler;

mod http;

use clap::{App, Arg};

fn main() {
    let arguments = App::new("http_server")
        .arg(
            Arg::with_name("default_path")
                .short("d")
                .long("path")
                .takes_value(true)
                .help("Path to public directory"),
        )
        .arg(
            Arg::with_name("ip_address")
                .short("ip")
                .long("ip-address")
                .takes_value(true)
                .help("Listen IP address"),
        )
        .arg(
            Arg::with_name("port")
                .short("p")
                .long("port")
                .takes_value(true)
                .help("Listen port"),
        )
        .get_matches();
    // set default path - at compile time use Cargo.toml location + /public
    let default_path = format!("{}/public", env!("CARGO_MANIFEST_DIR"));
    let public_path = if arguments.is_present("default_path") {
        arguments.value_of("default_path").unwrap().to_string()
    } else {
        env::var("PUBLIC_PATH").unwrap_or(default_path)
    };
    dbg!(&public_path);
    if !Path::new(&public_path).exists() {
        eprintln!("Given path doesn't exist");
        return;
    }
    let absolute_path = fs::canonicalize(&public_path)
        .unwrap()
        .into_os_string()
        .into_string()
        .unwrap();
    dbg!(&absolute_path);
    let ip = arguments.value_of("ip_address").unwrap_or("127.0.0.1");
    let port = arguments.value_of("port").unwrap_or("6969");
    // Initialize Server
    let server: Server = Server::new(format!("{}:{}", ip, port));
    // Run server
    server.run(WebsiteHandler::new(absolute_path));
}
