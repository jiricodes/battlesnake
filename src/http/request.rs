use super::method::{Method, MethodError};
use super::QueryString;
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::fmt;
use std::str;
use std::str::Utf8Error;

fn get_next_word(request: &str) -> Option<(&str, &str)> {
    for (i, c) in request.chars().enumerate() {
        if c == ' ' || c == '\r' {
            return Some((&request[..i], &request[i + 1..])); // safe since we know it's a space
        }
    }

    None
}

fn get_body(request: &str) -> &str {
	let mut l: usize = 0;
	for line in request.lines() {
		match line.find("Content-Length:") {
			Some(_) => {
				let input: Vec<&str> = line.split(' ').collect();
				l = input[1].parse().unwrap();
				// dbg!(&input);
				// dbg!(&l);
				break;
			},
			None => { continue; },
		}
	}
	for (i, c) in request.chars().enumerate() {
        if c == '{' {
            return &request[i..i+l]; // safe since we know it's a space
        }
    }
    ""
}

#[derive(Debug)]
pub struct Request<'a> {
    path: &'a str,
    query: Option<QueryString<'a>>,
    method: Method,
	body: &'a str,
}

impl<'a> Request<'a> {
    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn method(&self) -> &Method {
        &self.method
    }

    pub fn query(&self) -> Option<&QueryString> {
        self.query.as_ref()
    }

	pub fn body(&self) -> &str {
		&self.body
	}
}

impl<'a> Display for Request<'a> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.method, self.path)
    }
}

impl<'a> TryFrom<&'a [u8]> for Request<'a> {
    type Error = ParseError;

    fn try_from(buffer: &'a [u8]) -> Result<Self, Self::Error> {
        let request = str::from_utf8(buffer)?;

        //variable shadowing == reuse of variables
        let (method, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (mut path, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (protocol, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;

        if protocol != "HTTP/1.1" {
            return Err(ParseError::InvalidProtocol);
        }

        let method: Method = method.parse()?;

        let mut query_string = None;

        if let Some(i) = path.find('?') {
            query_string = Some(QueryString::from(&path[i + 1..]));
            path = &path[..i];
        }

		let body = get_body(request);
		// dbg!(body);

        Ok(Self {
            path,
            query: query_string,
            method,
			body: body,
        })
    }
}

pub enum ParseError {
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod,
}

impl ParseError {
    fn message(&self) -> &str {
        match self {
            Self::InvalidRequest => "Invalid Request",
            Self::InvalidEncoding => "Invalid Encoding - not utf8",
            Self::InvalidProtocol => "Invalid Protocol - Only HTTP/1.1 supported",
            Self::InvalidMethod => "Invalid Method - not existing or not implemented",
        }
    }
}

impl From<MethodError> for ParseError {
    fn from(_: MethodError) -> Self {
        Self::InvalidMethod
    }
}

impl From<Utf8Error> for ParseError {
    fn from(_: Utf8Error) -> Self {
        Self::InvalidEncoding
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Error for ParseError {}
