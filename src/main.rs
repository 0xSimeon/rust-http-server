// BUILDING AN HTTP 1.1 SERVER USING RUST
#![allow(dead_code)] // silence deadcode warnings
#![allow(unused_imports)] // silence unused imports warnings
#![allow(unused)] // silence unused codes warnings (like unused variables.)

use http::{method, request};
use server::Server;
use std::env;
use website_handler::WebsiteHandler;

mod http;
mod server;
mod website_handler;

fn main() {
    let default_path = format!("{}/public", env!("CARGO_MANIFEST_DIR")); // inspect and expands the environment variable
    let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path); // get the environment variable if it was passed by user. if it wasn't set default to default_path

    // println!("public path: {}", public_path);  logging to confirm currentl path

    // server is a struct (similar to class in other langs)
    let server = Server::new(String::from("127.0.0.1:8080"));
    server.run(WebsiteHandler::new(public_path));
}
