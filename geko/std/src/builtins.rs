/// Impovms
use crate::{arg, builtin_class, error, expect, scope};
use crate::{callable, class, native_fun};
use common::bail;
use dune::errors::RuntimeError;
use dune::frame::Scope;
use dune::refs::{MutRef, Ref};
use dune::value::{Native, Value};
use std::{collections::HashMap, rc::Rc};

/// Put definition
pub fn put() -> Ref<Native> {
    native_fun! {
        arity = 1,
        fun = |vm, _, values| {
            vm.io.output(&values.first().unwrap().to_string());
            vm.io.flush();
            Value::Null
        }
    }
}

/// Putln definition
pub fn putln() -> Ref<Native> {
    native_fun! {
        arity = 1,
        fun = |vm, _, values| {
            vm.io.output(&format!("{}\n", values.first().unwrap()));
            vm.io.flush();
            Value::Null
        }
    }
}

/// Readln definition
pub fn readln() -> Ref<Native> {
    native_fun! {
        arity = 0,
        fun = |vm, _, _| {
            Value::String(vm.io.input())
        }
    }
}

/// String of definition
pub fn str_of() -> Ref<Native> {
    native_fun! {
        arity = 1,
        fun = |_, _, values| {
            Value::String(values.first().cloned().unwrap().to_string())
        }
    }
}

/*
/// Ok definition
pub fn ok() -> Ref<Native> {
    native_fun! {
        arity = 1,
        fun = |vm, span, values| {
            let value = values.first().cloned().unwrap();
            Value::Instance(make_result(vm, span, value, true))
        }
    }
}

/// Error definition
pub fn error() -> Ref<Native> {
    native_fun! {
        arity = 1,
        fun = |vm, span, values| {
            let value = values.first().cloned().unwrap();
            Value::Instance(make_result(vm, span, value, false))
        }
    }
}
*/

/// Bail definition
pub fn bail() -> Ref<Native> {
    dune::refs::Ref::new(dune::value::Native {
        arity: 1,
        function: Box::new(|_, span, values| {
            let text = expect!(span, arg!(values, 0), Value::String);
            error!(span, format!("{text:?}"))
        }),
    })
}

/// Todo definition
pub fn todo() -> Ref<Native> {
    native_fun! {
        arity = 0,
        fun = |_, span, _| {
            error!(span, "found todo")
        }
    }
}

/// Length of string or list
pub fn len_of() -> Ref<Native> {
    native_fun! {
        arity = 1,
        fun = |vm, span, values| {
            // Matching value to find out way how to get length
            match values.first().cloned().unwrap() {
                // If string, retrieving it's len
                Value::String(str) => Value::Int(str.len() as i64),
                // If instance, checking of which class this instance is
                Value::Instance(instance) => {
                    // Retrieving list class
                    let list_class = builtin_class!(vm, "List");

                    // Retrieving dict class
                    let dict_class = builtin_class!(vm, "Dict");

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
                                    _ => error!(span, "couldn't get len of corrupted list"),
                                }
                            }
                            _ => error!(span, "couldn't get len of corrupted list"),
                        }
                    }
                    // Checking instance is dict
                    else if Rc::ptr_eq(&instance.borrow_mut().type_of, &dict_class) {
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
                                match list.borrow_mut().downcast_mut::<HashMap<Value, Value>>() {
                                    Some(map) => Value::Int(map.len() as i64),
                                    _ => error!(span, "couldn't get len of corrupted dict"),
                                }
                            }
                            _ => error!(span, "couldn't get len of corrupted dict"),
                        }
                    } else {
                        error!(
                            span,
                            &format!("couldn't get len of `{:?}`", Value::Instance(instance)),
                        )
                    }
                }
                // Anything else => error
                other => error!(span, &format!("couldn't get len of `{:?}`", other)),
            }
        }
    }
}

/// Provides builtins scope
pub fn provide() -> MutRef<Scope> {
    scope! {
        put => callable!(put()),
        putln => callable!(putln()),
        readln => callable!(readln()),
        str_of => callable!(str_of()),
        len_of => callable!(len_of()),
        // ok => callable!(ok()),
        // error => callable!(error()),
        bail => callable!(bail()),
        todo => callable!(todo()),
        // List => class!(list::provide_class()),
        // Dict => class!(dict::provide_class()),
        // Result => class!(result::provide_class())
    }
}
