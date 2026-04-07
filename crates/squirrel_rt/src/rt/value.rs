/// Imports
use crate::{
    interpreter::Interpreter,
    refs::{EnvRef, MutRef, Ref},
};
use squirrel_ast::stmt::Block;
use squirrel_lex::token::Span;
use std::{
    any::Any,
    collections::HashMap,
    fmt::{Debug, Display},
    hash::{Hash, Hasher},
    rc::Rc,
};

/// Native function value
#[derive(Clone, Debug)]
pub struct Native {
    /// Function parameters arity
    pub arity: usize,
    /// Native function
    #[allow(clippy::type_complexity)]
    pub function: Box<fn(&mut Interpreter, &Span, Vec<Value>) -> Value>,
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

/// User class type
#[derive(Clone, Debug)]
pub struct Class {
    /// Class type name
    pub name: String,
    /// Class type methods
    pub methods: HashMap<String, Method>,
}

/// User class type instance
#[derive(Clone, Debug)]
pub struct Instance {
    /// Type of
    pub type_of: Ref<Class>,
    /// Instance fields
    pub fields: HashMap<String, Value>,
}

/// User enum type
#[derive(Clone, Debug)]
pub struct Enum {
    /// Enum type name
    pub name: String,
    /// Enum type variants
    pub variants: Vec<String>,
}

/// User trait function
#[derive(Clone, Debug)]
pub struct TraitFunction {
    /// Trait function name
    pub name: String,
    /// Trait function arity
    pub arity: usize,
}

/// User trait type
#[derive(Clone, Debug)]
pub struct Trait {
    /// Trait name
    pub name: String,
    /// Trait functions
    pub functions: Vec<TraitFunction>,
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
    Class(Ref<Class>),
    /// Enum type
    Enum(Ref<Enum>),
    /// Trait
    Trait(Ref<Trait>),
    /// Module
    Module(MutRef<Module>),
    /// Class instance
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
            Value::Class(typ) => write!(f, "Class({})", typ.name),
            Value::Enum(typ) => write!(f, "Enum({})", typ.name),
            Value::Trait(trt) => write!(f, "Trait({})", trt.name),
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

/// Hash implementation
impl Hash for Value {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // Hashing descriminant
        std::mem::discriminant(self).hash(state);

        // Hashing self
        match self {
            // Primitives hash
            Value::Bool(v) => v.hash(state),
            Value::Int(v) => v.hash(state),
            Value::Float(v) => {
                if v.is_nan() {
                    0.hash(state);
                } else {
                    v.to_bits().hash(state);
                }
            }
            Value::String(v) => v.hash(state),

            // Reference-types hash
            Value::Callable(c) => match c {
                Callable::Closure(r) => {
                    (Rc::as_ptr(r) as usize).hash(state);
                }
                Callable::Bound(r) => {
                    (Rc::as_ptr(r) as usize).hash(state);
                }
                Callable::Native(r) => {
                    (Rc::as_ptr(r) as usize).hash(state);
                }
            },
            Value::Class(r) => {
                (Rc::as_ptr(r) as usize).hash(state);
            }
            Value::Enum(r) => {
                (Rc::as_ptr(r) as usize).hash(state);
            }
            Value::Trait(r) => {
                (Rc::as_ptr(r) as usize).hash(state);
            }
            Value::Module(r) => {
                (Rc::as_ptr(r) as usize).hash(state);
            }
            Value::Instance(r) => {
                (Rc::as_ptr(r) as usize).hash(state);
            }
            Value::Any(r) => {
                (Rc::as_ptr(r) as *const () as usize).hash(state);
            }
            Value::Null => {}
        }
    }
}

/// Eq implementation
impl Eq for Value {}

/// PartialEq implementation
impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Bool(a), Self::Bool(b)) => a == b,
            (Self::Int(a), Self::Int(b)) => a == b,
            (Self::Float(a), Self::Float(b)) => a == b,
            (Self::String(a), Self::String(b)) => a == b,
            (Self::Callable(a), Self::Callable(b)) => a == b,
            (Self::Class(a), Self::Class(b)) => Rc::ptr_eq(a, b),
            (Self::Enum(a), Self::Enum(b)) => Rc::ptr_eq(a, b),
            (Self::Module(a), Self::Module(b)) => Rc::ptr_eq(a, b),
            (Self::Instance(a), Self::Instance(b)) => Rc::ptr_eq(a, b),
            (Self::Any(a), Self::Any(b)) => Rc::ptr_eq(a, b),
            _ => false,
        }
    }
}
