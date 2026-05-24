/// Imports
use crate::{
    arg, arg_ref, builtin_class, error, expect,
    interpreter::Interpreter,
    native_class, native_method,
    refs::{MutRef, Ref},
    rt::value::{Class, Instance, Method, Native, Value},
    to_string,
};
use geko_common::bug;
use geko_lex::token::Span;
use rand::RngExt;
use std::{cell::RefCell, collections::HashMap};

/// Helper: validates list
fn validate_list<F, V>(span: &Span, list: Value, f: F) -> V
where
    F: FnOnce(&mut Vec<Value>) -> V,
{
    // Retrieving instance
    let instance = expect!(span, list, Value::Instance);

    // Safety: borrow is temporal for this line
    let internal = instance
        .borrow_mut()
        .fields
        .get("$internal")
        .cloned()
        .unwrap();

    match internal {
        Value::Any(list) => match list.borrow_mut().downcast_mut::<Vec<Value>>() {
            Some(vec) => f(vec),
            _ => error!(span, "corrupted list"),
        },
        _ => {
            error!(span, "corrupted list");
        }
    }
}

/// Helper: validates list argument
fn validate_list_arg<F, V>(span: &Span, values: &[Value], f: F) -> V
where
    F: FnOnce(&mut Vec<Value>) -> V,
{
    validate_list(span, arg!(values, 0), f)
}

/// Helper: validates index
fn validate_idx<F, V>(span: &Span, idx: Value, len: usize, f: F) -> V
where
    F: FnOnce(usize) -> V,
{
    match idx {
        Value::Int(idx) => {
            if idx < 0 {
                error!(span, "index should be positive int")
            } else {
                let idx = idx as usize;
                if idx >= len {
                    error!(span, "index out of bounds")
                } else {
                    f(idx)
                }
            }
        }
        _ => error!(span, "index should be an int"),
    }
}

/// Helper: validates index argument
fn validate_idx_arg<F, V>(span: &Span, values: &[Value], idx: usize, len: usize, f: F) -> V
where
    F: FnOnce(usize) -> V,
{
    validate_idx(span, arg!(values, idx), len, f)
}

/// Helper: makes new list
pub fn make_list(rt: &mut Interpreter, span: &Span) -> MutRef<Instance> {
    // Getting builtin class
    let class = builtin_class!(rt, "List");

    // Calling class
    match rt.call_class(span, class, vec![]) {
        Ok(Value::Instance(instance)) => instance,
        Ok(_) => unreachable!(),
        Err(err) => {
            bug!(format!(
                "calling of builtin `List` has ended with a control flow leak: {err:?}"
            ))
        }
    }
}

/// Init method
fn init_method() -> Method {
    native_method! {
        arity = 1,
        fun = |_, span, values| {
            // Retrieving instance
            let instance = expect!(span, arg!(values, 0), Value::Instance);

            // Preparing vector
            let vec = Value::Any(MutRef::new(RefCell::new(Vec::<Value>::new())));

            // Safety: borrow is temporal for this line
            instance
                .borrow_mut()
                .fields
                .insert("$internal".to_string(), vec);

            Value::Null
        }
    }
}

/// To string method
fn to_string_method() -> Method {
    native_method! {
        arity = 1,
        fun = |rt, span, values| {
            validate_list_arg(span, &values, |vec| Value::String(
                vec.iter()
                    .map(|v| {
                        to_string!(span, rt, v)
                    })
                    .collect::<Vec<_>>()
                    .join(",")
            ))
        }
    }
}

/// Push method
fn push_method() -> Method {
    native_method! {
        arity = 2,
        fun = |_, span, values| {
            validate_list_arg(span, &values, |vec| {
                vec.push(arg!(values, 1));
                Value::Null
            })
        }
    }
}

/// Get method
fn get_method() -> Method {
    native_method! {
        arity = 2,
        fun = |_, span, values| {
            validate_list_arg(span, &values, |vec| {
                validate_idx_arg(span, &values, 1, vec.len(), |idx| vec[idx].clone())
            })
        }
    }
}

/// Set method
fn set_method() -> Method {
    native_method! {
        arity = 3,
        fun = |_, span, values| {
            validate_list_arg(span, &values, |vec| {
                validate_idx_arg(span, &values, 1, vec.len(), |idx| {
                    vec[idx] = arg!(values, 2);
                    Value::Null
                })
            })
        }
    }
}

/// Insert method
fn insert_method() -> Method {
    native_method! {
        arity = 3,
        fun = |_, span, values| {
            validate_list_arg(span, &values, |vec| {
                validate_idx_arg(span, &values, 1, vec.len(), |idx| {
                    vec.insert(idx, arg!(values, 2));
                    Value::Null
                })
            })
        }
    }
}

/// Remove method
fn remove_method() -> Method {
    native_method! {
        arity = 2,
        fun = |_, span, values| {
            validate_list_arg(span, &values, |vec| {
                validate_idx_arg(span, &values, 1, vec.len(), |idx| {
                    vec.remove(idx);
                    Value::Null
                })
            })
        }
    }
}

/// Len method
fn len_method() -> Method {
    native_method! {
        arity = 1,
        fun = |_, span, values| {
            validate_list_arg(span, &values, |vec| Value::Int(vec.len() as i64))
        }
    }
}

/// Clear method
fn clear_method() -> Method {
    native_method! {
        arity = 1,
        fun = |_, span, values| {
            validate_list_arg(span, &values, |vec| {
                vec.clear();
                Value::Null
            })
        }
    }
}

/// Pop method
fn pop_method() -> Method {
    native_method! {
        arity = 1,
        fun = |_, span, values| {
            validate_list_arg(span, &values, |vec| vec.pop().unwrap_or(Value::Null))
        }
    }
}

/// Index of method
fn index_of_method() -> Method {
    native_method! {
        arity = 2,
        fun = |_, span, values| {
            validate_list_arg(span, &values, |vec| {
                let value = arg!(values, 1);
                vec.iter()
                    .position(|v| *v == value)
                    .map(|it| Value::Int(it as i64))
                    .unwrap_or(Value::Int(-1))
            })
        }
    }
}

/// Contains
fn contains_method() -> Method {
    native_method! {
        arity = 2,
        fun = |_, span, values| {
            validate_list_arg(span, &values, |vec| {
                Value::Bool(vec.contains(arg_ref!(values, 1)))
            })
        }
    }
}

/// Choice method
fn choice_method() -> Method {
    native_method! {
        arity = 1,
        fun = |_, span, values| {
            validate_list_arg(span, &values, |vec| {
                match vec.get(rand::rng().random_range(0..vec.len())) {
                    Some(val) => val.clone(),
                    _ => error!(
                        span,
                        "list must have 1 or more elements to perform random choice on it",
                    ),
                }
            })
        }
    }
}

/// Provides list class
pub fn provide_class() -> Ref<Class> {
    native_class! {
        name = List,
        methods = {
            init => init_method(),
            to_string => to_string_method(),
            push => push_method(),
            get => get_method(),
            set => set_method(),
            insert => insert_method(),
            remove => remove_method(),
            len => len_method(),
            clear => clear_method(),
            pop => pop_method(),
            index_of => index_of_method(),
            contains => contains_method(),
            choice => choice_method()
        }
    }
}
