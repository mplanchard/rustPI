//! # Python Simple Index
//!
//! A PEP-0503 compliant simple index for packages, which can be used
//! by `pip` and other compatible package managers.
//!

use crate::domain::model::python_package::PythonPackageMetadata;

pub trait ToHtml {
    fn to_html(&self) -> String;
}

pub trait SimpleIndexRenderer {
    fn render(&self, simple_index: &SimpleIndex) -> String;
}

pub struct SimpleIndex<'a> {
    pub packages: &'a [PythonPackageMetadata<'a>],
    renderer: &'a SimpleIndexRenderer,
}
impl<'a> ToHtml for SimpleIndex<'a> {
    fn to_html(&self) -> String {
        self.renderer.render(&self)
    }
}
impl<'a> SimpleIndex<'a> {
    pub fn new<R: SimpleIndexRenderer>(
        renderer: &'a R,
        packages: &'a [PythonPackageMetadata<'a>],
    ) -> Self {
        Self { packages, renderer }
    }
    pub fn to_template_block(&self) -> String {
        let lines = self
            .packages
            .iter()
            .map(|pkg| format!("<a href=\"{}/\">{}</a><br/>\n", pkg.name, pkg.name));
        lines.collect()
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    struct RendererFake {}
    impl SimpleIndexRenderer for RendererFake {
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
            index.to_template_block(),
            "<a href=\"foo/\">foo</a><br/>\n<a href=\"bar/\">bar</a><br/>\n"
        )
    }
}
