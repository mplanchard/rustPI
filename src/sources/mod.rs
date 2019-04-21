// Trait definition for a package source

pub mod file;
use std::io;
use crate::packages::{Package};



#[derive(Debug)]
pub enum SourceError {
    LoadFailure(String),
    SaveFailure(String),
}

pub trait PackageSource<'a> {
    fn new(pkg: Package<'a>) -> Self;
    fn load(&self) -> Result<Box<[u8]>, SourceError>;
    fn save(&self, bytes: &[u8]) -> Result<(), SourceError>;
}
