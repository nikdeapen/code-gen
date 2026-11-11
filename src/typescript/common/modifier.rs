use crate::{CodeBuffer, Expression};

/// A keyword modifier of a declaration.
///
/// # Note
/// - The modifiers are in the order TypeScript requires. An element writes them in that order.
#[must_use]
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum Modifier {
    Export,
    Static,
    Override,
    Readonly,
    Async,

    /// Makes a function of a class a getter. (ex: `get x(): number`)
    Get,

    /// Makes a function of a class a setter. (ex: `set x(value: number)`)
    Set,
}

impl Expression for Modifier {
    fn write(&self, b: &mut CodeBuffer) {
        let keyword: &str = match self {
            Self::Export => "export",
            Self::Static => "static",
            Self::Override => "override",
            Self::Readonly => "readonly",
            Self::Async => "async",
            Self::Get => "get",
            Self::Set => "set",
        };
        b.write(keyword);
        b.space();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keywords() {
        assert_eq!(Modifier::Export.to_code(), "export ");
        assert_eq!(Modifier::Static.to_code(), "static ");
        assert_eq!(Modifier::Override.to_code(), "override ");
        assert_eq!(Modifier::Readonly.to_code(), "readonly ");
        assert_eq!(Modifier::Async.to_code(), "async ");
        assert_eq!(Modifier::Get.to_code(), "get ");
        assert_eq!(Modifier::Set.to_code(), "set ");
    }
}
