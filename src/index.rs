// Database interaction

use std::env;
use std::error;
use std::fmt;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use diesel::migration::RunMigrationsError;
use diesel::Connection;
use diesel::ConnectionError;
use diesel::ExpressionMethods;
use diesel::RunQueryDsl;
use diesel::SqliteConnection;
use diesel_migrations::embed_migrations;
use dirs;

use crate::models::{IdxPackage, NewIdxPackage};
use crate::packages::Package;
use crate::schema::packages;

embed_migrations!();

pub struct Index {
    // location: String,
    connection: SqliteConnection,
}

impl<'a> Index {
    pub fn connect() -> Result<Index, IndexError> {
        let location = index_path();
        let location_string = location.to_string_lossy();
        make_index_dir(&location)
            .map_err(|_| IndexError::PathError(location_string.to_string()))?;

        let connection = SqliteConnection::establish(
            &location_string
        ).map_err(IndexError::CouldNotConnect)?;
        // Ok(Index { location: location_string.to_string(), connection })
        Ok(Index { connection })
    }

    pub fn migrate(&self) -> Result<(), IndexError> {
        embedded_migrations::run(&self.connection).map_err(IndexError::MigrationFailed)?;
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

fn make_index_dir(index_path: &Path) -> io::Result<()> {
    if let Some(parent) = index_path.parent() {
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
    use super::*;

    fn set_db_path() -> &'static str {
        let db_path = "resources/testdb.sql";
        env::set_var("PYPISERVER_INDEX_PATH", db_path);
        db_path
    }

    fn teardown() {
        let db_path = "resources/testdb.sql";
        if let Some(path) = env::var_os("PYPISERVER_INDEX_PATH") {
            if path.to_str().unwrap() == db_path {
                fs::metadata(&db_path[..])
                    .map(|_| fs::remove_file(db_path).unwrap())
                    .unwrap();
            }
        }
    }

    #[test]
    fn connect_makes_db_file_if_not_exists() {
        let db_path = set_db_path();

        assert!(fs::metadata(db_path).is_err());  // does not exist

        Index::connect().unwrap();

        assert!(fs::metadata(db_path).is_ok());  // will fail if does not exist

        teardown();
    }

    #[test]
    fn connect_makes_db_dir_if_not_exists() {
        let db_dir = "testdir";
        let db_path = "testdir/testdb.sql";
        env::set_var("PYPISERVER_INDEX_PATH", db_path);

        assert!(fs::metadata(db_dir).is_err());
        assert!(fs::metadata(db_path).is_err());

        Index::connect().unwrap();

        let mut checks = Vec::new();
        for _ in 0..10 {
            checks.push(fs::metadata(db_path));
        }

        assert!(checks.iter().any(|r| r.is_ok()));

        fs::remove_dir_all(db_dir).unwrap();

    }

    #[test]
    fn migration_adds_stuff_to_db() {
        let db_path = set_db_path();
    }


}
