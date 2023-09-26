#![allow(dead_code)]

use server::Server;
use std::env;
use http::Method;
use http::Request;
use web_handler::WebHandler;

mod server;
mod http;
mod web_handler;

fn main() {
    let default_path = format!("{}/public", env!("CARGO_MANIFEST_DIR"));
    let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path);
    let server = Server::new(String::from("127.0.0.1:8080"));
    server.run(WebHandler::new(public_path));
}