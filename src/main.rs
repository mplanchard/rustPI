use std::env;

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

use dotenv::dotenv;

mod index;
mod models;
mod packages;
mod schema;
mod sources;

use packages::Package;
use sources::PackageSource;
use index::Index;


fn main() {
    use std::str;
    let pkg = Package{name: "foo", version: "bar", location: "file://local.pkg"};
    println!("{}", pkg.path_prefix());
    println!("{}", pkg.path());
    println!("{:?}", pkg.source());
    println!("{:?}", str::from_utf8(&pkg.source().load().unwrap()).unwrap());
    pkg.source().save(b"new package text").unwrap();
    println!("{:?}", str::from_utf8(&pkg.source().load().unwrap()).unwrap());
}
