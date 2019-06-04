//! # Repository &/or Registry Interfaces
//!
//! This module provides trait interfaces for persistence repositories
//! and their operations.
//!

pub struct RepositoryRegistry<'a, T> {
    repository_getter: &'a Fn(&str) -> T,
}
impl<'a, T> RepositoryRegistry<'a, T> {
    fn new(f: &'a Fn(&str) -> T) -> Self {
        RepositoryRegistry {
            repository_getter: f,
        }
    }
}
