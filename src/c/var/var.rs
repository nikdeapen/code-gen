use crate::c::{CType, WithCType};
use crate::{CodeBuffer, Expression, WithName};

/// A name with an associated C type.
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct Var {
    name: String,
    c_type: CType,
}

impl<S: Into<String>, T: Into<CType>> From<(S, T)> for Var {
    fn from(tuple: (S, T)) -> Self {
        Self {
            name: tuple.0.into(),
            c_type: tuple.1.into(),
        }
    }
}

impl Var {
    //! Const

    /// Converts the variable to a `const` variable of itself.
    pub(in crate::c) fn into_const(self) -> Self {
        Self {
            name: self.name,
            c_type: self.c_type.to_const(),
        }
    }
}

impl WithName for Var {
    fn name(&self) -> &str {
        self.name.as_str()
    }
}

impl WithCType for Var {
    fn c_type(&self) -> &CType {
        &self.c_type
    }
}

impl Expression for Var {
    fn write(&self, b: &mut CodeBuffer) {
        self.c_type.write_before_name(b);
        self.write_name(b);
        self.c_type.write_after_name(b);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::c::WithVarParams;

    #[test]
    fn named_var() {
        let v: Var = Var::from(("count", "uint32_t"));
        assert_eq!(v.to_code(), "uint32_t count");
    }

    #[test]
    fn pointer_var() {
        let v: Var = Var::from(("data", CType::from("uint8_t").to_pointer()));
        assert_eq!(v.to_code(), "uint8_t *data");
    }

    #[test]
    fn pointer_to_const_var() {
        let v: Var = Var::from(("name", CType::from("char").to_const().to_pointer()));
        assert_eq!(v.to_code(), "const char *name");
    }

    #[test]
    fn const_pointer_var() {
        let v: Var = Var::from(("name", CType::from("char").to_pointer().to_const()));
        assert_eq!(v.to_code(), "char *const name");
    }

    #[test]
    fn named_pointer_text_var() {
        let v: Var = Var::from(("name", "const char *"));
        assert_eq!(v.to_code(), "const char *name");
    }

    #[test]
    fn array_var() {
        let v: Var = Var::from(("id", CType::from("uint8_t").to_array("16")));
        assert_eq!(v.to_code(), "uint8_t id[16]");
    }

    #[test]
    fn array_of_arrays_var() {
        let v: Var = Var::from(("rows", CType::from("uint8_t").to_array("4").to_array("2")));
        assert_eq!(v.to_code(), "uint8_t rows[2][4]");
    }

    #[test]
    fn array_of_pointers_var() {
        let name: CType = CType::from("char").to_const().to_pointer();
        let v: Var = Var::from(("names", name.to_array("3")));
        assert_eq!(v.to_code(), "const char *names[3]");
    }

    #[test]
    fn pointer_to_array_var() {
        let v: Var = Var::from(("rows", CType::from("uint8_t").to_array("4").to_pointer()));
        assert_eq!(v.to_code(), "uint8_t (*rows)[4]");
    }

    #[test]
    fn function_pointer_var() {
        let getter: CType = CType::from("int32_t").with_param(("self", "const void *"));
        let v: Var = Var::from(("x", getter.to_pointer()));
        assert_eq!(v.to_code(), "int32_t (*x)(const void *self)");
    }

    #[test]
    fn pointer_to_function_pointer_var() {
        let v: Var = Var::from((
            "next",
            CType::from("void").to_function().to_pointer().to_pointer(),
        ));
        assert_eq!(v.to_code(), "void (**next)(void)");
    }

    #[test]
    fn const_var() {
        let v: Var = Var::from(("name", CType::from("char").to_const().to_pointer()));
        assert_eq!(v.into_const().to_code(), "const char *const name");
    }

    #[test]
    fn const_function_pointer_var() {
        let v: Var = Var::from(("run", CType::from("void").to_function().to_pointer()));
        assert_eq!(v.into_const().to_code(), "void (*const run)(void)");
    }
}
