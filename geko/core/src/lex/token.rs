/// Imports
use common::span::Span;
use std::fmt::Debug;

/// Represents token kind
#[derive(Debug, PartialEq, Clone, Eq, Copy)]
pub enum TokenKind {
    For,       // `for` keyword
    While,     // `while` keyword
    Until,     // `until` keyword
    In,        // `in` keyword
    Use,       // `use` keyword
    Class,     // `class` keyword
    Enum,      // `enum` keyword
    If,        // `if` keyword
    Else,      // `else` keyword
    Return,    // `return` keyword
    Continue,  // `continue` keyword
    Break,     // `break` keyword
    As,        // `as` keyword
    Fun,       // `fun` keyword
    Trait,     // `trait` keyword
    Pick,      // `pick` keyword
    Comma,     // ,
    Dot,       // .
    Lbrace,    // {
    Rbrace,    // }
    Lparen,    // (
    Rparen,    // )
    Lbracket,  // [
    Rbracket,  // ]
    Plus,      // +
    Minus,     // -
    Star,      // *
    Slash,     // /
    Percent,   // %
    Caret,     // ^
    Amp,       // &
    Bang,      // !
    Bar,       // |
    Eq,        // =
    Ge,        // >=
    Le,        // <=
    Gt,        // >
    GtColon,   // >:
    GtBang,    // >!
    Arrow,     // ->
    Lt,        // <
    Colon,     // :
    Walrus,    // :=
    DoubleEq,  // ==
    DoubleBar, // ||
    DoubleAmp, // &&
    BangEq,    // !=
    PlusEq,    // +=
    MinusEq,   // -=
    StarEq,    // *=
    SlashEq,   // /=
    CaretEq,   // ^=
    PercentEq, // %=
    BarEq,     // |=
    AmpEq,     // &=
    DoubleDot, // ..
    Number,    // any number
    String,    // "quoted text"
    Id,        // identifier
    Bool,      // bool
    Null,      // null
}

/// Represents token
#[derive(Debug, PartialEq, Clone, Eq)]
pub struct Token {
    pub span: Span,
    pub kind: TokenKind,
    pub lexeme: String,
}

/// Implementation
impl Token {
    /// Creates new token
    pub fn new(span: Span, kind: TokenKind, lexeme: String) -> Self {
        Self { span, kind, lexeme }
    }
}
