use crate::{CodeBuffer, ExpressionStatement, Literal};

#[test]
fn display() {
    let es: ExpressionStatement<Literal> = ExpressionStatement::from(Literal::from("value"));
    let result: String = CodeBuffer::display_statement(&es);
    assert_eq!(result.as_str(), "value\n");
}
