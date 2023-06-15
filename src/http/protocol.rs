use std::str::FromStr;

use super::ParseError;

#[derive(Debug)]
pub enum Protocol {
    HTTP1,
}

impl FromStr for Protocol {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "HTTP/1.1" => Ok(Protocol::HTTP1),
            _ => Err(ParseError::InvalidProtocol),
        }
    }
}
