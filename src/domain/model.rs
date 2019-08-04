//! Domain for pypiserver
//!

use crate::error;

/// Metadata about a package
pub struct PkgMeta {
    pub name: String,
    pub version: String,
    pub location: String,
}
impl PkgMeta {
    pub fn new<N, V, L>(name: N, version: V, location: L) -> Self
    where
        N: Into<String>,
        V: Into<String>,
        L: Into<String>,
    {
        Self {
            name: name.into(),
            version: version.into(),
            location: location.into(),
        }
    }
}

/// A package
pub struct Pkg<'a> {
    meta: &'a PkgMeta,
    bytes: Box<[u8]>,
}

/// An repository for package metadata
pub trait PkgMetaRepo {
    /// Add metadata to the repo
    fn add(&self, meta: &PkgMeta) -> Result<(), error::Error>;
    /// Remove a package's metadata from the repo
    fn delete(&self, meta: &PkgMeta) -> Result<(), error::Error>;
    /// Get a package by name & version
    fn get<T: AsRef<str>, U: AsRef<str>>(
        &self,
        name: T,
        ver: U,
    ) -> Result<Option<PkgMeta>, error::Error>;
    /// Get all packages in the repo
    fn get_all(&self) -> Result<Vec<PkgMeta>, error::Error>;
    /// Return all records for a package with the given name
    fn with_name<T: AsRef<str>>(&self, name: T) -> Result<Vec<PkgMeta>, error::Error>;
}

/// Package repository
pub trait PkgRepo {
    fn add(&self, pkg: Pkg) -> Result<PkgMeta, error::Error>;
    fn delete(&self, meta: &PkgMeta) -> Result<(), error::Error>;
    fn get(&self, meta: &PkgMeta) -> Option<Pkg>;
    fn replace(&self, meta: &PkgMeta, pkg: Pkg) -> Result<PkgMeta, error::Error>;
}
