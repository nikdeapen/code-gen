use crate::{CodeBuffer, Expression};

/// A function receiver.
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum Receiver {
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
