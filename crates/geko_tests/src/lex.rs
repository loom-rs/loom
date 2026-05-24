/// Imports
use crate::assert_tokens;

#[test]
pub fn test_lex_1() {
    assert_tokens!("id 12345 \"hello\"");
}

#[test]
pub fn test_lex_2() {
    assert_tokens!("use as for while class fun in if else let return continue break trait pick");
}

#[test]
pub fn test_lex_3() {
    assert_tokens!(
        r#"
        # hello
        #[
            multiline
            comment
        ]#
        "#
    );
}

#[test]
pub fn test_lex_4() {
    assert_tokens!(
        r#"
        , . & && | || ^ !
        + * - += -= *= /= != |= &=
        ^= := ( ) { } [ ]
        "#
    );
}

// Note: should bail
#[test]
pub fn test_lex_5() {
    assert_tokens!(
        r#"
        #[ unterminated comment
        "#
    );
}
