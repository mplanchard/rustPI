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
//! ## Architecture
//!
//! This application is organized largely according to Eric Evans'
//! [Domain-driven Design].
//!
//! ### Domain Models
//!
//! PythonPackage: a binary package file
//! PythonPackageMeta: metadata associated with a Package
//! PythonPackageRepository: a persistence layer for Packages
//! PythonPackageMetaRepository: a persistence layer for PackageMeta data
//!
//! User: a managed user (aggregate)
//! Group: a group of users with a default Role or Permissions
//! UserIdentity: information used to verify a user
//! Permission: a bitwise combination of
//!   * View
//!   * Download
//!   * Upload - No Replace
//!   * Upload - Replace
//!   * Delete
//! Role: a friendly name for a suite of permissions
//!
//! UserRepository: persistence layer for user data
//!
//! ### Interfaces
//!
//! * CLI
//! * PythonSimple (pip-compatible API)
//! * Web
//!
//! ### Infrastructure
//!
//! PackageRepositoryFS: filesystem storage of packages
//! PackageMetaRepositorySqlite: Sqlite storage of package metadata
//! UserRepositorySqlite: Sqlite storage of user data
//!
//! [Domain-driven Design]: https://en.wikipedia.org/wiki/Domain-driven_design
//!

use std::env;

use dotenv::dotenv;

mod db;
mod domain;
mod error;
mod infra;

// use lazy_static::lazy_static;
// use warp;

// use infrastructure::web::warp::routes;

// lazy_static! {
//         PythonPackageMetadata::new("bar", "1.2", "fs://bar"),
// }

fn main() {}
