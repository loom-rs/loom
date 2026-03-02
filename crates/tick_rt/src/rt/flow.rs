/// Imports
use crate::rt::value::Value;

/// Control flow
pub enum ControlFlow {
    Continue,
    Break,
    Return(Value),
}

/// Flow type
pub type Flow<T> = Result<T, ControlFlow>;
