use crate::rust::{RustType, Var};
use crate::{CodeBuffer, Expression};

/// Enum fields.
#[derive(Default)]
pub enum EnumFields {
    #[default]
    Empty,
    Named(Vec<Var>),
    Unnamed(Vec<RustType>),
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
