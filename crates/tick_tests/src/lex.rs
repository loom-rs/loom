/// Imports
use crate::assert_tokens;
use tick_lex::token::TokenKind;

#[test]
pub fn test_lex_1() {
    assert_tokens!(
        "id 12345 \"hello\"",
        &[TokenKind::Id, TokenKind::Number, TokenKind::String]
    );
}

#[test]
pub fn test_lex_2() {
    assert_tokens!(
        "use as for while type fn in if else let return continue break",
        &[
            TokenKind::Use,
            TokenKind::As,
            TokenKind::For,
            TokenKind::While,
            TokenKind::Type,
            TokenKind::Fn,
            TokenKind::In,
            TokenKind::If,
            TokenKind::Else,
            TokenKind::Let,
            TokenKind::Return,
            TokenKind::Continue,
            TokenKind::Break
        ]
    );
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
        "#,
        &[]
    );
}

#[test]
pub fn test_lex_4() {
    assert_tokens!(
        r#"
        , . & && | || ^ !
        + * - += -= *= /= != |= &=
        ^= ; ( ) { } [ ]
        "#,
        &[
            TokenKind::Comma,
            TokenKind::Dot,
            TokenKind::Ampersand,
            TokenKind::DoubleAmp,
            TokenKind::Bar,
            TokenKind::DoubleBar,
            TokenKind::Caret,
            TokenKind::Bang,
            TokenKind::Plus,
            TokenKind::Star,
            TokenKind::Minus,
            TokenKind::PlusEq,
            TokenKind::MinusEq,
            TokenKind::StarEq,
            TokenKind::SlashEq,
            TokenKind::BangEq,
            TokenKind::BarEq,
            TokenKind::AmpersandEq,
            TokenKind::CaretEq,
            TokenKind::Semi,
            TokenKind::Lparen,
            TokenKind::Rparen,
            TokenKind::Lbrace,
            TokenKind::Rbrace,
            TokenKind::Lbracket,
            TokenKind::Rbracket
        ]
    );
}
