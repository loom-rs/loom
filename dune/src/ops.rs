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
    Jump(Label),
    JumpIfTrue(Label),
    JumpIfFalse(Label),
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

/// Defines chunk label
#[derive(Debug, Clone, Default, Copy)]
pub struct Label(usize);

/// Defines chunk of opcodes
#[derive(Debug, Clone, Default)]
pub struct Chunk {
    /// Chunk bytecode
    pub code: Vec<Opcode>,

    /// Chunk labels map:
    /// Id -> Pc
    pub labels: Vec<usize>,

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

    /// Inserts new label
    pub fn insert_label(&mut self, pc: usize) -> Label {
        self.labels.push(pc);
        Label(self.labels.len() - 1)
    }

    /// Creates fresh label
    pub fn fresh_label(&mut self) -> Label {
        self.labels.push(self.code.len() - 1);
        Label(self.labels.len() - 1)
    }

    /// Patches specified label with new pc
    pub fn patch_label(&mut self, label: Label, pc: usize) {
        self.labels[label.0] = pc
    }

    /// Returns pc by label
    pub fn pc_of_label(&self, label: Label) -> usize {
        self.labels[label.0]
    }
}
