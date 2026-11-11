use crate::java::JavaType;
use crate::{CodeBuffer, Expression};

/// An element with a Java type.
pub trait WithJavaType {
    /// Gets the Java type.
    fn java_type(&self) -> &JavaType;

    /// Writes the Java type.
    fn write_java_type(&self, b: &mut CodeBuffer) {
        self.java_type().write(b);
    }
}
