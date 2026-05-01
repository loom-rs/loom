/// Imports
use crate::{
    callable, native_fun, realm,
    refs::{RealmRef, Ref},
    rt::{
        realm::Realm,
        value::{Native, Value},
    },
};
use std::rc::Rc;

/// Is int
fn int() -> Ref<Native> {
    native_fun! {
        arity = 1,
        fun = |_, _, values| {
            match values.first().unwrap() {
                Value::Int(_) => Value::Bool(true),
                _ => Value::Bool(false),
            }
        }
    }
}

/// Is float
fn float() -> Ref<Native> {
    native_fun! {
        arity = 1,
        fun = |_, _, values| {
            match values.first().unwrap() {
                Value::Float(_) => Value::Bool(true),
                _ => Value::Bool(false),
            }
        }
    }
}

/// Is bool
fn bool() -> Ref<Native> {
    native_fun! {
        arity = 1,
        fun = |_, _, values| {
            match values.first().unwrap() {
                Value::Bool(_) => Value::Bool(true),
                _ => Value::Bool(false),
            }
        }
    }
}

/// Is string
fn string() -> Ref<Native> {
    native_fun! {
        arity = 1,
        fun = |_, _, values| {
            match values.first().unwrap() {
                Value::String(_) => Value::Bool(true),
                _ => Value::Bool(false),
            }
        }
    }
}

/// Is callable
fn callable() -> Ref<Native> {
    native_fun! {
        arity = 1,
        fun = |_, _, values| {
            match values.first().unwrap() {
                Value::Callable(_) => Value::Bool(true),
                _ => Value::Bool(false),
            }
        }
    }
}

/// Is meta class
fn meta() -> Ref<Native> {
    native_fun! {
        arity = 1,
        fun = |_, _, values| {
            match values.first().unwrap() {
                Value::Class(_) => Value::Bool(true),
                _ => Value::Bool(false),
            }
        }
    }
}

/// Is module
fn module() -> Ref<Native> {
    native_fun! {
        arity = 1,
        fun = |_, _, values| {
            match values.first().unwrap() {
                Value::Module(_) => Value::Bool(true),
                _ => Value::Bool(false),
            }
        }
    }
}

/// Is instance
fn instance() -> Ref<Native> {
    native_fun! {
        arity = 1,
        fun = |_, _, values| {
            match values.first().unwrap() {
                Value::Instance(_) => Value::Bool(true),
                _ => Value::Bool(false),
            }
        }
    }
}

/// Is type of
fn type_of() -> Ref<Native> {
    native_fun! {
        arity = 2,
        fun = |_, _, values| {
            match values.first().unwrap() {
                Value::Instance(instance) => match values.get(1).unwrap() {
                    Value::Class(ty) => Value::Bool(Rc::ptr_eq(&instance.borrow().type_of, ty)),
                    _ => Value::Bool(false),
                },
                _ => Value::Bool(false),
            }
        }
    }
}

/// Provides `is` module env
pub fn provide_env() -> RealmRef {
    realm! {
        int => callable!(int()),
        float => callable!(float()),
        bool => callable!(bool()),
        string => callable!(string()),
        callable => callable!(callable()),
        meta => callable!(meta()),
        module => callable!(module()),
        instance => callable!(instance()),
        type_of => callable!(type_of())
    }
}
