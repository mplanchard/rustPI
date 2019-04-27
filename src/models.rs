// DB Models

use diesel::{ Insertable, Queryable };
use crate::schema::packages;


#[derive(Queryable)]
pub struct IdxPackage {
    pub id: i64,
    pub name: String,
    pub version: String,
    pub location: String,
}

#[derive(Insertable)]
#[table_name = "packages"]
pub struct NewIdxPackage {
    pub name: String,
    pub version: String,
    pub location: String,
}
