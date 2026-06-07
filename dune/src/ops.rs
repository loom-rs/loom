/// Imports
use crate::value::Value;
use common::span::Span;

/// Defines virtual machine operation code
#[derive(Debug, Clone)]
pub enum Opcode {
    Nop,
    Push(Value),
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
    Neg,
    Bang,
    Jump(usize),
    JumpIf(usize),
    Return,
    Halt,
    Load(String),
    Store(String),
    Define(String),
    Call(usize),
    Field(String),
    Import(String),
}

/// Defines chunk of opcodes
#[derive(Debug, Clone)]
pub struct Chunk {
    /// Chunk bytecode
    pub code: Vec<Opcode>,

    /// Source map:
    /// Pc -> Span
    pub source_map: Vec<Span>,
}

/// Chunk implementation
impl Chunk {
    /// Returns len of the chunk bytecode
    pub fn len(&self) -> usize {
        self.code.len()
    }
}
