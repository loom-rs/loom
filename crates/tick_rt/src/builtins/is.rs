/// Imports
use crate::{
    refs::{EnvRef, Ref},
    rt::env::Environment,
    rt::value::{Callable, Native, Value},
};
use std::{cell::RefCell, rc::Rc};

/// Is int
fn int() -> Ref<Native> {
    return Ref::new(Native {
        arity: 1,
        function: Box::new(|_, _, values| match values.get(0).unwrap() {
            Value::Int(_) => Value::Bool(true),
            _ => Value::Bool(false),
        }),
    });
}

/// Is float
fn float() -> Ref<Native> {
    return Ref::new(Native {
        arity: 1,
        function: Box::new(|_, _, values| match values.get(0).unwrap() {
            Value::Float(_) => Value::Bool(true),
            _ => Value::Bool(false),
        }),
    });
}

/// Is bool
fn bool() -> Ref<Native> {
    return Ref::new(Native {
        arity: 1,
        function: Box::new(|_, _, values| match values.get(0).unwrap() {
            Value::Bool(_) => Value::Bool(true),
            _ => Value::Bool(false),
        }),
    });
}

/// Is string
fn string() -> Ref<Native> {
    return Ref::new(Native {
        arity: 1,
        function: Box::new(|_, _, values| match values.get(0).unwrap() {
            Value::String(_) => Value::Bool(true),
            _ => Value::Bool(false),
        }),
    });
}

/// Is callable
fn callable() -> Ref<Native> {
    return Ref::new(Native {
        arity: 1,
        function: Box::new(|_, _, values| match values.get(0).unwrap() {
            Value::Callable(_) => Value::Bool(true),
            _ => Value::Bool(false),
        }),
    });
}

/// Is meta type
fn meta() -> Ref<Native> {
    return Ref::new(Native {
        arity: 1,
        function: Box::new(|_, _, values| match values.get(0).unwrap() {
            Value::Type(_) => Value::Bool(true),
            _ => Value::Bool(false),
        }),
    });
}

/// Is module
fn module() -> Ref<Native> {
    return Ref::new(Native {
        arity: 1,
        function: Box::new(|_, _, values| match values.get(0).unwrap() {
            Value::Module(_) => Value::Bool(true),
            _ => Value::Bool(false),
        }),
    });
}

/// Is instance
fn instance() -> Ref<Native> {
    return Ref::new(Native {
        arity: 1,
        function: Box::new(|_, _, values| match values.get(0).unwrap() {
            Value::Instance(_) => Value::Bool(true),
            _ => Value::Bool(false),
        }),
    });
}

/// Is type of
fn type_of() -> Ref<Native> {
    return Ref::new(Native {
        arity: 2,
        function: Box::new(|_, _, values| match values.get(0).unwrap() {
            Value::Instance(instance) => match values.get(1).unwrap() {
                Value::Type(ty) => Value::Bool(Rc::ptr_eq(&instance.borrow().type_of, ty)),
                _ => Value::Bool(false),
            },
            _ => Value::Bool(false),
        }),
    });
}

/// Provides `is` module env
pub fn provide_env() -> EnvRef {
    let mut env = Environment::default();
    env.force_define("int", Value::Callable(Callable::Native(int())));
    env.force_define("float", Value::Callable(Callable::Native(float())));
    env.force_define("bool", Value::Callable(Callable::Native(bool())));
    env.force_define("string", Value::Callable(Callable::Native(string())));
    env.force_define("callable", Value::Callable(Callable::Native(callable())));
    env.force_define("meta", Value::Callable(Callable::Native(meta())));
    env.force_define("module", Value::Callable(Callable::Native(module())));
    env.force_define("instance", Value::Callable(Callable::Native(instance())));
    env.force_define("type_of", Value::Callable(Callable::Native(type_of())));
    Rc::new(RefCell::new(env))
}
