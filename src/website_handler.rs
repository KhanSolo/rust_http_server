use super::http::{Method, Request, Response, StatusCode};
use super::server::Handler;

use std::env;
use std::fs;

pub struct WebsiteHandler {
    public_path: String,
}

impl WebsiteHandler {
    pub fn new(public_path: String) -> Self {
        Self { public_path }
    }

    fn read_file(&self, file_path: &str) -> Option<String> {
        match env::consts::OS {
            "windows" => self.read_file_windows(file_path),
            "linux" => self.read_file_linux(file_path),
            _ => None,
        }
    }

    fn read_file_windows(&self, file_path: &str) -> Option<String> {
        match file_path.contains("..") {
            false => {
                let path = format!("{}{}", self.public_path, file_path.replace("/", "\\"));
                fs::read_to_string(path).ok()
            }
            true => {
                println!("Directory traversal attack attempted: {}", file_path);
                None
            }
        }
    }

    fn read_file_linux(&self, file_path: &str) -> Option<String> {
        let path = format!("{}/{}", self.public_path, file_path);
        match fs::canonicalize(path) {
            Ok(path) => {
                if path.starts_with(&self.public_path) {
                    fs::read_to_string(path).ok()
                } else {
                    println!("Directory traversal attack attempted: {}", file_path);
                    None
                }
            }
            Err(_) => None,
        }
    }
}

impl Handler for WebsiteHandler {
    fn handle_request(&self, request: &Request) -> Response {
        match request.method() {
            Method::GET => match request.path() {
                "/" => Response::new(StatusCode::Ok, self.read_file("/index.html")),
                "/hello" => Response::new(StatusCode::Ok, self.read_file("/hello.html")),
                path => match self.read_file(path) {
                    Some(contents) => Response::new(StatusCode::Ok, Some(contents)),
                    None => Response::new(StatusCode::NotFound, None),
                },
            },
            _ => Response::new(StatusCode::NotFound, None),
        }
    }
}

impl Clone for WebsiteHandler{
    fn clone(&self) -> Self{
        WebsiteHandler::new(self.public_path.clone())
    }
}

//impl Copy for WebsiteHandler{}