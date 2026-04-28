/// Imports
use crate::{
    builtins::utils,
    interpreter::Interpreter,
    refs::{MutRef, Ref},
    rt::value::{Class, Instance, Method, Native, Value},
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
    match list {
        Value::Instance(instance) => {
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
                    _ => utils::error(span, "corrupted list"),
                },
                _ => {
                    utils::error(span, "corrupted list");
                }
            }
        }
        _ => unreachable!(),
    }
}

/// Helper: validates list argument
fn validate_list_arg<F, V>(span: &Span, values: &[Value], f: F) -> V
where
    F: FnOnce(&mut Vec<Value>) -> V,
{
    validate_list(span, values.first().cloned().unwrap(), f)
}

/// Helper: validates index
fn validate_idx<F, V>(span: &Span, idx: Value, len: usize, f: F) -> V
where
    F: FnOnce(usize) -> V,
{
    match idx {
        Value::Int(idx) => {
            if idx < 0 {
                utils::error(span, "index should be positive int")
            } else {
                let idx = idx as usize;
                if idx >= len {
                    utils::error(span, "index out of bounds")
                } else {
                    f(idx)
                }
            }
        }
        _ => utils::error(span, "index should be an int"),
    }
}

/// Helper: validates index argument
fn validate_idx_arg<F, V>(span: &Span, values: &[Value], idx: usize, len: usize, f: F) -> V
where
    F: FnOnce(usize) -> V,
{
    validate_idx(span, values.get(idx).cloned().unwrap(), len, f)
}

/// Helper: makes new list
pub fn make_list(rt: &mut Interpreter, span: &Span) -> MutRef<Instance> {
    // Getting builtin class
    let class = utils::get_builtin(rt, "List");

    // Matching list value
    match class {
        // Calling class
        Value::Class(class) => match rt.call_class(span, Vec::new(), class) {
            Ok(Value::Instance(instance)) => instance,
            Ok(_) => unreachable!(),
            Err(err) => {
                bug!(format!(
                    "calling of builtin `List` has ended with a control flow leak: {err:?}"
                ))
            }
        },
        _ => bug!(format!("builtin `List` is not a class")),
    }
}

/// Init method
fn init_method() -> Method {
    Method::Native(Ref::new(Native {
        arity: 1,
        function: Box::new(|_, _, values| {
            let list = values.first().cloned().unwrap();
            match list {
                Value::Instance(instance) => {
                    let vec = Value::Any(MutRef::new(RefCell::new(Vec::<Value>::new())));

                    // Safety: borrow is temporal for this line
                    instance
                        .borrow_mut()
                        .fields
                        .insert("$internal".to_string(), vec);

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
            validate_list_arg(span, &values, |vec| Value::String(format!("{vec:?}")))
        }),
    }))
}

/// Push method
fn push_method() -> Method {
    Method::Native(Ref::new(Native {
        arity: 2,
        function: Box::new(|_, span, values| {
            validate_list_arg(span, &values, |vec| {
                vec.push(values.get(1).cloned().unwrap());
                Value::Null
            })
        }),
    }))
}

/// Get method
fn get_method() -> Method {
    Method::Native(Ref::new(Native {
        arity: 2,
        function: Box::new(|_, span, values| {
            validate_list_arg(span, &values, |vec| {
                validate_idx_arg(span, &values, 1, vec.len(), |idx| vec[idx].clone())
            })
        }),
    }))
}

/// Set method
fn set_method() -> Method {
    Method::Native(Ref::new(Native {
        arity: 3,
        function: Box::new(|_, span, values| {
            validate_list_arg(span, &values, |vec| {
                validate_idx_arg(span, &values, 1, vec.len(), |idx| {
                    vec[idx] = values.get(2).cloned().unwrap();
                    Value::Null
                })
            })
        }),
    }))
}

/// Insert method
fn insert_method() -> Method {
    Method::Native(Ref::new(Native {
        arity: 3,
        function: Box::new(|_, span, values| {
            validate_list_arg(span, &values, |vec| {
                validate_idx_arg(span, &values, 1, vec.len(), |idx| {
                    vec.insert(idx, values.get(2).cloned().unwrap());
                    Value::Null
                })
            })
        }),
    }))
}

/// Remove method
fn remove_method() -> Method {
    Method::Native(Ref::new(Native {
        arity: 2,
        function: Box::new(|_, span, values| {
            validate_list_arg(span, &values, |vec| {
                validate_idx_arg(span, &values, 1, vec.len(), |idx| {
                    vec.remove(idx);
                    Value::Null
                })
            })
        }),
    }))
}

/// Len method
fn len_method() -> Method {
    Method::Native(Ref::new(Native {
        arity: 1,
        function: Box::new(|_, span, values| {
            validate_list_arg(span, &values, |vec| Value::Int(vec.len() as i64))
        }),
    }))
}

/// Clear method
fn clear_method() -> Method {
    Method::Native(Ref::new(Native {
        arity: 1,
        function: Box::new(|_, span, values| {
            validate_list_arg(span, &values, |vec| {
                vec.clear();
                Value::Null
            })
        }),
    }))
}

/// Pop method
fn pop_method() -> Method {
    Method::Native(Ref::new(Native {
        arity: 1,
        function: Box::new(|_, span, values| {
            validate_list_arg(span, &values, |vec| vec.pop().unwrap_or(Value::Null))
        }),
    }))
}

/// Index of method
fn index_of_method() -> Method {
    Method::Native(Ref::new(Native {
        arity: 2,
        function: Box::new(|_, span, values| {
            validate_list_arg(span, &values, |vec| {
                let value = values.get(1).cloned().unwrap();
                vec.iter()
                    .position(|v| *v == value)
                    .map(|it| Value::Int(it as i64))
                    .unwrap_or(Value::Int(-1))
            })
        }),
    }))
}

/// Contains
fn contains_method() -> Method {
    Method::Native(Ref::new(Native {
        arity: 2,
        function: Box::new(|_, span, values| {
            validate_list_arg(span, &values, |vec| {
                Value::Bool(vec.contains(values.get(1).unwrap()))
            })
        }),
    }))
}

/// Choice method
fn choice_method() -> Method {
    Method::Native(Ref::new(Native {
        arity: 1,
        function: Box::new(|_, span, values| {
            validate_list_arg(span, &values, |vec| {
                match vec.get(rand::rng().random_range(0..vec.len())) {
                    Some(val) => val.clone(),
                    _ => utils::error(
                        span,
                        "list must have 1 or more elements to perform random choice on it",
                    ),
                }
            })
        }),
    }))
}

/// Provides list class
pub fn provide_class() -> Ref<Class> {
    Ref::new(Class {
        name: "List".to_string(),
        methods: HashMap::from([
            // Init method
            ("init".to_string(), init_method()),
            // To string method
            ("to_string".to_string(), to_string_method()),
            // Push method
            ("push".to_string(), push_method()),
            // Get method
            ("get".to_string(), get_method()),
            // Set method
            ("set".to_string(), set_method()),
            // Insert method
            ("insert".to_string(), insert_method()),
            // Remove method
            ("remove".to_string(), remove_method()),
            // Len method
            ("len".to_string(), len_method()),
            // Clear method
            ("clear".to_string(), clear_method()),
            // Pop method
            ("pop".to_string(), pop_method()),
            // Index of method
            ("index_of".to_string(), index_of_method()),
            // Contains method
            ("contains".to_string(), contains_method()),
            // Choice method
            ("choice".to_string(), choice_method()),
        ]),
    })
}
