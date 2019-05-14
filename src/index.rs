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
        Index { location: location.into(), connection: None }
    }

    pub fn connect(&self) -> Result<(), IndexError> {
        make_index_dir(&self.location)
            .map_err(|_| IndexError::PathError(self.location))?;

        let connection = rusqlite::Connection::open(&self.location)
            .map_err(IndexError::CouldNotConnect)?;

        Ok(())
    }

    pub fn save_package(&self, pkg: &'a Package) -> Result<(), IndexError> {
        diesel::replace_into(packages::table)
            .values((
                packages::name.eq(pkg.name),
                packages::version.eq(pkg.version),
                packages::columns::location.eq(pkg.location),
            ))
            .execute(&self.connection)
            .unwrap();
        Ok(())
    }
}


#[derive(Debug)]
/// A wrapper for errors that might occur with the index
pub enum IndexError {
    CouldNotConnect(ConnectionError),
    MigrationFailed(RunMigrationsError),
    PathError(String),
}

impl fmt::Display for IndexError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            IndexError::CouldNotConnect(e) => write!(f, "Could not connect to index: {}", e),
            IndexError::MigrationFailed(e) => write!(f, "Could not upgrade index: {}", e),
            IndexError::PathError(e) => write!(f, "Problem resolving index path: {}", e),
        }
    }
}

impl error::Error for IndexError {
    fn description(&self) -> &str {
        "An error occurred trying to access or update the index."
    }

    fn cause(&self) -> Option<&error::Error> {
        // Generic error, underlying cause isn't tracked.
        match self {
            IndexError::CouldNotConnect(e) => Some(e),
            IndexError::MigrationFailed(e) => Some(e),
            IndexError::PathError(p) => None,
        }
    }
}

/// Return the expected home directory of the index
///
/// Used if the full path is not specified.
fn index_home() -> PathBuf {
    let dir = dirs::data_local_dir();
    match dir {
        Some(mut dir) => {
            dir.push("rustpi");
            dir
        }
        None => {
            println!(
                "WARNING! Could not discover the standard system data directory.
                Using the current directory instead."
            );
            PathBuf::from("rustpi")
        }
    }
}

fn index_path() -> PathBuf {
    if let Some(path) = env::var_os("PYPISERVER_INDEX_PATH") {
        return PathBuf::from(path);
    }
    let mut path = index_home();
    path.push("index.sql");
    path
}

fn index_exists() -> bool {
    fs::metadata(index_path())
        .map(|f| f.is_file())
        .unwrap_or(false)
}

fn make_index_dir<T: Into<PathBuf>>(index_path: T) -> io::Result<()> {
    if let Some(parent) = index_path.into().parent() {
        return fs::DirBuilder::new().recursive(true).create(parent);
    }
    Ok(())
}


#[cfg(test)]
mod tests {
    use std::env;
    use std::fs;
    use std::io;
    use std::time;

    use tempfile;

    use super::*;

    fn set_db_path() -> PathBuf {
        let tmpdir = tempfile::tempdir().unwrap();
        let db_path = &tmpdir.path().join("testdb.sql");
        env::set_var("PYPISERVER_INDEX_PATH", db_path);
        db_path.to_owned()
    }

    #[test]
    fn migration_adds_stuff_to_db() {
        set_db_path();
        let idx = Index::connect().unwrap();
        idx.migrate().unwrap();
        let res = idx.connection.execute("PRAGMA table_info([packages]);").unwrap();
        println!("{:?}", res);
        assert!(res == 1);
    }

}
