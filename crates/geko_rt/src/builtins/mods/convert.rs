/// Imports
use crate::{
    callable, error, native_fun, realm,
    refs::{RealmRef, Ref},
    rt::{
        realm::Realm,
        value::{Native, Value},
    },
};

/// Any -> int
fn int() -> Ref<Native> {
    native_fun! {
        arity = 1,
        fun = |_, span, values| {
            match values.first().unwrap() {
                Value::Int(i) => Value::Int(*i),
                Value::Float(f) => Value::Float(*f),
                other => error!(
                    span,
                    &format!("could not convert `{other}` into int value")
                ),
            }
        }
    }
}

/// Any -> float
fn float() -> Ref<Native> {
    native_fun! {
        arity = 1,
        fun = |_, span, values| {
            match values.first().unwrap() {
                Value::Int(i) => Value::Float(*i as f64),
                Value::Float(f) => Value::Float(*f),
                other => error!(
                    span,
                    &format!("could not convert `{other}` into float value")
                ),
            }
        }
    }
}

/// Any -> bool
fn bool() -> Ref<Native> {
    native_fun! {
        arity = 1,
        fun = |_, span, values| {
            match values.first().unwrap() {
                Value::Bool(b) => Value::Bool(*b),
                Value::String(s) if s == "true" => Value::Bool(true),
                Value::String(s) if s == "false" => Value::Bool(false),
                other => error!(
                    span,
                    &format!("could not convert `{other}` into float value")
                ),
            }
        }
    }
}

/// Any -> string
fn string() -> Ref<Native> {
    native_fun! {
        arity = 1,
        fun = |_, _, values| {
            Value::String(format!("{}", values.first().unwrap()))
        }
    }
}

/// Provides `convert` module realm
pub fn provide_env() -> RealmRef {
    realm! {
        int => callable!(int()),
        float => callable!(float()),
        bool => callable!(bool()),
        string => callable!(string())
    }
}
