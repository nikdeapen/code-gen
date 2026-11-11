use crate::go::Import;
use crate::{CodeBuffer, Expression, Statement};

/// An `import` block.
///
/// # Note
/// - A block with no imports writes nothing.
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, Default)]
pub struct ImportBlock {
    imports: Vec<Import>,
}

impl ImportBlock {
    //! Imports

    /// Gets the imports.
    #[must_use]
    pub fn imports(&self) -> &[Import] {
        self.imports.as_slice()
    }

    /// Adds the import.
    pub fn add_import<I>(&mut self, import: I)
    where
        I: Into<Import>,
    {
        self.imports.push(import.into());
    }

    /// Adds the import.
    #[must_use]
    pub fn with_import<I>(mut self, import: I) -> Self
    where
        I: Into<Import>,
    {
        self.add_import(import);
        self
    }
}

impl Statement for ImportBlock {
    fn write(&self, b: &mut CodeBuffer, level: usize) {
        if !self.imports.is_empty() {
            b.line(level, "import (");
            for import in &self.imports {
                b.indent(level + 1);
                import.write(b);
                b.end_line();
            }
            b.line(level, ")");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_import_block() {
        let i = ImportBlock::default();
        assert_eq!(i.to_code(), "");
    }

    #[test]
    fn import_block() {
        let i = ImportBlock::default()
            .with_import("errors")
            .with_import(Import::from("example.com/protopacket").with_alias("pp"));
        assert_eq!(
            i.to_code(),
            "import (\n    \"errors\"\n    pp \"example.com/protopacket\"\n)\n"
        );
    }
}
