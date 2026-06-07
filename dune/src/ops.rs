/// Imports
use common::span::Span;

/// Represents opcode value
#[derive(Debug, Clone)]
pub enum OpcodeValue {
    Int(i64),
    Float(f64),
    Bool(bool),
    String(String),
}

/// Defines virtual machine operation code
#[derive(Debug, Clone)]
pub enum Opcode {
    Nop,
    Push(OpcodeValue),
    Pop,
    Dup,
    Add,
    Sub,
    Mul,
    Div,
    Rem,
    Gt,
    Ge,
    Lt,
    Le,
    Eq,
    Ne,
    And,
    Or,
    Xor,
    Band,
    Bor,
    Impls,
    NotImpls,
    Neg,
    Bang,
    Jump(usize),
    JumpIf(usize),
    Return,
    Call,
    MakeFunction(Chunk),
    MakeClass(usize),
    MakeEnum(Vec<String>),
    MakeTrait(Vec<(String, usize)>),
    Import(String),
    ImportPick(String, Vec<String>),
}

/// Defines chunk of opcodes
#[derive(Debug, Clone)]
pub struct Chunk {
    /// Chunk bytecode
    code: Vec<Opcode>,

    /// Source map:
    /// Pc -> Span
    source: Vec<Span>,
}
