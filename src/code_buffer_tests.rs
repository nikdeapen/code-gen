use crate::CodeBuffer;

#[test]
fn write_peek_export() {
    let mut b: CodeBuffer = CodeBuffer::default();

    b.indent(1);
    b.write("line 1");
    b.end_line();
    b.line(2, "line 2");
    b.space();

    let expected: &str = "\tline 1\n\t\tline 2\n ";
    assert_eq!(b.peek(), expected);
    assert_eq!(b.export(), expected.to_string());
}

#[test]
fn clear() {
    let mut b: CodeBuffer = CodeBuffer::default();
    b.write("the code");
    assert_eq!(b.peek(), "the code");
    b.clear();
    assert_eq!(b.peek(), "");
}
