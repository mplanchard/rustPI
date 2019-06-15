//! # Liquid templating infrastructure

use liquid::value::{Object, Value};
use liquid::{Parser, Template};

use crate::domain::model::python_package::PythonPackageMetadata;
use crate::interface::python_simple::{SimpleIndex, SimpleIndexRenderer};

pub struct SimpleIndexRendererLiquid<'a> {
    parser: &'a Parser,
    template: &'a Template,
}
impl<'a> SimpleIndexRenderer for SimpleIndexRendererLiquid<'a> {
    fn render(&self, simple_index: &SimpleIndex) -> String {
        let globals: Object = simple_index.into();
        self.template.render(&globals).unwrap()
    }
}
impl<'a> SimpleIndexRendererLiquid<'a> {
    fn new(parser: &'a Parser, template: &'a Template) -> Self {
        Self { parser, template }
    }
}

/// Convert a SimpleIndex struct into a Map of values for templating.
impl<'a> From<&SimpleIndex<'a>> for Object {
    fn from(simple_index: &SimpleIndex) -> Self {
        let mut values = Object::new();
        values.insert(
            "packages".into(),
            Value::scalar(simple_index.to_template_block()),
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

        let renderer = SimpleIndexRendererLiquid::new(&parser, &template);

        let packages = vec![
            PythonPackageMetadata::new("foo", "1.2", "fs://foo"),
            PythonPackageMetadata::new("bar", "2.1", "fs::/bar"),
        ];
        let index = SimpleIndex::new(&renderer, &packages);

        let rendered = renderer.render(&index);

        assert_eq!(rendered, index.to_template_block());
    }
}
