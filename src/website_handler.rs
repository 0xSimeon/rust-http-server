use super::http::{Method, Request, Response, StatusCode};
use super::server::Handler;
use std::fmt::format;
use std::fs;

pub struct WebsiteHandler {
    public_path: String,
}

impl WebsiteHandler {
    pub fn new(public_path: String) -> Self {
        Self { public_path }
    }

    fn read_file(&mut self, file_path: &str) -> Option<String> {
        let path = format!("{}/{}", self.public_path, file_path);

        match fs::canonicalize(path) {
            // get absolute path
            Ok(path) => {
                if path.starts_with(&self.public_path) {
                    fs::read_to_string(path).ok() // Path begins with public path and is safe to work with - read from it and convert to string then Result >> Option
                } else {
                    println!("Directory Traversal Attack attempted: {}", file_path); // path is bad, return None
                    None
                }
            } //
            Err(_) => None,
        }
    }
}

impl Handler for WebsiteHandler {
    fn handle_request(&mut self, request: &Request) -> Response {
        match request.method() {
            Method::GET => match request.path() {
                "/" => Response::new(StatusCode::Ok, self.read_file("index.html")),
                "/hello" => Response::new(StatusCode::Ok, self.read_file("hello.html")),
                path => match self.read_file(path) {
                    Some(contents) => Response::new(StatusCode::Ok, Some(contents)),
                    None => Response::new(StatusCode::NotFound, None),
                },
            },
            _ => Response::new(StatusCode::NotFound, None),
        }
    }
}
