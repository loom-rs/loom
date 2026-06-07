use common::{bail, bug, span::Span};

/// Imports
use crate::{
    errors::RuntimeError,
    ops::Chunk,
    refs::{MutRef, Ref},
    value::Value,
};
use std::{cell::RefCell, collections::HashMap};

/// Defines scope of variables
#[derive(Default, Debug)]
pub struct Scope {
    /// Variables map
    variables: HashMap<String, Value>,

    /// Enclosing scope
    enclosing: Option<MutRef<Scope>>,
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
pub struct Frame {
    /// Chunk of code
    chunk: Ref<Chunk>,

    /// Program counter
    pc: usize,

    /// Scope chain
    scope: MutRef<Scope>,

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
}
