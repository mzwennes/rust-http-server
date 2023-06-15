use super::ParseError;
use std::str::FromStr;

#[derive(Debug)]
pub enum Method {
    Get,
    Post,
    Delete,
    Put,
    Head,
    Connect,
    Options,
    Trace,
    Patch,
}

impl FromStr for Method {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "GET" => Ok(Method::Get),
            "POST" => Ok(Method::Post),
            "DELETE" => Ok(Method::Delete),
            "PUT" => Ok(Method::Put),
            "HEAD" => Ok(Method::Head),
            "CONNECT" => Ok(Method::Connect),
            "TRACE" => Ok(Method::Trace),
            "OPTIONS" => Ok(Method::Options),
            "PATCH" => Ok(Method::Patch),
            _ => Err(ParseError::InvalidMethod),
        }
    }
}
