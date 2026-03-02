/// Imports
use miette::NamedSource;
use std::{
    fmt::Debug,
    ops::{Add, Range},
    sync::Arc,
};

/// Represents token kind
#[derive(Debug, PartialEq, Clone, Eq)]
pub enum TokenKind {
    For,         // `for` keyword
    While,       // `while` keyword
    In,          // `in` keyword
    Let,         // `let` keyword
    Use,         // `use` keyword
    Type,        // `type` keyword
    If,          // `if` keyword
    Else,        // `else` keyword
    Return,      // `return` keyword
    Continue,    // `continue` keyword
    Break,       // `break` keyword
    As,          // `as` keyword
    Fn,          // `fn` keyword
    Bail,        // `bail` keyword
    Comma,       // ,
    Dot,         // .
    Lbrace,      // {
    Rbrace,      // }
    Lparen,      // (
    Rparen,      // )
    Lbracket,    // [
    Rbracket,    // ]
    Plus,        // +
    Minus,       // -
    Star,        // *
    Slash,       // /
    Percent,     // %
    Caret,       // ^
    Ampersand,   // &
    Bang,        // !
    Bar,         // |
    Eq,          // =
    Ge,          // >=
    Le,          // <=
    Gt,          // >
    Lt,          // <
    Semi,        // ;
    DoubleEq,    // ==
    DoubleBar,   // ||
    DoubleAmp,   // &&
    BangEq,      // !=
    PlusEq,      // +=
    MinusEq,     // -=
    StarEq,      // *=
    SlashEq,     // /=
    CaretEq,     // ^=
    PercentEq,   // %=
    BarEq,       // |=
    AmpersandEq, // &=
    DoubleDot,   // ..
    Number,      // any number
    String,      // "quoted text"
    Id,          // identifier
    Bool,        // bool
    Null,        // null
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

/// Represents span
#[derive(PartialEq, Clone, Eq)]
pub struct Span(pub Arc<NamedSource<String>>, pub Range<usize>);

/// Debug implementation
impl Debug for Span {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Span").field(&self.1).finish()
    }
}

/// Add implementation
impl Add for Span {
    type Output = Span;

    fn add(self, rhs: Self) -> Self::Output {
        // Checking that files are same
        if self.0 != rhs.0 {
            panic!("attemp to perform `+` operation on two spans from different files.")
        }
        // Calculating new span range
        let start = self.1.start.min(rhs.1.start);
        let end = self.1.end.max(rhs.1.end);
        Span(self.0, start..end)
    }
}
