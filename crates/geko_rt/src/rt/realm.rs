/// Imports
use crate::{errors::RuntimeError, refs::RealmRef, rt::value::Value};
use geko_common::bail;
use geko_lex::token::Span;
use std::collections::HashMap;

/// Variables realm
#[derive(Default, Debug)]
pub struct Realm {
    /// Variables map
    pub variables: HashMap<String, Value>,
    /// Enclosing
    enclosing: Option<RealmRef>,
}

/// Implementation
impl Realm {
    /// Creates new realm with enclosing
    pub fn new(enclosing: RealmRef) -> Self {
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

    /// Sets a variable value
    pub fn set(&mut self, span: &Span, name: &str, value: Value) {
        if self.variables.contains_key(name) {
            self.variables.insert(name.to_string(), value);
        } else {
            match &self.enclosing {
                Some(env) => env.borrow_mut().set(span, name, value),
                None => bail!(RuntimeError::UndefinedVariable {
                    name: name.to_string(),
                    src: span.0.clone(),
                    span: span.1.clone().into()
                }),
            }
        }
    }

    /// Defines a variable
    pub fn define(&mut self, name: &str, value: Value) {
        self.variables.insert(name.to_string(), value);
    }
}
