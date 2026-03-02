/// Imports
use crate::atom::{BinaryOp, Lit, UnaryOp};
use tick_lex::token::Span;

/// Range
#[derive(Debug, Clone)]
pub enum Range {
    // x..y
    IncludeLast {
        span: Span,
        from: Expression,
        to: Expression,
    },
    // x..=y
    ExcludeLast {
        span: Span,
        from: Expression,
        to: Expression,
    },
}

/// Expression
#[derive(Debug, Clone)]
pub enum Expression {
    // Literal
    Lit {
        span: Span,
        lit: Lit,
    },
    // Binary operation
    Bin {
        span: Span,
        op: BinaryOp,
        left: Box<Expression>,
        right: Box<Expression>,
    },
    // Unary operation
    Unary {
        span: Span,
        op: UnaryOp,
        value: Box<Expression>,
    },
    // Variable access
    Variable {
        span: Span,
        name: String,
    },
    // Field access
    Field {
        span: Span,
        name: String,
        container: Box<Expression>,
    },
    // Call expression
    Call {
        span: Span,
        args: Vec<Expression>,
        what: Box<Expression>,
    },
    /// List expression
    List {
        span: Span,
        list: Vec<Expression>,
    },
}
