use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct ReaderError {
    details: String,
}

impl ReaderError {
    pub fn new(msg: &str) -> ReaderError {
        ReaderError {
            details: msg.to_string(),
        }
    }
}

impl fmt::Display for ReaderError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for ReaderError {
    fn description(&self) -> &str {
        &self.details
    }
}
