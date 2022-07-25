use crate::{CodeBuffer, Literal};

#[test]
fn write() {
    let literal: Literal = Literal::from("value");
    let result: String = CodeBuffer::display_expression(&literal);
    assert_eq!(result.as_str(), "value");
}
