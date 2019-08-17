/// Filesystem implementations
///

use std::fs;
use std::path::{Path, PathBuf};

// use crate::domain::model;
// use crate::domain::model::Pkg;
use crate::error::{ErrType, Error};


#[derive(Debug)]
pub struct FSPkgRepo {
    path: PathBuf,
}
impl FSPkgRepo {
    pub fn new<P: Into<PathBuf>>(path: P) -> Result<Self, Error> {
        let pathbuf = path.into();
        let metadata = fs::metadata(&pathbuf)?;
        if !metadata.is_dir() {
            return Err(Error::new(
                ErrType::Usage,
                format!("{:?} is not a directory", pathbuf),
            ));
        }
        if metadata.permissions().readonly() {
            return Err(Error::new(
                ErrType::Usage,
                format!("{:?} is not writeable", pathbuf),
            ));
        }
        Ok(Self { path: pathbuf })
    }
}

#[cfg(test)]
mod fspkgrpo_tests {

    use super::*;

    use std::process::Command;
    use tempfile::tempdir;

    #[test]
    fn new_err_path_does_not_exist() {
        assert!(FSPkgRepo::new("fapowiejfapowiejf").is_err());
    }

    #[test]
    fn new_err_path_is_not_a_directory() {
        let err = FSPkgRepo::new("Cargo.toml");
        assert!(err
            .unwrap_err()
            .message
            .unwrap()
            .contains("not a directory"));
    }

    #[test]
    fn new_err_path_is_readonly() {
        // Not bothering making this windows compatible
        if cfg!(windows) {
            return;
        }
        let tmpdir = tempdir().unwrap();
        let ro_dir_path = tmpdir.path().join("readonly");
        Command::new("mkdir")
            .arg(ro_dir_path.to_str().unwrap())
            .output()
            .expect("Could not make readonly dir");
        Command::new("chmod")
            .arg("0444")
            .arg(ro_dir_path.to_str().unwrap())
            .output()
            .expect("Could not set dir permissions");

        let err = FSPkgRepo::new(ro_dir_path);
        assert!(err.unwrap_err().message.unwrap().contains("writeable"));
    }

    #[test]
    fn new_success() {
        let tmpdir = tempdir().unwrap();
        assert!(FSPkgRepo::new(tmpdir.path()).is_ok());
    }
}
