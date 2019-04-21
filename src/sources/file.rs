// File source

use std::fs;
use std::io;
use std::io::{Read, Write};

use crate::packages::Package;
use crate::sources::{PackageSource, SourceError};

pub struct FileSource<'a> {
    pub pkg: Package<'a>,
}

impl<'a> PackageSource<'a> for FileSource<'a> {
    fn new(pkg: Package<'a>) -> FileSource {
        FileSource { pkg }
    }

    /// Returns a result of a file's bytes loaded from the disk
    fn load(&self) -> Result<Box<[u8]>, SourceError> {
        fs::File::open(self.pkg.path())
            .map(|f| Box::from(f.bytes().map(|r| r.unwrap()).collect::<Vec<u8>>()))
            .map_err(|e| SourceError::LoadFailure(format!("{}", e)))
    }

    /// Saves a file's bytes to the disk
    fn save(&self, bytes: &[u8]) -> Result<(), SourceError> {
        let path = self.pkg.path();
        fs::OpenOptions::new().create(true).write(true).open(path)
            .map(|mut f| f.write(bytes))
            .map(|_| ())
            .map_err(|e| SourceError::SaveFailure(format!("{}", e)))
    }
}
