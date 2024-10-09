use crate::rust::{TypeTag, Var};
use crate::{CodeBuffer, Expression};

/// Enum fields.
pub enum EnumFields {
    Empty,
    Named(Vec<Var>),
    Unnamed(Vec<TypeTag>),
}

impl Default for EnumFields {
    fn default() -> Self {
        Self::Empty
    }
}

impl Expression for EnumFields {
    fn write(&self, b: &mut CodeBuffer) {
        match self {
            Self::Empty => {}
            Self::Named(vars) => {
                if let Some((first, rest)) = vars.split_first() {
                    b.write("{ ");
                    first.write(b);
                    for var in rest {
                        b.write(", ");
                        var.write(b);
                    }
                    b.write(" }");
                }
            }
            Self::Unnamed(tags) => {
                if let Some((first, rest)) = tags.split_first() {
                    b.write("(");
                    first.write(b);
                    for var in rest {
                        b.write(", ");
                        var.write(b);
                    }
                    b.write(")");
                }
            }
        }
    }
}
