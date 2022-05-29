// Module main file featuring all the modules for the project

// Expose implementations that will be called elsewhere.
pub use method::Method;
pub use query_string::{QueryString, Value as QueryStringValue};
pub use request::ParseError; // expose custom errors so we can use from anywhere.
pub use request::Request;
pub use response::Response;
pub use status_code::StatusCode;

pub mod method;
pub mod query_string;
pub mod request;
pub mod response;
pub mod status_code;
