use crate::cpp::CppType;
use crate::{CodeBuffer, Expression};

/// An element with a C++ type.
pub trait WithCppType {
    /// Gets the C++ type.
    fn cpp_type(&self) -> &CppType;

    /// Writes the C++ type.
    fn write_cpp_type(&self, b: &mut CodeBuffer) {
        self.cpp_type().write(b);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cpp::Var;

    #[test]
    fn write_cpp_type() {
        let var = Var::from(("count", "std::uint32_t"));
        let mut b = CodeBuffer::default();
        var.write_cpp_type(&mut b);
        assert_eq!(b.peek(), "std::uint32_t");
    }
}
