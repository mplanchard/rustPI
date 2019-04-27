// File source

use std::collections::hash_map::DefaultHasher;
use std::env;
use std::fs;
use std::hash::{Hash, Hasher};
use std::io::{Read, Write};
use std::path;
use std::thread;
use std::time;

use crate::packages::Package;
use crate::sources::{PackageSource, SourceError};

#[derive(Debug)]
pub struct FileSource<'a> {
    pub pkg: &'a Package<'a>,
    tmp_dir: Box<path::Path>
}

impl<'a> FileSource<'a> {
    /// Return a path to write a temporary version of the file to.
    fn tmp_path(&self) -> path::PathBuf {
        let pkg_path = self.pkg.path();
        if let Some(file_name) = path::Path::new(pkg_path).file_name() {
            return self.tmp_dir.join(file_name);
        }
        // If we couldn't get the file name for some reason, hash the path.
        let mut hasher = DefaultHasher::new();
        pkg_path.hash(&mut hasher);
        self.tmp_dir.join(format!("{:x}", hasher.finish()))
    }
}


impl<'a> PackageSource<'a> for FileSource<'a> {
    fn new(pkg: &'a Package<'a>) -> FileSource {
        let root_tmp_dir = env::temp_dir();
        let tmp_dir_path = root_tmp_dir.join("rustpi");
        let tmp_dir = fs::create_dir_all(&tmp_dir_path)
            .map(|_| tmp_dir_path)
            .unwrap_or(root_tmp_dir);
        FileSource { pkg, tmp_dir: tmp_dir.into_boxed_path() }
    }

    /// Returns a result of a file's bytes loaded from the disk
    fn load(&self) -> Result<Box<[u8]>, SourceError> {
        // TODO: better error handling here
        fs::File::open(self.pkg.path())
            .map(|f| Box::from(f.bytes().map(Result::unwrap).collect::<Vec<u8>>()))
            .map_err(|e| SourceError::LoadFailure(format!("{}", e)))
    }

    /// Saves a file's bytes to the disk
    fn save(&self, bytes: &[u8]) -> Result<(), SourceError> {
        // TODO: maybe write a separate file and then rename for safety.
        let tmp_path = self.tmp_path();
        fs::OpenOptions::new()
            .create(true)
            .write(true)
            .open(&tmp_path)
            // write new content to the file
            .and_then(|mut f| f.write(bytes).map(|len| (len, f)))
            // truncate the file to the written length
            .and_then(|(len, f)| f.set_len(len as u64))
            .map_err(|e| SourceError::SaveFailure(format!("{}", e)))?;

        fs::rename(&tmp_path, self.pkg.path())
            .map(Ok)
            .map_err(|e| SourceError::SaveFailure(format!("{}", e)))?

    }
}
