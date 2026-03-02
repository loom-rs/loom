/// Imports
use crate::refs::{EnvRef, MutRef, Ref};
use tick_ast::stmt::Block;
use tick_common::io::IO;
use tick_lex::token::Span;
use std::{
    any::Any,
    collections::HashMap,
    fmt::{Debug, Display},
    rc::Rc,
};

/// Native function value
#[derive(Clone, Debug)]
pub struct Native {
    /// Function parameters arity
    pub arity: usize,
    /// Native function
    pub function: Box<fn(&dyn IO, &Span, Vec<Value>) -> Value>,
}

/// Function value
#[derive(Clone, Debug)]
pub struct Function {
    /// Function parameters
    pub params: Vec<String>,
    /// Function block
    pub block: Block,
}

/// Closure function
#[derive(Clone, Debug)]
pub struct Closure {
    /// Function
    pub function: Ref<Function>,
    /// Environment
    pub environment: EnvRef,
}

/// Bound method
#[derive(Clone, Debug)]
pub struct Bound {
    /// Bound method
    pub method: Method,
    /// Instance bound method belongs to
    pub belongs_to: MutRef<Instance>,
}

/// User data type method
#[derive(Clone, Debug)]
pub enum Method {
    // Native method
    Native(Ref<Native>),
    // Closure method
    Closure(Ref<Closure>),
}

/// User data type
#[derive(Clone, Debug)]
pub struct Type {
    /// Data type name
    pub name: String,
    /// Data type methods
    pub methods: HashMap<String, Method>,
}

/// User data type instance
#[derive(Clone, Debug)]
pub struct Instance {
    /// Type of
    pub type_of: Ref<Type>,
    /// Instance fields
    pub fields: HashMap<String, Value>,
}

/// Module
#[derive(Clone, Debug)]
pub struct Module {
    /// Module environment
    pub env: EnvRef,
}

/// Runtime callable
#[derive(Clone, Debug)]
pub enum Callable {
    /// Closure
    Closure(Ref<Closure>),
    /// Bound to instance method
    Bound(Ref<Bound>),
    /// Native method
    Native(Ref<Native>),
}

/// PartialEq implementation
impl PartialEq for Callable {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Callable::Closure(a), Callable::Closure(b)) => Rc::ptr_eq(a, b),
            (Callable::Bound(a), Callable::Bound(b)) => Rc::ptr_eq(a, b),
            (Callable::Native(a), Callable::Native(b)) => Rc::ptr_eq(a, b),
            _ => false,
        }
    }
}

/// Runtime value representation
#[derive(Clone)]
pub enum Value {
    /// Boolean value
    Bool(bool),
    /// Integer number value
    Int(i64),
    /// Float number value
    Float(f64),
    /// String value
    String(String),
    /// Function value
    Callable(Callable),
    /// Meta type
    Type(Ref<Type>),
    /// Module
    Module(MutRef<Module>),
    /// Type instance
    Instance(MutRef<Instance>),
    /// Rust's any type
    Any(MutRef<dyn Any>),
    /// Null reference
    Null,
}

/// Display implementation
impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Matchin value
        match self {
            Value::Bool(val) => write!(f, "{val}"),
            Value::Int(int) => write!(f, "{int}"),
            Value::Float(float) => write!(f, "{float}"),
            Value::String(string) => write!(f, "{string}"),
            Value::Callable(_) => write!(f, "Callable"),
            Value::Type(typ) => write!(f, "Type({})", typ.name),
            Value::Module(_) => write!(f, "Module"),
            Value::Instance(instance) => write!(f, "Instance({})", instance.borrow().type_of.name),
            Value::Any(_) => write!(f, "Any"),
            Value::Null => write!(f, "null"),
        }
    }
}

/// Debug implementation
impl Debug for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self}")
    }
}

/// PartialEq implementation
impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Bool(a), Self::Bool(b)) => a == b,
            (Self::Int(a), Self::Int(b)) => a == b,
            (Self::Float(a), Self::Float(b)) => a == b,
            (Self::String(a), Self::String(b)) => a == b,
            (Self::Callable(a), Self::Callable(b)) => a == b,
            (Self::Type(a), Self::Type(b)) => Rc::ptr_eq(a, b),
            (Self::Module(a), Self::Module(b)) => Rc::ptr_eq(a, b),
            (Self::Instance(a), Self::Instance(b)) => Rc::ptr_eq(a, b),
            _ => false,
        }
    }
}
