/// Imports
use crate::{
    builtins::{
        dict, list,
        result::{self, make_result},
        utils,
    },
    errors::RuntimeError,
    interpreter::Interpreter,
    refs::{RealmRef, Ref},
    rt::{
        realm::Realm,
        value::{Callable, Class, Native, Value},
    },
};
use geko_common::{bail, bug};
use std::{cell::RefCell, collections::HashMap, rc::Rc};

/// Put definition
pub fn put() -> Ref<Native> {
    Ref::new(Native {
        arity: 1,
        function: Box::new(|rt, _, values| {
            rt.io.output(&values.first().unwrap().to_string());
            rt.io.flush();
            Value::Null
        }),
    })
}

/// Putln definition
pub fn putln() -> Ref<Native> {
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

/// Ok definition
pub fn ok() -> Ref<Native> {
    Ref::new(Native {
        arity: 1,
        function: Box::new(|rt, span, values| {
            let value = values.get(0).cloned().unwrap();
            Value::Instance(make_result(rt, span, value, true))
        }),
    })
}

/// Error definition
pub fn error() -> Ref<Native> {
    Ref::new(Native {
        arity: 1,
        function: Box::new(|rt, span, values| {
            let value = values.get(0).cloned().unwrap();
            Value::Instance(make_result(rt, span, value, false))
        }),
    })
}

/// Bail definition
pub fn bail() -> Ref<Native> {
    Ref::new(Native {
        arity: 1,
        function: Box::new(|_, span, values| {
            let text = values.get(0).cloned().unwrap();
            bail!(RuntimeError::Bail {
                text: format!("{text}"),
                src: span.0.clone(),
                span: span.1.clone().into()
            })
        }),
    })
}

/// Todo definition
pub fn todo() -> Ref<Native> {
    Ref::new(Native {
        arity: 0,
        function: Box::new(|_, span, _| {
            bail!(RuntimeError::Bail {
                text: format!("found todo"),
                src: span.0.clone(),
                span: span.1.clone().into()
            })
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
                    let list_class = match utils::get_builtin(rt, "List") {
                        Value::Class(t) => t,
                        _ => bug!("builtin `List` is not a class"),
                    };

                    // Retrieving dict class
                    let dict_class = match utils::get_builtin(rt, "Dict") {
                        Value::Class(t) => t,
                        _ => bug!("builtin `List` is not a class"),
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
                                    _ => utils::error(span, "couldn't get len of corrupted dict"),
                                }
                            }
                            _ => utils::error(span, "couldn't get len of corrupted dict"),
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

/// Provides `core` realm
pub fn provide_env() -> RealmRef {
    let mut realm = Realm::default();

    realm.define("put", Value::Callable(Callable::Native(put())));
    realm.define("putln", Value::Callable(Callable::Native(putln())));
    realm.define("readln", Value::Callable(Callable::Native(readln())));
    realm.define("str_of", Value::Callable(Callable::Native(str_of())));
    realm.define("len_of", Value::Callable(Callable::Native(len_of())));
    realm.define("ok", Value::Callable(Callable::Native(ok())));
    realm.define("error", Value::Callable(Callable::Native(error())));
    realm.define("bail", Value::Callable(Callable::Native(bail())));
    realm.define("todo", Value::Callable(Callable::Native(todo())));
    realm.define("List", Value::Class(list::provide_class()));
    realm.define("Dict", Value::Class(dict::provide_class()));
    realm.define("Result", Value::Class(result::provide_class()));

    Rc::new(RefCell::new(realm))
}
