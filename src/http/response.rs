use super::StatusCode;
use std::io::{Result as IoResult, Write};
use std::fmt;

#[derive(Debug)]
pub struct Response {
    status_code: StatusCode,
    body: Option<String>,
}

impl Response {
    pub fn new(status_code: StatusCode, body: Option<String>) -> Self {
        Self { status_code, body }
    }

    pub fn send(&self, stream: &mut impl Write) -> IoResult<()> {
        let body = match &self.body {
            Some(b) => b,
            None => "",
        };
        write!(
            stream,
            "HTTP/1.1 {} {}\r\n\r\n{}",
            self.status_code,
            self.status_code.reason_phrase(),
            body
        )
    }
}

impl fmt::Display for Response {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let body = match &self.body {
			Some(s) => String::from(s),
			None => String::from(""),
		};
        write!(f, "Code: {}\nBody: {}", self.status_code, body)
    }
}
