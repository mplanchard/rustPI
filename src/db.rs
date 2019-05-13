//! Database
//!

use std::error;
use std::fmt;

use rusqlite;

const UP_MIGRATION: &str =
    "
    BEGIN;
    CREATE TABLE IF NOT EXISTS packages (
    id INTEGER NOT NULL PRIMARY KEY,
    name TEXT NOT NULL,
    version TEXT NOT NULL,
    location TEXT NOT NULL
    );

    CREATE UNIQUE INDEX idx_name_version ON packages (name, version);
    COMMIT;
    ";


const DOWN_MIGRATION: &str =
    "
    BEGIN;
    DROP INDEX IF EXISTS idx_name_version;
    DROP TABLE IF EXISTS packages;
    COMMIT;
    ";


pub enum DBError<T> where
    T: error::Error + fmt::Display + fmt::Debug
{
    MigrationFailed(T)
}


pub fn migrate(conn: rusqlite::Connection) -> Result<(), DBError<rusqlite::Error>> {
    conn.execute_batch(UP_MIGRATION).map_err(DBError::MigrationFailed)
}
