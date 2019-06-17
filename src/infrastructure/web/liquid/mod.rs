//! # Liquid templating infrastructure

use liquid::value::{Object, Value};
use liquid::{Parser, Template};

use crate::domain::model::python_package::PythonPackageMetadata;
use crate::interface::web::python_simple::SimpleIndex;
use crate::interface::web::traits::Renderer;

pub struct SimpleIndexRendererLiquid<'a> {
    template: &'a Template,
}
impl<'a> SimpleIndexRendererLiquid<'a> {
    pub fn new(template: &'a Template) -> Self {
        Self { template }
    }
}
impl<'a> Renderer<SimpleIndex<'a>> for SimpleIndexRendererLiquid<'a> {
    fn render(&self, simple_index: &SimpleIndex) -> String {
        let globals: Object = simple_index.into();
        self.template.render(&globals).unwrap()
    }
}

/// Convert a SimpleIndex struct into a Map of values for templating.
impl<'a> From<&SimpleIndex<'a>> for Object {
    fn from(simple_index: &SimpleIndex) -> Self {
        let mut values = Object::new();
        values.insert(
            "packages".into(),
            Value::scalar(simple_index.package_links()),
        );
        values
    }
}

#[cfg(test)]
mod test {
    use liquid::ParserBuilder;

    use super::*;

    #[test]
    /// We can render packages into a template.
    fn render_simple_index() {
        let parser = ParserBuilder::new().build().unwrap();
        let template = parser.parse("{{ packages }}").unwrap();

        let renderer = SimpleIndexRendererLiquid::new(&template);

        let packages = vec![
            PythonPackageMetadata::new("foo", "1.2", "fs://foo"),
            PythonPackageMetadata::new("bar", "2.1", "fs::/bar"),
        ];
        let index = SimpleIndex::new(&renderer, &packages);

        let rendered = renderer.render(&index);

        assert_eq!(rendered, index.package_links());
    }
}
