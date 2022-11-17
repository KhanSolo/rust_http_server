use server::Server;
use std::env;
use website_handler::WebsiteHandler;

mod http;
mod server;
mod website_handler;

fn main() {
    let public_path = get_public_path();
    let handler = WebsiteHandler::new(public_path);
    let server = Server::new("127.0.0.1:8080".to_string());

    server.run(&handler);
}

fn get_public_path() -> String {
    let delimiter = match env::consts::OS {
        "windows" => "\\",
        _ => "/",
    };

    let default_path = format!("{}{}public", env!("CARGO_MANIFEST_DIR"), delimiter);
    let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path);

    let public_path = match std::fs::canonicalize(public_path) {
        Ok(path) => path.into_os_string().into_string().unwrap(),
        Err(_) => todo!(),
    };
    println!("public path: {}", public_path);
    public_path
}
