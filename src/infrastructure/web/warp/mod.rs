mod python_simple;
mod server;

use crate::domain::model::python_package::PythonPackageMetadata;
use crate::infrastructure::web::liquid::SimpleIndexRendererLiquid;
use crate::interface::web::python_simple::SimpleIndex;
use crate::interface::web::traits::{GetEndpoint, Templated};

use liquid;
use warp;
use warp::Filter;

pub fn routes() -> warp::filters::BoxedFilter<(impl warp::reply::Reply,)> {
    warp::get2()
        .and(warp::path(SimpleIndex::ENDPOINT).map(|| {
            let parser = liquid::ParserBuilder::new().build().unwrap();
            let template = parser.parse(SimpleIndex::TEMPLATE).unwrap();
            let renderer = SimpleIndexRendererLiquid::new(&template);
            let pkgs = vec![
                PythonPackageMetadata::new("foo", "1.2", "fs://foo"),
                PythonPackageMetadata::new("bar", "1.1", "fs://bar"),
            ];
            let html = SimpleIndex::new(&renderer, &pkgs).get();
            warp::reply::html(html)
        }))
        .boxed()
}
