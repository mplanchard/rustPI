// Representation of a python package

use std::borrow::Cow;

use lazy_static;
use regex;

use crate::error::Error;

lazy_static::lazy_static! {
    pub static ref NAME_RE: regex::Regex = regex::Regex::new(
        "^([A-Z0-9]|[A-Z0-9][A-Z0-9._-]*[A-Z0-9])$"
    ).unwrap();
}

trait PackageRepository {
    fn get_package<'a>(&self, package: &'a PythonPackageMetadata) -> Result<PythonPackage, Error>;
    fn insert_package<'a>(&self, package: &'a PythonPackage) -> Result<PythonPackageMetadata, Error>;
    fn save_package<'a>(&self, package: &'a PythonPackage) -> Result<PythonPackageMetadata, Error>;
}

/// The metadata for a package
///
/// Includes information that might be used to search for a given package,
/// plus its `url`, which indicates which indicates which persistence
/// repository to retrieve it from and the path within that repository.
#[derive(Debug)]
pub struct PythonPackageMetadata<'a> {
    pub name: Cow<'a, str>,
    pub version: Cow<'a, str>,
    pub url: Cow<'a, str>,
}
impl<'a> PythonPackageMetadata<'a> {
    pub fn new<S: Into<Cow<'a, str>>>(name: S, version: S, url: S) -> Self {
        PythonPackageMetadata {
            name: name.into(),
            version: version.into(),
            url: url.into(),
        }
    }
}

#[derive(Debug)]
pub struct PythonPackage<'a> {
    pub meta: &'a PythonPackageMetadata<'a>,
    pub data: &'a [u8],
}
