/// Imports
use crate::{
    builtins::list,
    refs::{EnvRef, Ref},
    rt::{
        env::Environment,
        value::{Callable, Native, Value},
    },
};
use std::{cell::RefCell, rc::Rc};

/// Print definition
pub fn print() -> Ref<Native> {
    Ref::new(Native {
        arity: 1,
        function: Box::new(|rt, _, values| {
            rt.io.output(&values.first().unwrap().to_string());
            rt.io.flush();
            Value::Null
        }),
    })
}

/// Println definition
pub fn println() -> Ref<Native> {
    Ref::new(Native {
        arity: 1,
        function: Box::new(|rt, _, values| {
            rt.io.output(&format!("{}\n", values.first().unwrap()));
            rt.io.flush();
            Value::Null
        }),
    })
}

/// Readln definition
pub fn readln() -> Ref<Native> {
    Ref::new(Native {
        arity: 0,
        function: Box::new(|rt, _, _| Value::String(rt.io.input())),
    })
}

/// String of definition
pub fn str_of() -> Ref<Native> {
    Ref::new(Native {
        arity: 1,
        function: Box::new(|_, _, values| {
            Value::String(values.first().cloned().unwrap().to_string())
        }),
    })
}

/// Provides env
pub fn provide_env() -> EnvRef {
    let mut env = Environment::default();

    env.force_define("print", Value::Callable(Callable::Native(print())));
    env.force_define("println", Value::Callable(Callable::Native(println())));
    env.force_define("readln", Value::Callable(Callable::Native(readln())));
    env.force_define("str_of", Value::Callable(Callable::Native(str_of())));
    env.force_define("List", Value::Type(list::provide_type()));

    Rc::new(RefCell::new(env))
}
