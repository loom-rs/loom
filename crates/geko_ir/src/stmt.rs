/// Imports
use crate::{
    atom::{AssignOp, Class, Enum, Function, Trait},
    expr::Expression,
};
use geko_lex::token::Span;

/// Usage
#[derive(Debug, Clone)]
pub enum UseKind {
    // As `name`
    As(String),
    // For `items`
    For(Vec<String>),
    // For every item
    All,
    // Just import
    Just,
}

/// Statement
#[derive(Debug, Clone)]
pub enum Statement {
    // While statement
    While {
        span: Span,
        condition: Expression,
        block: Block,
    },
    // If statement
    If {
        span: Span,
        condition: Expression,
        then: Block,
        else_: Option<Box<Statement>>,
    },
    // For statement
    For {
        span: Span,
        var: String,
        iterable: Expression,
        block: Block,
    },
    // Class declaration
    Class(Class),
    // Enum declaration
    Enum(Enum),
    // Trait declaration
    Trait(Trait),
    // Function declaration
    Function(Function),
    // Assignment declaration
    Assign {
        span: Span,
        name: String,
        op: AssignOp,
        value: Expression,
    },
    // Field assignment declaration
    Set {
        span: Span,
        container: Expression,
        name: String,
        op: AssignOp,
        value: Expression,
    },
    // Return statement
    Return {
        span: Span,
        expr: Option<Expression>,
    },
    // Continue statement
    Continue(Span),
    // Break statement
    Break(Span),
    // Expr
    Expr(Expression),
    // Block
    Block(Box<Block>),
    // Use statement
    Use {
        span: Span,
        path: String,
        kind: UseKind,
    },
}

/// Represents block
#[derive(Debug, Clone)]
pub struct Block {
    pub span: Span,
    pub statements: Vec<Statement>,
}
