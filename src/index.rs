// Database interaction

use std::env;
use std::error;
use std::fmt;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use dirs;
use rusqlite;

use crate::models::{IdxPackage, NewIdxPackage};
use crate::packages::Package;

pub struct Index {
    location: String,
    connection: Option<rusqlite::Connection>,
}

impl<'a> Index {
    pub fn new<T: Into<String>>(location: T) -> Index {
        Index {
            location: location.into(),
            connection: None,
        }
    }
}
