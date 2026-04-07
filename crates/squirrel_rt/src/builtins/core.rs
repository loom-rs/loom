/// Imports
use crate::{
    builtins::{dict, list, utils},
    refs::{EnvRef, Ref},
    rt::{
        env::Environment,
        value::{Callable, Native, Value},
    },
};
use squirrel_common::bug;
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

/// Length of string or list
pub fn len_of() -> Ref<Native> {
    Ref::new(Native {
        arity: 1,
        function: Box::new(|rt, span, values| {
            // Matching value to find out way how to get length
            match values.first().cloned().unwrap() {
                // If string, retrieving it's len
                Value::String(str) => Value::Int(str.len() as i64),
                // If instance, checking of which class this instance is
                Value::Instance(instance) => {
                    // Retrieving list class
                    let list_class = {
                        let list_value = rt
                            .builtins
                            .env
                            .borrow()
                            .lookup("List")
                            .unwrap_or_else(|| bug!("no builtin `List` found"));

                        match list_value {
                            Value::Class(t) => t,
                            _ => bug!("builtin `List` is not a class"),
                        }
                    };

                    // Checking instance is list
                    if Rc::ptr_eq(&instance.borrow_mut().type_of, &list_class) {
                        // If instance is list, retrieving len of it's internal vector
                        // Safety: borrow is temporal for this line
                        let internal = instance
                            .borrow_mut()
                            .fields
                            .get("$internal")
                            .cloned()
                            .unwrap();

                        match internal {
                            Value::Any(list) => {
                                match list.borrow_mut().downcast_mut::<Vec<Value>>() {
                                    Some(vec) => Value::Int(vec.len() as i64),
                                    _ => utils::error(span, "couldn't get len of corrupted list"),
                                }
                            }
                            _ => utils::error(span, "couldn't get len of corrupted list"),
                        }
                    } else {
                        utils::error(
                            span,
                            &format!("couldn't get len of `{:?}`", Value::Instance(instance)),
                        )
                    }
                }
                // Anything else => error
                other => utils::error(span, &format!("couldn't get len of `{:?}`", other)),
            }
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
    env.force_define("len_of", Value::Callable(Callable::Native(len_of())));
    env.force_define("List", Value::Class(list::provide_class()));
    env.force_define("Dict", Value::Class(dict::provide_class()));

    Rc::new(RefCell::new(env))
}
