use super::http::{Method, Request, Response, StatusCode};
use super::server::Handler;
use super::battlesnake::Snake;
use std::fs;

pub struct RequestHandler {
    public_path: String,
	snake: Snake,
}

impl RequestHandler {
    pub fn new(public_path: String) -> Self {
        Self { 
				public_path: public_path,
				snake: Snake::new() }
    }

    fn read_file(&self, file_path: &str) -> Option<String> {
        let path = format!("{}{}", self.public_path, file_path);
        match fs::canonicalize(path) {
            Ok(path) => {
                if path.starts_with(&self.public_path) {
                    dbg!(&path);
                    fs::read_to_string(path).ok()
                } else {
                    println!("Warning! Attempting Directory Traversal:");
                    dbg!(&path);
                    None
                }
            }
            Err(_) => None,
        }
    }

	fn get_snake_params(&self) -> Option<String> {
		Some(self.snake.get_string())
	}
}

impl Handler for RequestHandler {
    fn handle_request(&mut self, request: &Request) -> Response {
        dbg!(request);

        match request.method() {
            Method::GET => match request.path() {
                "/" => Response::new(StatusCode::Ok, self.get_snake_params()),
                "/emojis" => Response::new(StatusCode::Ok, self.read_file("/emoji.html")),
                path => match self.read_file(&path) {
                    Some(body) => Response::new(StatusCode::Ok, Some(body)), //nonnoonononononononono Directory Traversal vulnerability - can read any file from the system!!!
                    None => Response::new(StatusCode::NotFound, None),
                },
            },
			Method::POST => match request.path() {
				"/start" => Response::new(StatusCode::Ok, None),
				"/move" => Response::new(StatusCode::Ok, Some(String::from(r#"{"move": "up", "shout" : "test"}"#))),
				"/end" => Response::new(StatusCode::Ok, None),
				_ => Response::new(StatusCode::NotFound, None),
			},
            _ => Response::new(StatusCode::NotFound, None),
        }
    }
}
