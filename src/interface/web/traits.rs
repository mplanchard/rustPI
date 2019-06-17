//! Shared for web interfaces
//!

pub trait Renderer<T>: Sync {
    fn render(&self, to_render: &T) -> String;
}

pub trait GetEndpoint {
    const ENDPOINT: &'static str;

    fn get(&self) -> String;
}

pub trait Templated {
    const TEMPLATE: &'static str;
}

pub trait Server {
    fn run(&self);
}
