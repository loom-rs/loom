/// Imports
use crate::{
    error,
    interpreter::Interpreter,
    refs::{MutRef, Ref},
    rt::value::{Class, Instance, Method, Native, Value},
};
use geko_common::bug;
use geko_lex::token::Span;
use std::collections::HashMap;

/// Helper: validates result
fn validate_result<F, V>(span: &Span, result: Value, f: F) -> V
where
    F: FnOnce(Value, bool) -> V,
{
    match result {
        Value::Instance(instance) => {
            // Safety: borrow is temporal for this lines
            let guard = instance.borrow_mut();
            let is_ok = guard.fields.get("$is_ok").cloned().unwrap();
            let value = guard.fields.get("$value").cloned().unwrap();

            match is_ok {
                Value::Bool(ok) => f(value, ok),
                _ => error!(span, "corrupted result"),
            }
        }
        _ => unreachable!(),
    }
}

/// Helper: validates result argument
fn validate_result_arg<F, V>(span: &Span, values: &[Value], f: F) -> V
where
    F: FnOnce(Value, bool) -> V,
{
    validate_result(span, values.first().cloned().unwrap(), f)
}

/// Helper: makes new result
#[allow(dead_code)]
pub fn make_result(
    rt: &mut Interpreter,
    span: &Span,
    value: Value,
    is_ok: bool,
) -> MutRef<Instance> {
    let result_value = rt
        .builtins
        .env
        .borrow()
        .lookup("Result")
        .unwrap_or_else(|| bug!("no builtin `Result` found"));

    match result_value {
        Value::Class(t) => match rt.call_class(span, vec![value, Value::Bool(is_ok)], t) {
            Ok(Value::Instance(instance)) => instance,
            Ok(_) => unreachable!(),
            Err(err) => {
                bug!(format!(
                    "calling of builtin `Result` has ended with a control flow leak: {err:?}"
                ))
            }
        },
        _ => bug!("builtin `Result` is not a class"),
    }
}

/// Init method
fn init_method() -> Method {
    Method::Native(Ref::new(Native {
        arity: 3,
        function: Box::new(|_, _, values| {
            let list = values.first().cloned().unwrap();
            match list {
                Value::Instance(instance) => {
                    // Safety: borrow is temporal for this lines
                    let mut guard = instance.borrow_mut();

                    // Preparing fields
                    guard
                        .fields
                        .insert("$value".to_owned(), values.get(1).cloned().unwrap());
                    guard
                        .fields
                        .insert("$is_ok".to_owned(), values.get(2).cloned().unwrap());

                    Value::Null
                }
                _ => unreachable!(),
            }
        }),
    }))
}

/// To string method
fn to_string_method() -> Method {
    Method::Native(Ref::new(Native {
        arity: 1,
        function: Box::new(|_, span, values| {
            validate_result_arg(span, &values, |value, is_ok| {
                if is_ok {
                    Value::String(format!("ok({})", value))
                } else {
                    Value::String(format!("error({})", value))
                }
            })
        }),
    }))
}

/// Is ok method
fn is_ok_method() -> Method {
    Method::Native(Ref::new(Native {
        arity: 1,
        function: Box::new(|_, span, values| {
            validate_result_arg(span, &values, |_, is_ok| Value::Bool(is_ok))
        }),
    }))
}

/// Is error method
fn is_error_method() -> Method {
    Method::Native(Ref::new(Native {
        arity: 1,
        function: Box::new(|_, span, values| {
            validate_result_arg(span, &values, |_, is_ok| Value::Bool(!is_ok))
        }),
    }))
}

/// Unwrap method
fn unwrap() -> Method {
    Method::Native(Ref::new(Native {
        arity: 1,
        function: Box::new(|_, span, values| {
            validate_result_arg(span, &values, |value, is_ok| {
                if is_ok {
                    value
                } else {
                    error!(span, "unwrap on error result")
                }
            })
        }),
    }))
}

/// Unwrap error method
fn unwrap_error() -> Method {
    Method::Native(Ref::new(Native {
        arity: 1,
        function: Box::new(|_, span, values| {
            validate_result_arg(span, &values, |value, is_ok| {
                if !is_ok {
                    value
                } else {
                    error!(span, "unwrap error on ok result")
                }
            })
        }),
    }))
}

/// If ok method
fn if_ok() -> Method {
    Method::Native(Ref::new(Native {
        arity: 2,
        function: Box::new(|rt, span, values| {
            validate_result_arg(span, &values, |value, is_ok| {
                if is_ok {
                    match values.get(1).cloned().unwrap() {
                        Value::Callable(callable) => match rt.call(span, vec![value], callable) {
                            Ok(value) => value,
                            Err(_) => bug!("control flow leak"),
                        },
                        _ => error!(span, "invalid function for `if_ok`"),
                    }
                } else {
                    Value::Null
                }
            })
        }),
    }))
}

/// If error method
fn if_error() -> Method {
    Method::Native(Ref::new(Native {
        arity: 2,
        function: Box::new(|rt, span, values| {
            validate_result_arg(span, &values, |value, is_ok| {
                if !is_ok {
                    match values.get(1).cloned().unwrap() {
                        Value::Callable(callable) => match rt.call(span, vec![value], callable) {
                            Ok(value) => value,
                            Err(_) => bug!("control flow leak"),
                        },
                        _ => error!(span, "invalid function for `if_error`"),
                    }
                } else {
                    Value::Null
                }
            })
        }),
    }))
}

/// Provides list class
pub fn provide_class() -> Ref<Class> {
    Ref::new(Class {
        name: "Result".to_string(),
        methods: HashMap::from([
            // Init method
            ("init".to_string(), init_method()),
            // To string method
            ("to_string".to_string(), to_string_method()),
            // Is ok method
            ("is_ok".to_string(), is_ok_method()),
            // Is error method
            ("is_error".to_string(), is_error_method()),
            // Unwrap method
            ("unwrap".to_string(), unwrap()),
            // Unwrap error method
            ("unwrap_error".to_string(), unwrap_error()),
            // If ok method
            ("if_ok".to_string(), if_ok()),
            // If error method
            ("if_error".to_string(), if_error()),
        ]),
    })
}
