use crate::http::{ParseError, Request, Response, StatusCode};
use std::convert::TryFrom;
use std::io::Read;
use std::net::TcpListener;

pub trait Handler {
    fn handle_request(&mut self, request: &Request) -> Response;
    fn handle_bad_request(&mut self, err: &ParseError) -> Response {
        println!("Failed to parse request: {}", err);
        Response::new(StatusCode::BadRequest, None)
    }
}

pub struct Server {
    address: String,
}

impl Server {
    pub fn new(address: String) -> Self {
        Server { address }
    }

    pub fn run(self, mut handler: impl Handler) -> std::io::Result<()> {
        let listener = TcpListener::bind(&self.address).unwrap();

        //TODO: implement
        // listener.set_nonblocking(true).unwrap();

        for stream in listener.incoming() {
            match stream {
                Ok(mut stream) => {
                    let mut buf = [0; 1024];
                    match stream.read(&mut buf) {
                        Ok(_) => {
                            let response = match Request::try_from(&buf[..]) {
                                Ok(request) => handler.handle_request(&request),
                                Err(err) => handler.handle_bad_request(&err),
                            };

                            if let Err(e) = response.send(&mut stream) {
                                println!("Failed to send response: {}", e);
                            }
                        }
                        Err(err) => println!("Failed to read from connection: {}", err),
                    }
                }
                Err(err) => println!("Failed to establish a connection: {}", err),
            }
        }

        println!("Listening on {}", self.address);
        Ok(())
    }
}
