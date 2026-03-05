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
                    b.push('(');
                    first.write(b);
                    for var in rest {
                        b.write(", ");
                        var.write(b);
                    }
                    b.push(')');
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_fields() {
        assert_eq!(EnumFields::Empty.to_code(), "");
    }

    #[test]
    fn unnamed_single() {
        let f = EnumFields::Unnamed(vec![RustType::from("u32")]);
        assert_eq!(f.to_code(), "(u32)");
    }

    #[test]
    fn unnamed_multiple() {
        let f = EnumFields::Unnamed(vec![RustType::from("u32"), RustType::from("String")]);
        assert_eq!(f.to_code(), "(u32, String)");
    }

    #[test]
    fn named_single() {
        let f = EnumFields::Named(vec![Var::from(("x", "u32"))]);
        assert_eq!(f.to_code(), "{ x: u32 }");
    }

    #[test]
    fn named_multiple() {
        let f = EnumFields::Named(vec![Var::from(("x", "u32")), Var::from(("y", "u32"))]);
        assert_eq!(f.to_code(), "{ x: u32, y: u32 }");
    }
}
