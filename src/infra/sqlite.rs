///! Sqlite implementations
use std::path::Path;

use rusqlite;
use rusqlite::OptionalExtension;

use crate::domain::model;
use crate::domain::model::PkgMeta;
use crate::error;

const UP_MIGRATION: &'static str = "
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

const DOWN_MIGRATION: &'static str = "
    BEGIN;
    DROP INDEX IF EXISTS idx_name_version;
    DROP TABLE IF EXISTS packages;
    COMMIT;
    ";


pub struct SqliteRepo {
    conn: rusqlite::Connection,
}
impl SqliteRepo {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self, error::Error> {
        let conn = rusqlite::Connection::open_with_flags(
            path,
            rusqlite::OpenFlags::SQLITE_OPEN_READ_WRITE | rusqlite::OpenFlags::SQLITE_OPEN_CREATE,
        )?;
        conn.execute_batch(UP_MIGRATION)?;
        Ok(Self { conn })
    }
    fn pkg_meta_from_row(row: &rusqlite::Row) -> Result<PkgMeta, rusqlite::Error> {
        Ok(PkgMeta::new(
            row.get::<usize, String>(0)?,
            row.get::<usize, String>(1)?,
            row.get::<usize, String>(2)?,
        ))
    }
}
impl model::PkgMetaRepo for SqliteRepo {
    fn add(&self, meta: &PkgMeta) -> Result<(), error::Error> {
        self.conn.execute_named(
            "INSERT INTO packages (name, version, location)
                VALUES (:name, :version, :location)",
            rusqlite::named_params! {
                ":name": meta.name,
                ":version": meta.version,
                ":location": meta.location,
            },
        )?;
        Ok(())
    }
    fn delete(&self, meta: &PkgMeta) -> Result<(), error::Error> {
        self.conn.execute_named(
            "DELETE FROM packages
                WHERE
                    name = ':name'
                    AND version = ':version'
                    AND location = ':location'",
            rusqlite::named_params! {
                ":name": meta.name,
                ":version": meta.version,
                ":location": meta.location,
            },
        )?;
        Ok(())
    }
    fn get<T: AsRef<str>, U: AsRef<str>>(
        &self,
        name: T,
        ver: U,
    ) -> Result<Option<PkgMeta>, error::Error> {
        self.conn
            .query_row_named(
                "SELECT name, version, location FROM packages WHERE
                name = ':name'
                AND version = ':version'",
                rusqlite::named_params! {
                    ":name": name.as_ref(), ":version": ver.as_ref()
                },
                Self::pkg_meta_from_row,
            )
            .optional()
            .map_err(error::Error::from)
    }
    fn get_all(&self) -> Result<Vec<PkgMeta>, error::Error> {
        Ok(self
            .conn
            .prepare("SELECT name, version, location FROM packages")?
            .query_map(rusqlite::NO_PARAMS, Self::pkg_meta_from_row)?
            .collect::<Result<Vec<PkgMeta>, rusqlite::Error>>()?)
    }
    fn with_name<T: AsRef<str>>(&self, name: T) -> Result<Vec<PkgMeta>, error::Error> {
        Ok(self
            .conn
            .prepare(
                "
                SELECT name, version, location FROM packages
                WHERE name = ':name'
            ",
            )?
            .query_map_named(
                rusqlite::named_params! {":name": name.as_ref()},
                Self::pkg_meta_from_row,
            )?
            .collect::<Result<Vec<PkgMeta>, rusqlite::Error>>()?)
    }
}


#[cfg(test)]
mod tests {
    use tempfile::{NamedTempFile, TempDir};

    use crate::domain::model::PkgMetaRepo;

    use super::*;

    #[test]
    fn create_get_delete() {
        let file = NamedTempFile::new().unwrap();
        let repo = SqliteRepo::new(file.path()).unwrap();
        let pkgmeta = PkgMeta::new("foo", "1.0", "fs://here");
        repo.add(&pkgmeta).unwrap();
        let retrieved = repo.get(&pkgmeta.name, &pkgmeta.version).unwrap().unwrap();
        assert_eq!(retrieved.location, pkgmeta.location);
        repo.delete(&pkgmeta).unwrap();
    }

}
