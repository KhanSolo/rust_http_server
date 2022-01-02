#![allow(dead_code)]

use server::Server;
use std::env;
use website_handler::WebsiteHandler;

mod http;
mod server;
mod website_handler;

fn main() {

    let os = std::env::consts::OS;
    let delimiter = match os {
        "windows" => "\\",
        _ => "/"
    };

    let default_path = format!("{}{}public", env!("CARGO_MANIFEST_DIR"), delimiter);
    let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path);

    let public_path = match std::fs::canonicalize(public_path) {
        Ok(path) => {
            path.into_os_string().into_string().unwrap()
        },
        Err(_) => todo!(),
    };

    println!("public path: {}", public_path);
    let server = Server::new("127.0.0.1:8080".to_string());
    let handler = WebsiteHandler::new(public_path);
    server.run(handler);
}
