/// Imports
use crate::{
    builtins::utils,
    interpreter::Interpreter,
    refs::{MutRef, Ref},
    rt::value::{Class, Instance, Method, Native, Value},
};
use squirrel_common::bug;
use squirrel_lex::token::Span;
use std::{cell::RefCell, collections::HashMap};

/// Helper: validates dict
fn validate_dict<F, V>(span: &Span, dict: Value, f: F) -> V
where
    F: FnOnce(&mut HashMap<Value, Value>) -> V,
{
    match dict {
        Value::Instance(instance) => {
            // Safety: borrow is temporal for this line
            let internal = instance
                .borrow_mut()
                .fields
                .get("$internal")
                .cloned()
                .unwrap();

            match internal {
                Value::Any(map) => match map.borrow_mut().downcast_mut::<HashMap<Value, Value>>() {
                    Some(map) => f(map),
                    _ => utils::error(span, "corrupted dict"),
                },
                _ => {
                    utils::error(span, "corrupted dict");
                }
            }
        }
        _ => unreachable!(),
    }
}

/// Helper: validates dict argument
fn validate_dict_arg<F, V>(span: &Span, values: &[Value], f: F) -> V
where
    F: FnOnce(&mut HashMap<Value, Value>) -> V,
{
    validate_dict(span, values.get(0).cloned().unwrap(), f)
}

/// Helper: makes new list
fn make_list(rt: &mut Interpreter, span: &Span) -> MutRef<Instance> {
    let list_value = rt
        .builtins
        .env
        .borrow()
        .lookup("List")
        .unwrap_or_else(|| bug!("no builtin `List` found"));

    match list_value {
        Value::Class(t) => match rt.call_class(span, Vec::new(), t) {
            Ok(Value::Instance(instance)) => instance,
            Ok(_) => unreachable!(),
            Err(err) => {
                bug!(format!(
                    "calling of builtin `List` has ended with a control flow leak: {err:?}"
                ))
            }
        },
        _ => bug!("builtin `List` is not a class"),
    }
}

/// Init method
fn init_method() -> Method {
    Method::Native(Ref::new(Native {
        arity: 1,
        function: Box::new(|_, _, values| {
            let dict = values.first().cloned().unwrap();
            match dict {
                Value::Instance(instance) => {
                    let vec = Value::Any(MutRef::new(RefCell::new(HashMap::<Value, Value>::new())));

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
            validate_dict_arg(span, &values, |map| Value::String(format!("{map:?}")))
        }),
    }))
}

/// Get method
fn get_method() -> Method {
    Method::Native(Ref::new(Native {
        arity: 2,
        function: Box::new(|_, span, values| {
            validate_dict_arg(span, &values, |map| {
                map.get(&values.get(1).cloned().unwrap())
                    .cloned()
                    .unwrap_or(Value::Null)
            })
        }),
    }))
}

/// Insert method
fn insert_method() -> Method {
    Method::Native(Ref::new(Native {
        arity: 3,
        function: Box::new(|_, span, values| {
            validate_dict_arg(span, &values, |map| {
                map.insert(
                    values.get(1).cloned().unwrap(),
                    values.get(2).cloned().unwrap(),
                );
                Value::Null
            })
        }),
    }))
}

/// Remove method
fn remove_method() -> Method {
    Method::Native(Ref::new(Native {
        arity: 2,
        function: Box::new(|_, span, values| {
            validate_dict_arg(span, &values, |map| {
                map.remove(&values.get(0).cloned().unwrap());
                Value::Null
            })
        }),
    }))
}

/// Len method
fn len_method() -> Method {
    Method::Native(Ref::new(Native {
        arity: 1,
        function: Box::new(|_, span, values| {
            validate_dict_arg(span, &values, |map| Value::Int(map.len() as i64))
        }),
    }))
}

/// Clear method
fn clear_method() -> Method {
    Method::Native(Ref::new(Native {
        arity: 1,
        function: Box::new(|_, span, values| {
            validate_dict_arg(span, &values, |map| {
                map.clear();
                Value::Null
            })
        }),
    }))
}

/// Contains key
fn contains_key_method() -> Method {
    Method::Native(Ref::new(Native {
        arity: 2,
        function: Box::new(|_, span, values| {
            validate_dict_arg(span, &values, |map| {
                Value::Bool(map.contains_key(values.get(1).unwrap()))
            })
        }),
    }))
}

/// Keys list
fn keys_method() -> Method {
    Method::Native(Ref::new(Native {
        arity: 1,
        function: Box::new(|rt, span, values| {
            validate_dict_arg(span, &values, |map| {
                // Preparing keys vector
                let keys = map.keys().cloned().collect::<Vec<Value>>();

                // Preparing list for keys
                let list = make_list(rt, span);

                // Setting new vector
                list.borrow_mut().fields.insert(
                    "$internal".to_string(),
                    Value::Any(MutRef::new(RefCell::new(keys))),
                );

                Value::Instance(list)
            })
        }),
    }))
}

/// Values list
fn values_method() -> Method {
    Method::Native(Ref::new(Native {
        arity: 1,
        function: Box::new(|rt, span, values| {
            validate_dict_arg(span, &values, |map| {
                // Preparing values vector
                let values = map.keys().cloned().collect::<Vec<Value>>();

                // Preparing list for keys
                let list = make_list(rt, span);

                // Setting new vector
                list.borrow_mut().fields.insert(
                    "$internal".to_string(),
                    Value::Any(MutRef::new(RefCell::new(values))),
                );

                Value::Instance(list)
            })
        }),
    }))
}

/// Provides dict class
pub fn provide_class() -> Ref<Class> {
    Ref::new(Class {
        name: "Dict".to_string(),
        methods: HashMap::from([
            // Init method
            ("init".to_string(), init_method()),
            // To string method
            ("to_string".to_string(), to_string_method()),
            // Get method
            ("get".to_string(), get_method()),
            // Insert method
            ("insert".to_string(), insert_method()),
            // Remove method
            ("remove".to_string(), remove_method()),
            // Len method
            ("len".to_string(), len_method()),
            // Clear method
            ("clear".to_string(), clear_method()),
            // Contains method
            ("contains_key".to_string(), contains_key_method()),
            // Keys method
            ("keys".to_string(), keys_method()),
            // Values method
            ("values".to_string(), values_method()),
        ]),
    })
}
