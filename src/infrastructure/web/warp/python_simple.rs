//! Warp endpoint implementations
//!
//!

// use warp;
// use warp::Filter;

// use crate::interface::python_simple::{Endpoint, SimpleIndex, SimpleIndexRenderer, ToHtml};

// pub struct SimpleIndexWarp<'a, E: Endpoint + ToHtml + Sync> {
//     endpoint: &'a E,
//     renderer: &'a SimpleIndexRenderer,
// }
// impl<'a, E: Endpoint + ToHtml + Sync> SimpleIndexWarp<'a, E> {
//     fn route(&self) -> warp::filters::BoxedFilter<(String,)> {
//         warp::path("foo").map(|| self.endpoint.to_html()).boxed()
//     }
// }
