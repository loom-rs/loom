/// Imports
use crate::{errors::RuntimeError, interpreter::Interpreter, rt::value::Value};
use geko_common::{bail, bug};
use geko_lex::token::Span;

/// Bails runtime error with provided span and text
pub fn error(span: &Span, text: &str) -> ! {
    bail!(RuntimeError::Bail {
        text: text.to_string(),
        src: span.0.clone(),
        span: span.1.clone().into()
    })
}

/// Returns builtin found by name
pub fn get_builtin(rt: &mut Interpreter, name: &str) -> Value {
    rt.builtins
        .env
        .borrow()
        .lookup(name)
        .unwrap_or_else(|| bug!(format!("no builtin `{name}` found")))
}
