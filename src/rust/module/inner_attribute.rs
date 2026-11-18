use crate::{CodeBuffer, Statement};

/// An inner attribute, which applies to the module or the crate it is written in.
/// (ex: `#![allow(dead_code)]`)
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct InnerAttribute {
    attribute: String,
}

impl<S: Into<String>> From<S> for InnerAttribute {
    fn from(attribute: S) -> Self {
        Self {
            attribute: attribute.into(),
        }
    }
}

impl InnerAttribute {
    //! Properties

    /// Gets the attribute, without its brackets. (ex: `allow(dead_code)`)
    #[must_use]
    pub fn attribute(&self) -> &str {
        self.attribute.as_str()
    }
}

impl Statement for InnerAttribute {
    fn write(&self, b: &mut CodeBuffer, level: usize) {
        b.indent(level);
        b.write("#![");
        b.write(self.attribute.as_str());
        b.push(']');
        b.end_line();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rust::ModBlock;
    use crate::{Block, WithStatements};

    #[test]
    fn inner_attribute() {
        let a: InnerAttribute = InnerAttribute::from("allow(non_camel_case_types)");
        assert_eq!(a.attribute(), "allow(non_camel_case_types)");
        assert_eq!(a.to_code(), "#![allow(non_camel_case_types)]\n");
    }

    #[test]
    fn inner_attribute_at_the_top_of_a_file() {
        let file: Block = Block::default()
            .with_statement(InnerAttribute::from(String::from("allow(dead_code)")))
            .with_empty_line()
            .with_semi("mod lexer");
        assert_eq!(file.to_code(), "#![allow(dead_code)]\n\nmod lexer;\n");
    }

    #[test]
    fn inner_attribute_in_a_mod_block() {
        let m: ModBlock =
            ModBlock::from("tests").with_statement(InnerAttribute::from("allow(dead_code)"));
        assert_eq!(m.to_code(), "mod tests {\n    #![allow(dead_code)]\n}\n");
    }
}
