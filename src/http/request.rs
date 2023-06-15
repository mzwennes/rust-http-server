use super::method::Method;
use super::protocol::Protocol;
use super::QueryString;
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Debug, Display};
use std::str::{self, Utf8Error};

#[derive(Debug)]
pub struct Request<'buf> {
    method: Method,
    path: &'buf str,
    query_string: Option<QueryString<'buf>>,
    protocol: Protocol,
    headers: Option<String>,
    body: Option<String>,
}

impl<'buf> Request<'buf> {
    pub fn path(&self) -> &str {
        &self.path
    }
    pub fn method(&self) -> &Method {
        &self.method
    }
    pub fn query_string(&self) -> Option<&QueryString> {
        self.query_string.as_ref()
    }
}

impl<'buf> TryFrom<&'buf [u8]> for Request<'buf> {
    type Error = ParseError;

    // GET /search?name=abc&sort=1 HTTP/1.1\r\n...HEADERS...
    fn try_from(buffer: &'buf [u8]) -> Result<Self, Self::Error> {
        let request = str::from_utf8(buffer)?;

        let (method, request) = get_next_lexicon(request).ok_or(ParseError::InvalidRequest)?;
        let (mut path, request) = get_next_lexicon(request).ok_or(ParseError::InvalidRequest)?;
        let (protocol, _) = get_next_lexicon(request).ok_or(ParseError::InvalidRequest)?;

        let mut query_string = None;
        if let Some(i) = path.find("?") {
            query_string = Some(QueryString::from(&path[i + 1..]));
            path = &path[..i];
        }

        Ok(Self {
            method: method.parse()?,
            path: path,
            query_string: query_string,
            protocol: protocol.parse()?,
            headers: None,
            body: None,
        })
    }
}

fn get_next_lexicon(request: &str) -> Option<(&str, &str)> {
    for (index, char) in request.chars().enumerate() {
        if char == ' ' || char == '\r' {
            return Some((&request[..index], &request[index + 1..]));
        }
    }

    None
}

#[derive(Debug)]
pub enum ParseError {
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod,
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message())
    }
}

impl From<Utf8Error> for ParseError {
    fn from(_: Utf8Error) -> Self {
        Self::InvalidEncoding
    }
}

impl ParseError {
    fn message(&self) -> &str {
        match self {
            ParseError::InvalidRequest => "Invalid Request",
            ParseError::InvalidEncoding => "Invalid Encoding",
            ParseError::InvalidProtocol => "Invalid Protocol",
            ParseError::InvalidMethod => "Invalid Method",
        }
    }
}

impl Error for ParseError {}
