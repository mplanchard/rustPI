// Representation of a python package

pub struct Package<'a> {
    pub name: &'a str,
    pub version: &'a str,
    pub location: &'a str,
}

impl<'a> Package<'a> {
    pub fn path(&self) -> &str {
        self.location
            .find("://")
            .map(|idx| &self.location[idx + 3..])
            .unwrap()
    }
}
