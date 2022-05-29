use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Copy, Clone, Debug)] // generate Copy,  Clone and Debug implementations.
pub enum StatusCode {
    Ok = 200,
    BadRequest = 400,
    NotFound = 404,
}

impl StatusCode {
    pub fn reason_phrase(&self) -> &str {
        match self {
            Self::Ok => "Okay",
            Self::BadRequest => "Bad Request",
            Self::NotFound => "Not Found",
        }
    }
}

impl Display for StatusCode {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", *self as u16) // cast a string to an unsigned integer (copy trait needs to be implemented.)
                                      // *self dereferences self
    }
}
