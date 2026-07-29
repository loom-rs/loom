/// Imports
use common::span::Span;
use std::fmt::Display;

/// Defines assignment operator
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

/// Defines binary operator
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

/// Defines unary operator
#[derive(Debug, Clone, Copy)]
pub enum UnOp {
    Neg,  // -
    Bang, // !
}

/// Display implementation
impl Display for UnOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UnOp::Neg => write!(f, "-"),
            UnOp::Bang => write!(f, "!"),
        }
    }
}

/// Defines literal
#[derive(Debug, Clone)]
pub enum Lit {
    Number(String),
    String(String),
    Bool(String),
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
    Lit {
        span: Span,
        lit: Lit,
    },
    Bin {
        span: Span,
        op: BinOp,
        lhs: Box<Expr>,
        rhs: Box<Expr>,
    },
    Un {
        span: Span,
        op: UnOp,
        value: Box<Expr>,
    },
    Assign {
        span: Span,
        what: Box<Expr>,
        op: AssignOp,
        value: Box<Expr>,
    },
    Variable {
        span: Span,
        name: String,
    },
    Field {
        span: Span,
        container: Box<Expr>,
        name: String,
    },
    Call {
        span: Span,
        callee: Box<Expr>,
        args: Vec<Expr>,
    },
    List {
        span: Span,
        list: Vec<Expr>,
    },
    Dict {
        span: Span,
        dict: Vec<(Expr, Expr)>,
    },
    Function {
        span: Span,
        params: Vec<String>,
        block: Block,
    },
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
    As(String),
    Pick(Vec<String>),
    All,
    Just,
}

/// Defines statement
#[derive(Debug, Clone)]
pub enum Stmt {
    While {
        span: Span,
        condition: Expr,
        block: Block,
    },
    Until {
        span: Span,
        condition: Expr,
        block: Block,
    },
    If {
        span: Span,
        condition: Expr,
        then: Block,
        else_: Option<Box<Stmt>>,
    },
    For {
        span: Span,
        var: String,
        iterable: Expr,
        block: Block,
    },
    Class(Class),
    Enum(Enum),
    Trait(Trait),
    Function(Function),
    Return {
        span: Span,
        value: Expr,
    },
    Continue(Span),
    Break(Span),
    Expr(Expr),
    Block(Box<Block>),
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
