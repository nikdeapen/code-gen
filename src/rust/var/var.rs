use crate::rust::{RustType, WithTypeTag};
use crate::{CodeBuffer, Expression, WithName};

/// A name with an associated type tag.
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct Var {
    name: String,
    type_tag: RustType,
}

impl<S: Into<String>, T: Into<RustType>> From<(S, T)> for Var {
    fn from(tuple: (S, T)) -> Self {
        Self {
            name: tuple.0.into(),
            type_tag: tuple.1.into(),
        }
    }
}

impl WithName for Var {
    fn name(&self) -> &str {
        self.name.as_str()
    }
}

impl WithTypeTag for Var {
    fn type_tag(&self) -> &RustType {
        &self.type_tag
    }
}

impl Expression for Var {
    fn write(&self, b: &mut CodeBuffer) {
        self.write_name(b);
        b.write(": ");
        self.write_type_tag(b);
    }
}
