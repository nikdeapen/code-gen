use crate::{CodeBuffer, Expression};

/// A reference lifetime.
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, Default)]
pub enum Lifetime {
    #[default]
    None,
    Static,
    Named(char),
}

/// A reference to a type.
///
/// # Default
/// The default reference is `&`; a shared reference with no lifetime.
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, Default)]
pub struct Reference {
    mutable: bool,
    lifetime: Lifetime,
}

impl Reference {
    //! Mutations

    /// Sets the reference to mutable.
    #[must_use]
    pub fn with_mut(self) -> Self {
        Self {
            mutable: true,
            lifetime: self.lifetime,
        }
    }

    /// Sets the lifetime.
    #[must_use]
    pub fn with_lifetime(self, c: char) -> Option<Self> {
        if c.is_ascii_lowercase() {
            Some(Self {
                mutable: self.mutable,
                lifetime: Lifetime::Named(c),
            })
        } else {
            None
        }
    }

    /// Sets the lifetime to `static`.
    #[must_use]
    pub fn with_static_lifetime(self) -> Self {
        Self {
            mutable: self.mutable,
            lifetime: Lifetime::Static,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_reference() {
        assert_eq!(Reference::default().to_code(), "&");
    }

    #[test]
    fn mut_reference() {
        assert_eq!(Reference::default().with_mut().to_code(), "&mut ");
    }

    #[test]
    fn static_lifetime() {
        assert_eq!(
            Reference::default().with_static_lifetime().to_code(),
            "&'static "
        );
    }

    #[test]
    fn named_lifetime() {
        assert_eq!(
            Reference::default().with_lifetime('a').unwrap().to_code(),
            "&'a "
        );
    }

    #[test]
    fn named_lifetime_rejects_uppercase() {
        assert!(Reference::default().with_lifetime('A').is_none());
    }

    #[test]
    fn mut_with_lifetime() {
        let r = Reference::default()
            .with_mut()
            .with_lifetime('a')
            .unwrap();
        assert_eq!(r.to_code(), "&'a mut ");
    }
}

impl Expression for Reference {
    fn write(&self, b: &mut CodeBuffer) {
        b.push('&');
        match self.lifetime {
            Lifetime::None => {}
            Lifetime::Static => b.write("'static "),
            Lifetime::Named(c) => {
                b.push('\'');
                b.push(c);
                b.push(' ');
            }
        }
        if self.mutable {
            b.write("mut ");
        }
    }
}
