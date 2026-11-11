use crate::{CodeBuffer, Expression};

/// A keyword modifier of a declaration.
///
/// # Note
/// - The cases are in the order they are written.
/// - The cases up to `Explicit` go before the declaration.
/// - The cases from `Const` go after the parameters of a function.
#[must_use]
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum Modifier {
    Static,
    Inline,
    Constexpr,
    Virtual,
    Explicit,
    Const,
    Noexcept,
    Override,

    /// The `= 0` of a pure virtual function.
    PureVirtual,

    /// The `= default` of a defaulted function.
    Default,
}

impl Modifier {
    //! Properties

    /// Checks if the modifier goes after the parameters of a function.
    pub(crate) const fn is_trailing(self) -> bool {
        matches!(
            self,
            Self::Const | Self::Noexcept | Self::Override | Self::PureVirtual | Self::Default
        )
    }

    /// Checks if the modifier is left off a definition outside the class.
    pub(crate) const fn is_declaration_only(self) -> bool {
        matches!(
            self,
            Self::Static | Self::Virtual | Self::Explicit | Self::Override | Self::PureVirtual
        )
    }
}

impl Expression for Modifier {
    fn write(&self, b: &mut CodeBuffer) {
        let keyword: &str = match self {
            Self::Static => "static",
            Self::Inline => "inline",
            Self::Constexpr => "constexpr",
            Self::Virtual => "virtual",
            Self::Explicit => "explicit",
            Self::Const => "const",
            Self::Noexcept => "noexcept",
            Self::Override => "override",
            Self::PureVirtual => "= 0",
            Self::Default => "= default",
        };
        b.write(keyword);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeSet;

    #[test]
    fn keywords() {
        assert_eq!(Modifier::Static.to_code(), "static");
        assert_eq!(Modifier::Inline.to_code(), "inline");
        assert_eq!(Modifier::Constexpr.to_code(), "constexpr");
        assert_eq!(Modifier::Virtual.to_code(), "virtual");
        assert_eq!(Modifier::Explicit.to_code(), "explicit");
        assert_eq!(Modifier::Const.to_code(), "const");
        assert_eq!(Modifier::Noexcept.to_code(), "noexcept");
        assert_eq!(Modifier::Override.to_code(), "override");
        assert_eq!(Modifier::PureVirtual.to_code(), "= 0");
        assert_eq!(Modifier::Default.to_code(), "= default");
    }

    #[test]
    fn is_trailing() {
        assert!(!Modifier::Static.is_trailing());
        assert!(!Modifier::Explicit.is_trailing());
        assert!(Modifier::Const.is_trailing());
        assert!(Modifier::Default.is_trailing());
    }

    #[test]
    fn is_declaration_only() {
        assert!(Modifier::Static.is_declaration_only());
        assert!(Modifier::Override.is_declaration_only());
        assert!(!Modifier::Constexpr.is_declaration_only());
        assert!(!Modifier::Const.is_declaration_only());
    }

    #[test]
    fn a_set_is_in_the_written_order() {
        let modifiers = BTreeSet::from([
            Modifier::Default,
            Modifier::PureVirtual,
            Modifier::Override,
            Modifier::Noexcept,
            Modifier::Const,
            Modifier::Explicit,
            Modifier::Virtual,
            Modifier::Constexpr,
            Modifier::Inline,
            Modifier::Static,
        ]);
        let written: Vec<String> = modifiers.iter().map(Expression::to_code).collect();
        assert_eq!(
            written.join(" "),
            "static inline constexpr virtual explicit const noexcept override = 0 = default"
        );
    }
}
