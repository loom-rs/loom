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
    NotImpls,
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
    LoadField(String),
    StoreField(String),
    DefineField(String),
    Import(String),
}

/// Defines chunk of opcodes
#[derive(Debug, Clone, Default)]
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

    /// Inserts new opcode and returns its index
    pub fn insert(&mut self, span: Span, op: Opcode) -> usize {
        self.source_map.push(span);
        self.code.push(op);
        self.code.len() - 1
    }

    /// Patches opcode at specified index
    pub fn patch(&mut self, idx: usize, op: Opcode) {
        self.code[idx] = op
    }
}
