use crate::go::{GoType, Var, WithComments, WithGoType, WithVar};
use crate::{CodeBuffer, Statement, WithName};

/// A type definition. (ex: `type Color uint16`)
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct TypeDef {
    comments: Vec<String>,
    var: Var,
}

impl<V: Into<Var>> From<V> for TypeDef {
    fn from(var: V) -> Self {
        Self {
            comments: Vec::default(),
            var: var.into(),
        }
    }
}

impl WithComments for TypeDef {
    fn comments(&self) -> &[String] {
        self.comments.as_slice()
    }

    fn add_comment<S>(&mut self, comment: S)
    where
        S: Into<String>,
    {
        self.comments.push(comment.into());
    }
}

impl WithName for TypeDef {
    fn name(&self) -> &str {
        self.var.name()
    }
}

impl WithGoType for TypeDef {
    fn go_type(&self) -> &GoType {
        self.var.go_type()
    }
}

impl WithVar for TypeDef {
    fn var(&self) -> &Var {
        &self.var
    }
}

impl Statement for TypeDef {
    fn write(&self, b: &mut CodeBuffer, level: usize) {
        self.write_comments(b, level);
        b.indent(level);
        b.write("type ");
        self.write_var(b);
        b.end_line();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn type_def() {
        let t = TypeDef::from(("Color", "uint16"));
        assert_eq!(t.to_code(), "type Color uint16\n");
    }

    #[test]
    fn type_def_with_comment() {
        let t =
            TypeDef::from(("Id", GoType::from("byte").to_array(16))).with_comment(" Id is a UUID.");
        assert_eq!(t.to_code(), "// Id is a UUID.\ntype Id [16]byte\n");
    }
}
