//! # Python Simple Index
//!
//! A PEP-0503 compliant simple index for packages, which can be used
//! by `pip` and other compatible package managers.
//!

use crate::domain::model::python_package::PythonPackageMetadata;
use crate::interface::web::traits::{GetEndpoint, Renderer, Templated};

// pub trait SimpleIndexRenderer: Renderer {
//     fn render(&self, simple_index: &SimpleIndex) -> String;
// }

/// The endpoint definition for a SimpleIndex
pub struct SimpleIndex<'a> {
    packages: &'a [PythonPackageMetadata<'a>],
    renderer: &'a Renderer<Self>,
}
impl<'a> SimpleIndex<'a> {
    pub fn new<R: Renderer<Self>>(
        renderer: &'a R,
        packages: &'a [PythonPackageMetadata<'a>],
    ) -> Self {
        Self { packages, renderer }
    }
    /// Get the template block of package links for the index
    pub fn package_links(&self) -> String {
        self.packages
            .iter()
            .map(|pkg| format!("    <a href=\"{}/\">{}</a><br/>\n", pkg.name, pkg.name))
            .collect()
    }
}
/// Implementation of GET for a SimpleIndex ednpoint.
impl<'a> GetEndpoint for SimpleIndex<'a> {
    const ENDPOINT: &'static str = "simple";

    fn get(&self) -> String {
        self.renderer.render(&self)
    }
}
impl<'a> Templated for SimpleIndex<'a> {
    const TEMPLATE: &'static str = include_str!("template/simple_index.html");
}

#[cfg(test)]
pub mod test {
    use super::*;

    struct RendererFake {}
    impl<'a> Renderer<SimpleIndex<'a>> for RendererFake {
        fn render(&self, simple_index: &SimpleIndex) -> String {
            "foo".into()
        }
    }

    #[test]
    /// We can convert an index to a bunch of HTML links.
    fn to_template_block() {
        let packages = vec![
            PythonPackageMetadata::new("foo", "1.2", "fs://foo"),
            PythonPackageMetadata::new("bar", "2.1", "fs::/bar"),
        ];
        let index = SimpleIndex::new(&RendererFake {}, &packages);
        assert_eq!(
            index.package_links(),
            "    <a href=\"foo/\">foo</a><br/>\n    <a href=\"bar/\">bar</a><br/>\n"
        )
    }
}
