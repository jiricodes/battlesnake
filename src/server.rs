//! Main server struct and loop
use crate::http::{ParseError, Request, Response, StatusCode};
use std::convert::TryFrom;
use std::io::Read;
use std::net::TcpListener;

/// Trait to handle requests
pub trait Handler {
    fn handle_request(&mut self, request: &Request) -> Response;

    fn handle_bad_request(&mut self, e: &ParseError) -> Response {
        println!("Failed to convert the request: {}", e);
        Response::new(
            StatusCode::BadRequest,
            Some("<h1>404 Bad Request</h1>".to_string()),
        )
    }
}

pub struct Server {
    address: String,
}

impl Server {
    pub fn new(address: String) -> Server {
        Server { address }
    }

    // takes ownership since we want to be dealocated upon exit
    pub fn run(self, mut handler: impl Handler) {
        let listener: TcpListener = TcpListener::bind(&self.address).unwrap();
        println!("Listening at {}", self.address);
        loop {
            match listener.accept() {
                Ok((mut stream, address)) => {
                    println!("Connection established with {}", address);
                    let mut buffer: [u8; 4096] = [0; 4096];
                    match stream.read(&mut buffer) {
                        Ok(size) => {
                            println!("Read {} bytes:", size);
                            // println!("{}", String::from_utf8_lossy(&buffer));

                            let response = match Request::try_from(&buffer as &[u8]) {
                                Ok(request) => handler.handle_request(&request),
                                Err(e) => handler.handle_bad_request(&e),
                            };
                            if let Err(e) = response.send(&mut stream) {
                                println!("Failed to send response: {}", e);
                            }
                        }
                        Err(e) => println!("Failed to read from connection: {}", e),
                    }
                }
                Err(e) => {
                    println!("Failed to establish a connection: {}", e);
                }
            }
            // let result = listener.accept();
            // if result.is_err() {
            // 	println!("Experienced connection error. Continuing.");
            // 	continue ;
            // }
            // let (stream, socket_address) = result.unwrap();
        }
    }
}
