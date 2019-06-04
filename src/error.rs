//! # Pypiserver Errors
//!
//! This module defines a custom error class used to unify error handling
//! throughout the application.
//!

use std::error;
use std::fmt;

#[derive(Debug)]
pub enum Errors {
    IO,
}

#[derive(Debug)]
pub struct Error {
    pub kind: Errors,
    pub message: String,
}
impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        // Generic error, underlying cause isn't tracked.
        None
    }
}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Error {{ kind: {:?}, message: {} }}",
            self.kind, self.message
        )
    }
}
