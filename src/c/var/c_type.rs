use crate::c::CType::{Array, Const, Function, Named, Pointer};
use crate::c::{Var, WithVarParams};
use crate::{CodeBuffer, Expression};
use std::fmt::{Display, Formatter};

/// A C type.
///
/// # Note
/// - A type is written around the name it declares. (ex: `uint8_t id[16]`)
/// - Adding a parameter converts a type to a function type with itself as the result.
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum CType {
    /// A named type, written as it is. (ex: `uint32_t`, `struct point`, `const char *`)
    Named(String),

    /// A `const` type.
    Const(Box<CType>),

    /// A pointer type.
    Pointer(Box<CType>),

    /// An array type. The `len` is a constant expression. (ex: `16`, `POINT_ENCODED_LEN`)
    Array { base: Box<CType>, len: String },

    /// A function type. A pointer to it is a function pointer.
    Function {
        result: Box<CType>,
        params: Vec<Var>,
    },
}

impl<S: Into<String>> From<S> for CType {
    fn from(name: S) -> Self {
        Named(name.into())
    }
}

impl CType {
    //! Conversions

    /// Converts the type to a `const` type of itself.
    #[must_use]
    pub fn to_const(self) -> Self {
        Const(Box::new(self))
    }

    /// Converts the type to a pointer type of itself.
    #[must_use]
    pub fn to_pointer(self) -> Self {
        Pointer(Box::new(self))
    }

    /// Converts the type to an array type of itself.
    #[must_use]
    pub fn to_array<S>(self, len: S) -> Self
    where
        S: Into<String>,
    {
        Array {
            base: Box::new(self),
            len: len.into(),
        }
    }

    /// Converts the type to a function type with itself as the result.
    #[must_use]
    pub fn to_function(self) -> Self {
        Function {
            result: Box::new(self),
            params: Vec::default(),
        }
    }
}

impl WithVarParams for CType {
    fn params(&self) -> &[Var] {
        match self {
            Function { params, .. } => params.as_slice(),
            _ => &[],
        }
    }

    fn add_param<V>(&mut self, param: V)
    where
        V: Into<Var>,
    {
        if let Function { params, .. } = self {
            params.push(param.into());
        } else {
            let result: CType = std::mem::replace(self, Named(String::default()));
            *self = Function {
                result: Box::new(result),
                params: vec![param.into()],
            };
        }
    }
}

impl CType {
    //! Declarations

    /// Writes the part of the type before the name it declares. (ex: the `char *` of `char *s`)
    pub(in crate::c) fn write_before_name(&self, b: &mut CodeBuffer) {
        self.write_prefix(b, false);
        self.write_space(b);
    }

    /// Writes the part of the type after the name it declares. (ex: the `[4]` of `char s[4]`)
    pub(in crate::c) fn write_after_name(&self, b: &mut CodeBuffer) {
        match self {
            Named(_) => {}
            Const(base) => base.write_after_name(b),
            Pointer(base) => {
                if base.is_suffixed() {
                    b.push(')');
                }
                base.write_after_name(b);
            }
            Array { base, len } => {
                b.push('[');
                b.write(len.as_str());
                b.push(']');
                base.write_after_name(b);
            }
            Function { result, .. } => {
                b.push('(');
                self.write_params(b);
                b.push(')');
                result.write_after_name(b);
            }
        }
    }

    /// Writes the part of the type before a name, without the space between them.
    ///
    /// # Note
    /// - A `const` goes before a named type & after the `*` of a pointer type.
    /// - A `const` array is an array of `const` elements.
    /// - A function type can't be `const`.
    fn write_prefix(&self, b: &mut CodeBuffer, is_const: bool) {
        match self {
            Named(name) => {
                if is_const {
                    b.write("const ");
                }
                b.write(name.as_str());
            }
            Const(base) => base.write_prefix(b, true),
            Pointer(base) => {
                base.write_prefix(b, false);
                self.write_space(b);
                if base.is_suffixed() {
                    b.push('(');
                }
                b.push('*');
                if is_const {
                    b.write("const");
                }
            }
            Array { base, .. } => base.write_prefix(b, is_const),
            Function { result, .. } => result.write_prefix(b, false),
        }
    }

    /// Writes a space, unless it is after a `*`. (ex: `char *name`, not `char * name`)
    fn write_space(&self, b: &mut CodeBuffer) {
        if !b.peek().ends_with('*') {
            b.space();
        }
    }

