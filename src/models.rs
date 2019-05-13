// DB Models

pub struct IdxPackage {
    pub id: i64,
    pub name: String,
    pub version: String,
    pub location: String,
}

pub struct NewIdxPackage {
    pub name: String,
    pub version: String,
    pub location: String,
}
