use super::http::{Method, Request, Response, StatusCode};
use super::server::Handler;
use std::fs;

pub struct WebsiteHandler {
    public_path: String,
}

impl WebsiteHandler {
    pub fn new(public_path: String) -> Self {
        Self { public_path }
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
}

impl Handler for WebsiteHandler {
    fn handle_request(&mut self, request: &Request) -> Response {
        dbg!(request);

        match request.method() {
            Method::GET => match request.path() {
                "/" => Response::new(StatusCode::Ok, self.read_file("/index.html")),
                "/emojis" => Response::new(StatusCode::Ok, self.read_file("/emoji.html")),
                path => match self.read_file(&path) {
                    Some(body) => Response::new(StatusCode::Ok, Some(body)), //nonnoonononononononono Directory Traversal vulnerability - can read any file from the system!!!
                    None => Response::new(StatusCode::NotFound, None),
                },
            },
            _ => Response::new(StatusCode::NotFound, None),
        }
    }
}
