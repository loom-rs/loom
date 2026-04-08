/// Imports
use crate::{
    atom::{BinOp, Lit, UnaryOp},
    stmt::Block,
};
use squirrel_lex::token::Span;

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
        op: BinOp,
        lhs: Box<Expression>,
        rhs: Box<Expression>,
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
    /// Dict expression
    Dict {
        span: Span,
        dict: Vec<(Expression, Expression)>,
    },
    /// Anonymous function expression
    Fn {
        span: Span,
        params: Vec<String>,
        block: Block,
    },
    /// Range expression
    Range {
        span: Span,
        lhs: Box<Expression>,
        rhs: Box<Expression>,
        includes_end: bool,
    },
}
