use rust_http_server::ThreadPool;

use crate::http::{ParseError, Request, Response, StatusCode};
use crate::website_handler::WebsiteHandler;

use std::convert::TryFrom;
use std::io::{Error, Read};
use std::marker::Sync;
use std::net::{SocketAddr, TcpListener, TcpStream};


pub trait Handler {
    fn handle_request(&self, request: &Request) -> Response;
    fn handle_bad_request(&self, e: &ParseError) -> Response {
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

    pub fn run<T: Handler + Send + Sync + Clone + 'static>(self, handler: T) {
        println!("Listening on {}", self.addr);

        let listener = TcpListener::bind(&self.addr).unwrap();
        let pool = ThreadPool::new(4);

        loop 
        {
            let result: Result<(TcpStream, SocketAddr), Error> = listener.accept();
            
            let clos =  || {
                process_stream(result, 
                    //&handler
                    &WebsiteHandler::new("".to_string())
                );
            };

            pool.execute(clos);
        }
    }
}

fn process_stream<T:  Handler>(result: Result<(TcpStream, SocketAddr), Error>, handler: &T) {
    match result {
        Ok((mut stream, _)) => {
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
