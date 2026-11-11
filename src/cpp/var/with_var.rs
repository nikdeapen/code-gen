use crate::cpp::Var;
use crate::{CodeBuffer, Expression};

/// An element with a variable.
pub trait WithVar {
    /// Gets the variable.
    fn var(&self) -> &Var;

    /// Writes the variable.
    fn write_var(&self, b: &mut CodeBuffer) {
        self.var().write(b);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cpp::Field;

    #[test]
    fn write_var() {
        let field = Field::from(("x_", "std::uint32_t"));
        let mut b = CodeBuffer::default();
        field.write_var(&mut b);
        assert_eq!(b.peek(), "std::uint32_t x_");
    }
}
