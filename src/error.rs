//! # Pypiserver Errors
//!
//! This module defines a custom error class used to unify error handling
//! throughout the application.
//!

use std::error;

use std::fmt;
use std::io;
use rusqlite;

#[derive(Debug)]
pub enum ErrType {
    IO,
    DB,
    Usage,
}

#[derive(Debug)]
pub struct Error {
    pub kind: ErrType,
    pub source: Option<Box<error::Error>>,
    pub message: Option<String>,
}
impl Error {
    /// Construct a custom error.
    pub fn new<S: Into<String>>(kind: ErrType, message: S) -> Self {
        Error {
            kind,
            source: None,
            message: Some(message.into()),
        }
    }
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
            kind: ErrType::DB,
            source: Some(Box::from(err)),
            message: Some("Error during sqlite operation".into()),
        }
    }
}
impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Self {
            kind: ErrType::IO,
            source: Some(Box::from(err)),
            message: Some("Error during IO".into()),
        }
    }
}
