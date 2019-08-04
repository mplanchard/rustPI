//! # Pypiserver Errors
//!
//! This module defines a custom error class used to unify error handling
//! throughout the application.
//!

use std::error;
use std::fmt;

use rusqlite;

#[derive(Debug)]
pub enum Errors {
    IO,
    DB,
}

#[derive(Debug)]
pub struct Error {
    kind: Errors,
    source: Option<Box<error::Error>>,
    message: Option<String>,
}
impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match &self.source {
            Some(err) => Some(&**err),
            None => None,
        }
    }
}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.source {
            Some(err) => write!(f, "{:?} error: {}", self.kind, err),
            None => write!(f, "{:?} error: {:?}", self.kind, self.message),
        }
    }
}
impl From<rusqlite::Error> for Error {
    fn from(err: rusqlite::Error) -> Self {
        Self {
            kind: Errors::DB,
            source: Some(Box::from(err)),
            message: None,
        }
    }
}
