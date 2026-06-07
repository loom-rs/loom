/// Imports
use common::span::Span;
use std::fmt::Display;

/// Assignment operator
#[derive(Debug, Clone, Copy)]
pub enum AssignOp {
    Define, // :=
    Assign, // =
    Add,    // +=
    Sub,    // -=
    Mul,    // *=
    Div,    // /=
    Mod,    // %=
    BitAnd, // &=
    BitOr,  // |=
    Xor,    // ^=
}

/// Binary operator
#[derive(Debug, Clone, Copy)]
pub enum BinOp {
    Add,      // +
    Sub,      // -
    Mul,      // *
    Div,      // /
    Mod,      // %
    And,      // &&
    Or,       // ||
    Gt,       // >
    Ge,       // >=
    Lt,       // <
    Le,       // <=
    Eq,       // ==
    Ne,       // !=
    BitAnd,   // &
    BitOr,    // |
    Xor,      // ^
    Impls,    // >:
    NotImpls, // >!
}

/// Display implementation
impl Display for BinOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BinOp::Add => write!(f, "+"),
            BinOp::Sub => write!(f, "-"),
            BinOp::Mul => write!(f, "*"),
            BinOp::Div => write!(f, "/"),
            BinOp::Mod => write!(f, "%"),
            BinOp::And => write!(f, "&&"),
            BinOp::Or => write!(f, "||"),
            BinOp::Gt => write!(f, ">"),
            BinOp::Ge => write!(f, ">="),
            BinOp::Lt => write!(f, "<"),
            BinOp::Le => write!(f, "<="),
            BinOp::Eq => write!(f, "=="),
            BinOp::Ne => write!(f, "!="),
            BinOp::BitAnd => write!(f, "&"),
            BinOp::BitOr => write!(f, "|"),
            BinOp::Xor => write!(f, "^"),
            BinOp::Impls => write!(f, ">:"),
            BinOp::NotImpls => write!(f, ">!"),
        }
    }
}

/// Unary operator
#[derive(Debug, Clone, Copy)]
pub enum UnaryOp {
    Neg,  // -
    Bang, // !
}

/// Display implementation
impl Display for UnaryOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UnaryOp::Neg => write!(f, "-"),
            UnaryOp::Bang => write!(f, "!"),
        }
    }
}

/// Defines literal
#[derive(Debug, Clone)]
pub enum Lit {
    /// Number literal
    Number(String),
    /// String literal
    String(String),
    /// Bool literal
    Bool(String),
    /// Null literal
    Null,
}

/// Represents function
#[derive(Debug, Clone)]
pub struct Function {
    pub span: Span,
    pub sign_span: Span,
    pub name: String,
    pub params: Vec<String>,
    pub block: Block,
}

/// Represents trait function
#[derive(Debug, Clone)]
pub struct TraitFunction {
    pub span: Span,
    pub name: String,
    pub params: Vec<String>,
}

/// Represents trait
#[derive(Debug, Clone)]
pub struct Trait {
    pub span: Span,
    pub name: String,
    pub functions: Vec<TraitFunction>,
}

/// Represents class
#[derive(Debug, Clone)]
pub struct Class {
    pub span: Span,
    pub name_span: Span,
    pub name: String,
    pub methods: Vec<Function>,
}

/// Represents enum
#[derive(Debug, Clone)]
pub struct Enum {
    pub span: Span,
    pub name_span: Span,
    pub name: String,
    pub variants: Vec<String>,
}

/// Defines an expression
#[derive(Debug, Clone)]
pub enum Expr {
    // Literal
    Lit {
        span: Span,
        lit: Lit,
    },
    // Binary operation
    Bin {
        span: Span,
        op: BinOp,
        lhs: Box<Expr>,
        rhs: Box<Expr>,
    },
    // Unary operation
    Unary {
        span: Span,
        op: UnaryOp,
        value: Box<Expr>,
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
        container: Box<Expr>,
    },
    // Call expression
    Call {
        span: Span,
        args: Vec<Expr>,
        what: Box<Expr>,
    },
    /// List expression
    List {
        span: Span,
        list: Vec<Expr>,
    },
    /// Dict expression
    Dict {
        span: Span,
        dict: Vec<(Expr, Expr)>,
    },
    /// Anonymous function expression
    Fun {
        span: Span,
        params: Vec<String>,
        block: Block,
    },
    /// Range expression
    Range {
        span: Span,
        lhs: Box<Expr>,
        rhs: Box<Expr>,
        includes_end: bool,
    },
}

/// Defines use of module
#[derive(Debug, Clone)]
pub enum UseKind {
    // As `name`
    As(String),
    // Pick `items`
    Pick(Vec<String>),
    // For every item
    All,
    // Just import
    Just,
}

/// Defines statement
#[derive(Debug, Clone)]
pub enum Stmt {
    // While statement
    While {
        span: Span,
        condition: Expr,
        block: Block,
    },
    // If statement
    If {
        span: Span,
        condition: Expr,
        then: Block,
        else_: Option<Box<Stmt>>,
    },
    // For statement
    For {
        span: Span,
        var: String,
        iterable: Expr,
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
        value: Expr,
    },
    // Field assignment declaration
    Set {
        span: Span,
        container: Expr,
        name: String,
        op: AssignOp,
        value: Expr,
    },
    // Return statement
    Return {
        span: Span,
        expr: Option<Expr>,
    },
    // Continue statement
    Continue(Span),
    // Break statement
    Break(Span),
    // Expr
    Expr(Expr),
    // Scope block
    Scope(Box<Block>),
    // Use statement
    Use {
        span: Span,
        path: String,
        kind: UseKind,
    },
}

/// Defines block of statements
#[derive(Debug, Clone)]
pub struct Block {
    pub span: Span,
    pub stmts: Vec<Stmt>,
}
