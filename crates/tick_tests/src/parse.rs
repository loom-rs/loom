/// Imports
use crate::assert_ast;

#[test]
fn test_parse_1() {
    assert_ast!(
        r#"
        let id = value;
        id = value;
        "#
    )
}

#[test]
fn test_parse_2() {
    assert_ast!(
        r#"
        id += value;
        id -= value;
        id *= value;
        id /= value;
        id %= value;
        id &= value;
        id |= value;
        "#
    )
}

#[test]
fn test_parse_3() {
    assert_ast!(
        r#"
        let value = ((((2 + 2) * 2 / 2) % 3) ^ 65) == true & false && false || true & false;
        "#
    )
}
