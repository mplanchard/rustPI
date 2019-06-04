//! # pypiserver
//!
//! A pip and twine compatible python package server, written in rust.
//!
//! ## Roadmap
//!
//! This project is currently under active development. With lots of
//! planned features coming down the pipe. Below is a broad overview
//! of features intended to be implemented for the initial version, plus
//! future improvements to be expected in later versions.
//!
//! ### 1.0.0 - The MVP
//!
//! No weaselly 0.1.0 releases here! Our first version will be labeled
//! as such.
//!
//! * Upload (with twine), download (with pip), and search (with pip)
//!   for managed packages
//! * Package storage on the local filesystem
//! * Use of a simple, local package index to ensure that we can serve
//!   as many packages as will fit on the filesystem
//! * Authentication and authorization for package operations with CLI
//!   management of users and permissions
//! * Deployment via `cargo install`, docker, and `pip install`
//! * Basic web interface for browsing packages
//!
//! ### 2.0.0 - The User-friendly Release
//!
//! * Improved web interface for browsing packages
//! * Upload & download packages via the web interface
//! * Manage users & permissions through the web interface
//!
//! ### 3.0.0 - The Distributed Release
//!
//! * Allow non-local packages, minimally supporting Amazon S3
//! * Enable the use of a non-local package index
//! * Multiple pypiserver instances can effectively share a remote store
//!   and package index for horizontal scalability
//!

use std::env;

use dotenv::dotenv;

mod db;
mod domain;
mod error;
mod index;
mod models;
mod packages;
mod sources;

fn main() {
    println!("{:?}", domain::model::package::NAME_RE.as_str())
}
