/// Imports
use crate::{
    ops::{Chunk, Handler, Label, Opcode},
    refs::{MutRef, Ref},
    value::Value,
};
use common::{bug, span::Span};
use std::{cell::RefCell, collections::HashMap};

/// Defines scope of variables
#[derive(Default, Debug)]
pub struct Scope {
    /// Variables map
    pub variables: HashMap<String, Value>,

    /// Enclosing scope
    pub enclosing: Option<MutRef<Scope>>,
}

/// Scope implementation
impl Scope {
    /// Creates new scope with enclosing
    pub fn new(enclosing: MutRef<Scope>) -> Self {
        Self {
            enclosing: Some(enclosing),
            ..Default::default()
        }
    }

    /// Looks up a variable
    pub fn lookup(&self, name: &str) -> Option<Value> {
        match self.variables.get(name) {
            Some(it) => Some(it.clone()),
            None => match &self.enclosing {
                Some(env) => env.borrow().lookup(name),
                None => None,
            },
        }
    }

    /// Inserts a variable
    pub fn insert(&mut self, name: &str, value: Value) {
        self.variables.insert(name.to_string(), value);
    }

    /// Checks variable existence
    pub fn exists(&self, name: &str) -> bool {
        if self.variables.contains_key(name) {
            true
        } else {
            match &self.enclosing {
                Some(scope) => scope.borrow().exists(name),
                None => false,
            }
        }
    }
}

/// Defines call frame
#[derive(Debug)]
pub struct Frame {
    /// Chunk of code
    pub chunk: Ref<Chunk>,

    /// Program counter
    pc: usize,

    /// Scope chain
    pub scope: MutRef<Scope>,

    /// Operands stack
    stack: Vec<Value>,
}

/// Frame implementation
impl Frame {
    /// Creates new frame with new scope
    pub fn new(chunk: Ref<Chunk>) -> Self {
        Self {
            chunk,
            pc: 0,
            scope: MutRef::new(RefCell::new(Scope::default())),
            stack: Vec::new(),
        }
    }

    /// Creates new frame with new scope with enclosing
    pub fn with_enclosing(chunk: Ref<Chunk>, enclosing: MutRef<Scope>) -> Self {
        Self {
            chunk,
            pc: 0,
            scope: MutRef::new(RefCell::new(Scope::new(enclosing))),
            stack: Vec::new(),
        }
    }

    /// Creates new frame with passed scope
    pub fn with_scope(chunk: Ref<Chunk>, scope: MutRef<Scope>) -> Self {
        Self {
            chunk,
            pc: 0,
            scope,
            stack: Vec::new(),
        }
    }

    /// Pushes new operand to the stack
    pub fn push(&mut self, value: Value) {
        self.stack.push(value)
    }

    /// Pops an operand from the stack
    pub fn pop(&mut self) -> Value {
        self.stack
            .pop()
            .unwrap_or_else(|| bug!("pop with empty stack"))
    }

    /// Jumps to target pc
    pub fn jump(&mut self, pc: usize) {
        self.pc = pc
    }

    /// Increments pc
    pub fn next_instruction(&mut self) {
        self.pc += 1
    }

    /// Returns pc of label
    pub fn pc_of_label(&mut self, label: Label) -> usize {
        self.chunk.pc_of_label(label)
    }

    /// Returns opcode by current pc
    pub fn op(&self) -> Option<Opcode> {
        self.chunk.code.get(self.pc).cloned()
    }

    /// Returns span by current pc
    pub fn span(&self) -> Span {
        self.chunk
            .source_map
            .get(self.pc)
            .cloned()
            .unwrap_or_else(|| bug!("pc > source map len"))
    }

    /// Returns handler by current pc
    pub fn handler(&self) -> Option<Handler> {
        self.chunk
            .handlers
            .iter()
            .rev()
            .find(|h| (h.start..h.end).contains(&self.pc))
            .cloned()
    }
}