    /// Checks if the type has a part after the name it declares.
    ///
    /// # Note
    /// - That part binds before a `*`.
    /// - A pointer to the type is written in brackets. (ex: `(*rows)[4]`)
    fn is_suffixed(&self) -> bool {
        match self {
            Named(_) | Pointer(_) => false,
            Const(base) => base.is_suffixed(),
            Array { .. } | Function { .. } => true,
        }
    }
}

impl Expression for CType {
    fn write(&self, b: &mut CodeBuffer) {
        self.write_prefix(b, false);
        self.write_after_name(b);
    }
}

impl Display for CType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.to_code().as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn named_type() {
        assert_eq!(CType::from("uint32_t").to_code(), "uint32_t");
        assert_eq!(CType::from("struct point").to_code(), "struct point");
    }

    #[test]
    fn const_type() {
        let t: CType = CType::from("char").to_const();
        assert_eq!(t.to_code(), "const char");
    }

    #[test]
    fn const_type_twice() {
        let t: CType = CType::from("char").to_const().to_const();
        assert_eq!(t.to_code(), "const char");
    }

    #[test]
    fn pointer_type() {
        let t: CType = CType::from("char").to_pointer();
        assert_eq!(t.to_code(), "char *");
    }

    #[test]
    fn pointer_to_const_type() {
        let t: CType = CType::from("char").to_const().to_pointer();
        assert_eq!(t.to_code(), "const char *");
    }

    #[test]
    fn const_pointer_type() {
        let t: CType = CType::from("char").to_pointer().to_const();
        assert_eq!(t.to_code(), "char *const");
    }

    #[test]
    fn pointer_to_pointer_type() {
        let t: CType = CType::from("char").to_pointer().to_pointer();
        assert_eq!(t.to_code(), "char **");
    }

    #[test]
    fn pointer_to_const_pointer_type() {
        let t: CType = CType::from("char").to_pointer().to_const().to_pointer();
        assert_eq!(t.to_code(), "char *const *");
    }

    #[test]
    fn pointer_to_named_pointer_text() {
        let t: CType = CType::from("const char *").to_pointer();
        assert_eq!(t.to_code(), "const char **");
    }

    #[test]
    fn array_type() {
        let t: CType = CType::from("uint8_t").to_array("16");
        assert_eq!(t.to_code(), "uint8_t[16]");
    }

    #[test]
    fn const_array_type() {
        let t: CType = CType::from("uint8_t").to_array("16").to_const();
        assert_eq!(t.to_code(), "const uint8_t[16]");
    }

    #[test]
    fn pointer_to_array_type() {
        let t: CType = CType::from("uint8_t").to_array("16").to_pointer();
        assert_eq!(t.to_code(), "uint8_t (*)[16]");
    }

    #[test]
    fn pointer_to_const_array_type() {
        let t: CType = CType::from("uint8_t")
            .to_array("16")
            .to_const()
            .to_pointer();
        assert_eq!(t.to_code(), "const uint8_t (*)[16]");
    }

    #[test]
    fn function_type_without_params() {
        let t: CType = CType::from("void").to_function();
        assert!(t.params().is_empty());
        assert_eq!(t.to_code(), "void(void)");
    }

    #[test]
    fn function_type_with_params() {
        let t: CType = CType::from("bool")
            .with_param(("a", "int32_t"))
            .with_param(("b", "int32_t"));
        assert_eq!(t.params().len(), 2);
        assert_eq!(t.to_code(), "bool(int32_t a, int32_t b)");
    }

    #[test]
    fn const_function_type() {
        let t: CType = CType::from("void").to_function().to_const();
        assert_eq!(t.to_code(), "void(void)");
    }

    #[test]
    fn function_pointer_type() {
        let t: CType = CType::from("int32_t")
            .with_param(("self", "const void *"))
            .to_pointer();
        assert_eq!(t.to_code(), "int32_t (*)(const void *self)");
    }

    #[test]
    fn const_function_pointer_type() {
        let t: CType = CType::from("void").to_function().to_pointer().to_const();
        assert_eq!(t.to_code(), "void (*const)(void)");
    }

    #[test]
    fn params_of_a_type_that_is_not_a_function() {
        assert!(CType::from("int32_t").params().is_empty());
    }

    #[test]
    fn display_matches_to_code() {
        let t: CType = CType::from("uint8_t").to_const().to_pointer();
        assert_eq!(format!("{t}"), t.to_code());
    }
}
