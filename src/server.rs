use rust_http_server::ThreadPool;

use crate::http::{ParseError, Request, Response, StatusCode};

use std::convert::TryFrom;
use std::io::Read;
use std::net::TcpListener;

pub trait Handler {
    fn handle_request(&mut self, request: &Request) -> Response;
    fn handle_bad_request(&mut self, e: &ParseError) -> Response {
        print!("Failed to parse request: {}", e);
        Response::new(StatusCode::BadRequest, None)
    }
}

pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self { addr }
    }

    pub fn run(self, mut handler: impl Handler) {
        println!("Listening on {}", self.addr);

        let listener = TcpListener::bind(&self.addr).unwrap();
        let pool = ThreadPool::new(4);

        loop {
            match listener.accept() {                
                Ok((mut stream, addr)) => {
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(bytes_count) => {
                            println!(
                                "Received {} bytes of a request: {}",
                                bytes_count,
                                String::from_utf8_lossy(&buffer)
                            );

                            let response = match Request::try_from(&buffer[..]) {
                                Ok(request) => handler.handle_request(&request),
                                Err(e) => handler.handle_bad_request(&e),
                            };
                            if let Err(e) = response.send(&mut stream) {
                                println!("Failed to send response {}", e);
                            }
                        }
                        Err(e) => {
                            println!("Failed to read from connecion {}", e);
                        }
                    };
                }
                Err(e) => {
                    println!("Failed to establish a connection: {}", e);
                }
            }
        }
    }
}
