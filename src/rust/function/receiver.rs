use crate::{CodeBuffer, Expression};

/// A function receiver.
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, Default)]
pub enum Receiver {
    #[default]
    Borrowed,
    BorrowedMut,
    Owned,
    OwnedMut,
}

impl Expression for Receiver {
    fn write(&self, b: &mut CodeBuffer) {
        let s: &str = match self {
            Self::Borrowed => "&self",
            Self::BorrowedMut => "&mut self",
            Self::Owned => "self",
            Self::OwnedMut => "mut self",
        };
        b.write(s);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn borrowed() {
        assert_eq!(Receiver::Borrowed.to_code(), "&self");
    }

    #[test]
    fn borrowed_mut() {
        assert_eq!(Receiver::BorrowedMut.to_code(), "&mut self");
    }

    #[test]
    fn owned() {
        assert_eq!(Receiver::Owned.to_code(), "self");
    }

    #[test]
    fn owned_mut() {
        assert_eq!(Receiver::OwnedMut.to_code(), "mut self");
    }

    #[test]
    fn default_is_borrowed() {
        assert_eq!(Receiver::default(), Receiver::Borrowed);
    }
}
