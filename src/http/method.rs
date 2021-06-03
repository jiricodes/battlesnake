use std::str::FromStr;
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum Method {
    GET,
    DELETE,
    POST,
    PUT,
    HEAD,
    CONNECT,
    OPTIONS,
    TRACE,
    PATCH,
}

impl FromStr for Method {
    type Err = MethodError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "GET" => Ok(Self::GET),
            "DELETE" => Ok(Self::DELETE),
            "POST" => Ok(Self::POST),
            "PUT" => Ok(Self::PUT),
            "HEAD" => Ok(Self::HEAD),
            "CONNECT" => Ok(Self::CONNECT),
            "OPTIONS" => Ok(Self::OPTIONS),
            "TRACE" => Ok(Self::TRACE),
            "PATCH" => Ok(Self::PATCH),
            _ => Err(MethodError),
        }
    }
}

pub struct MethodError;

impl fmt::Display for Method {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let m = match self {
			Method::GET => "GET",
			Method::DELETE => "DELETE",
			Method::POST => "POST",
			Method::PUT => "PUT",
			Method::HEAD => "HEAD",
			Method::CONNECT => "CONNECT",
			Method::OPTIONS => "OPTIONS",
			Method::TRACE => "TRACE",
			Method::PATCH => "PATCH",
		};
        write!(f, "{}", m)
    }
}