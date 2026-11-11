use crate::cpp::{CppType, Modifier, Var, WithModifiers, WithVarParams};
use crate::{CodeBuffer, Expression, WithName};
use std::collections::BTreeSet;

/// A function signature.
///
/// # Note
/// - A function with no result returns `void`.
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct Signature {
    modifiers: BTreeSet<Modifier>,
    result: Option<CppType>,
    name: String,
    params: Vec<Var>,
}

impl<S: Into<String>> From<S> for Signature {
    fn from(name: S) -> Self {
        Self {
            modifiers: BTreeSet::default(),
            result: None,
            name: name.into(),
            params: Vec::default(),
        }
    }
}

impl WithModifiers for Signature {
    fn modifiers(&self) -> &BTreeSet<Modifier> {
        &self.modifiers
    }

    fn add_modifier(&mut self, modifier: Modifier) {
        self.modifiers.insert(modifier);
    }
}

impl Signature {
    //! Result

    /// Gets the optional result.
    #[must_use]
    pub fn result(&self) -> Option<&CppType> {
        self.result.as_ref()
    }

    /// Sets the result.
    pub fn set_result<T>(&mut self, result: T)
    where
        T: Into<CppType>,
    {
        self.result = Some(result.into());
    }

    /// Sets the result.
    #[must_use]
    pub fn with_result<T>(mut self, result: T) -> Self
    where
        T: Into<CppType>,
    {
        self.set_result(result);
        self
    }
}

impl WithName for Signature {
    fn name(&self) -> &str {
        self.name.as_str()
    }
}

impl WithVarParams for Signature {
    fn params(&self) -> &[Var] {
        self.params.as_slice()
    }

    fn add_param<V>(&mut self, param: V)
    where
        V: Into<Var>,
    {
        self.params.push(param.into());
    }
}

impl Signature {
    //! Scope

    /// Converts the signature to the one of its definition outside the class.
    ///
    /// # Note
    /// - The `scope` is the name of the class. (ex: `Point::x` for the `x` of `Point`)
    /// - A modifier that only a declaration inside the class has is dropped. (ex: `static`)
    #[must_use]
    pub fn to_scoped<S>(mut self, scope: S) -> Self
    where
        S: Into<String>,
    {
        self.name = format!("{}::{}", scope.into(), self.name);
        self.modifiers
            .retain(|modifier| !modifier.is_declaration_only());
        self
    }
}

impl Expression for Signature {
    fn write(&self, b: &mut CodeBuffer) {
        self.write_modifiers(b);
        match &self.result {
            Some(result) => result.write(b),
            None => b.write("void"),
        }
        b.space();
        self.write_name(b);
        b.push('(');
        self.write_params(b);
        b.push(')');
        self.write_trailing_modifiers(b);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_signature() {
        let s = Signature::from("foo");
        assert_eq!(s.to_code(), "void foo()");
    }

    #[test]
    fn signature_with_result() {
        let s = Signature::from("is_empty").with_result("bool");
        assert_eq!(s.result(), Some(&CppType::from("bool")));
        assert_eq!(s.to_code(), "bool is_empty()");
    }

    #[test]
    fn signature_with_params() {
        let s = Signature::from("set")
            .with_param(("x", "std::uint32_t"))
            .with_param(("name", CppType::from("std::string").to_const().to_ref()));
        assert_eq!(
            s.to_code(),
            "void set(std::uint32_t x, const std::string& name)"
        );
    }

    #[test]
    fn signature_with_modifiers() {
        let s = Signature::from("x")
            .with_result("std::uint32_t")
            .with_modifier(Modifier::Noexcept)
            .with_modifier(Modifier::Const)
            .with_modifier(Modifier::Constexpr);
        assert_eq!(s.to_code(), "constexpr std::uint32_t x() const noexcept");
    }

    #[test]
    fn pure_virtual_signature() {
        let s = Signature::from("x")
            .with_result("std::uint32_t")
            .with_modifier(Modifier::PureVirtual)
            .with_modifier(Modifier::Const)
            .with_modifier(Modifier::Virtual);
        assert_eq!(s.to_code(), "virtual std::uint32_t x() const = 0");
    }

    #[test]
    fn scoped_signature() {
        let s = Signature::from("x")
            .with_result("std::uint32_t")
            .with_modifier(Modifier::Const)
            .to_scoped("Point");
        assert_eq!(s.name(), "Point::x");
        assert_eq!(s.to_code(), "std::uint32_t Point::x() const");
    }

    #[test]
    fn scoped_signature_drops_declaration_modifiers() {
        let s = Signature::from("x")
            .with_result("std::uint32_t")
            .with_modifier(Modifier::Virtual)
            .with_modifier(Modifier::Const)
            .with_modifier(Modifier::Noexcept)
            .with_modifier(Modifier::Override)
            .to_scoped("Point");
        assert_eq!(s.to_code(), "std::uint32_t Point::x() const noexcept");
        let s = Signature::from("decode")
            .with_result("Point")
            .with_modifier(Modifier::Static)
            .with_modifier(Modifier::Constexpr)
            .to_scoped("Point");
        assert_eq!(s.to_code(), "constexpr Point Point::decode()");
    }
}
