// Representation of a python package

use std::fmt::Debug;
use crate::sources::PackageSource;
use crate::sources::file::FileSource;


#[derive(Debug)]
pub struct Package<'a> {
    pub name: &'a str,
    pub version: &'a str,
    pub location: &'a str,
}


impl<'a> Package<'a> {
    pub fn path(&self) -> &str {
        self.location
            .find("://")
            .map(|idx| &self.location[idx + 3..])
            .unwrap()
    }

    pub fn path_prefix(&self) -> & str {
        self.location
            .find("://")
            .map(|idx| &self.location[..idx])
            .unwrap()
    }

    pub fn source(&self) -> impl PackageSource + Debug {
        match self.path_prefix() {
            "file" => FileSource::new(&self),
            _ => FileSource::new(&self),
        }
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    fn make_pkg<'a>() -> Package<'a> {
        Package {name: "name", version: "1.0.0", location: "file://local.zip"}
    }

    #[test]
    fn test_path() {
        assert_eq!(make_pkg().path(), "local.zip")
    }

    #[test]
    fn test_path_prefix() {
        assert_eq!(make_pkg().path_prefix(), "file")
    }
}
