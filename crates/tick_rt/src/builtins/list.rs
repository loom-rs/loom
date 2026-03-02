/// Imports
use crate::{
    builtins::utils,
    refs::{MutRef, Ref},
    rt::value::{Method, Native, Type, Value},
};
use std::{cell::RefCell, collections::HashMap};

/// Init method
fn init_method() -> Method {
    Method::Native(Ref::new(Native {
        arity: 1,
        function: Box::new(|_, _, values| {
            let list = values.get(0).cloned().unwrap();
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
            let list = values.get(0).cloned().unwrap();
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
                            Some(vec) => Value::String(format!("{vec:?}")),
                            _ => utils::error(span, "corrupted list."),
                        },
                        _ => {
                            utils::error(span, "corrupted list.");
                        }
                    }
                }
                _ => unreachable!(),
            }
        }),
    }))
}

/// Push method
fn push_method() -> Method {
    Method::Native(Ref::new(Native {
        arity: 2,
        function: Box::new(|_, span, values| {
            let list = values.get(0).cloned().unwrap();
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
                            Some(vec) => {
                                vec.push(values.get(1).cloned().unwrap());
                                Value::Null
                            }
                            _ => utils::error(span, "corrupted list."),
                        },
                        _ => {
                            utils::error(span, "corrupted list.");
                        }
                    }
                }
                _ => unreachable!(),
            }
        }),
    }))
}

/// Get method
fn get_method() -> Method {
    Method::Native(Ref::new(Native {
        arity: 2,
        function: Box::new(|_, span, values| {
            let list = values.get(0).cloned().unwrap();
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
                            Some(vec) => match values.get(1).cloned().unwrap() {
                                Value::Int(idx) => {
                                    if idx < 0 {
                                        utils::error(span, "index should be positive int")
                                    } else {
                                        let idx = idx as usize;
                                        if idx >= vec.len() {
                                            utils::error(span, "index out of bounds")
                                        } else {
                                            vec[idx].clone()
                                        }
                                    }
                                }
                                _ => utils::error(span, "index should be an int"),
                            },
                            _ => utils::error(span, "corrupted list."),
                        },
                        _ => {
                            utils::error(span, "corrupted list.");
                        }
                    }
                }
                _ => unreachable!(),
            }
        }),
    }))
}

/// Set method
fn set_method() -> Method {
    Method::Native(Ref::new(Native {
        arity: 3,
        function: Box::new(|_, span, values| {
            let list = values.get(0).cloned().unwrap();
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
                            Some(vec) => {
                                match values.get(1).cloned().unwrap() {
                                    Value::Int(idx) => {
                                        if idx < 0 {
                                            utils::error(span, "index should be positive int")
                                        } else {
                                            let idx = idx as usize;
                                            if idx >= vec.len() {
                                                utils::error(span, "index out of bounds")
                                            } else {
                                                vec[idx] = values.get(2).cloned().unwrap()
                                            }
                                        }
                                    },
                                    _ => utils::error(span, "index should be an int"),
                                };
                                Value::Null
                            }
                            _ => utils::error(span, "corrupted list."),
                        },
                        _ => {
                            utils::error(span, "corrupted list.");
                        }
                    }
                }
                _ => unreachable!(),
            }
        }),
    }))
}

/// Insert method
fn insert_method() -> Method {
    Method::Native(Ref::new(Native {
        arity: 3,
        function: Box::new(|_, span, values| {
            let list = values.get(0).cloned().unwrap();
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
                            Some(vec) => {
                                match values.get(1).cloned().unwrap() {
                                    Value::Int(idx) => {
                                        if idx < 0 {
                                            utils::error(span, "index should be positive int")
                                        } else {
                                            let idx = idx as usize;
                                            if idx > vec.len() {
                                                utils::error(span, "index out of bounds")
                                            } else {
                                                vec.insert(
                                                    idx,
                                                    values.get(2).cloned().unwrap(),
                                                )
                                            }
                                        }
                                    },
                                    _ => utils::error(span, "index should be an int"),
                                };
                                Value::Null
                            }
                            _ => utils::error(span, "corrupted list."),
                        },
                        _ => {
                            utils::error(span, "corrupted list.");
                        }
                    }
                }
                _ => unreachable!(),
            }
        }),
    }))
}

/// Remove method
fn remove_method() -> Method {
    Method::Native(Ref::new(Native {
        arity: 2,
        function: Box::new(|_, span, values| {
            let list = values.get(0).cloned().unwrap();
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
                            Some(vec) => {
                                match values.get(1).cloned().unwrap() {
                                    Value::Int(idx) => {
                                        if idx < 0 {
                                            utils::error(span, "index should be positive int")
                                        } else {
                                            let idx = idx as usize;
                                            if idx >= vec.len() {
                                                utils::error(span, "index out of bounds")
                                            } else {
                                                vec.remove(idx);
                                            }
                                        }
                                    },
                                    _ => utils::error(span, "index should be an int"),
                                };
                                Value::Null
                            }
                            _ => utils::error(span, "corrupted list."),
                        },
                        _ => {
                            utils::error(span, "corrupted list.");
                        }
                    }
                }
                _ => unreachable!(),
            }
        }),
    }))
}

/// Len method
fn len_method() -> Method {
    Method::Native(Ref::new(Native {
        arity: 1,
        function: Box::new(|_, span, values| {
            let list = values.get(0).cloned().unwrap();
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
                            Some(vec) => Value::Int(vec.len() as i64),
                            _ => utils::error(span, "corrupted list."),
                        },
                        _ => {
                            utils::error(span, "corrupted list.");
                        }
                    }
                }
                _ => unreachable!(),
            }
        }),
    }))
}

/// Clear method
fn clear_method() -> Method {
    Method::Native(Ref::new(Native {
        arity: 1,
        function: Box::new(|_, span, values| {
            let list = values.get(0).cloned().unwrap();
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
                            Some(vec) => {
                                vec.clear();
                                Value::Null
                            }
                            _ => utils::error(span, "corrupted list."),
                        },
                        _ => {
                            utils::error(span, "corrupted list.");
                        }
                    }
                }
                _ => unreachable!(),
            }
        }),
    }))
}

/// Pop method
fn pop_method() -> Method {
    Method::Native(Ref::new(Native {
        arity: 1,
        function: Box::new(|_, span, values| {
            let list = values.get(0).cloned().unwrap();
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
                            Some(vec) => vec.pop().unwrap_or(Value::Null),
                            _ => utils::error(span, "corrupted list."),
                        },
                        _ => {
                            utils::error(span, "corrupted list.");
                        }
                    }
                }
                _ => unreachable!(),
            }
        }),
    }))
}

/// Index of method
fn index_of_method() -> Method {
    Method::Native(Ref::new(Native {
        arity: 2,
        function: Box::new(|_, span, values| {
            let list = values.get(0).cloned().unwrap();
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
                            Some(vec) => {
                                let value = values.get(1).cloned().unwrap();
                                vec.iter()
                                    .position(|v| *v == value)
                                    .map(|it| Value::Int(it as i64))
                                    .unwrap_or(Value::Int(-1))
                            }
                            _ => utils::error(span, "corrupted list."),
                        },
                        _ => {
                            utils::error(span, "corrupted list.");
                        }
                    }
                }
                _ => unreachable!(),
            }
        }),
    }))
}

/// Contains
fn contains_method() -> Method {
    Method::Native(Ref::new(Native {
        arity: 2,
        function: Box::new(|_, span, values| {
            let list = values.get(0).cloned().unwrap();
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
                            Some(vec) => Value::Bool(vec.contains(values.get(1).unwrap())),
                            _ => utils::error(span, "corrupted list."),
                        },
                        _ => {
                            utils::error(span, "corrupted list.");
                        }
                    }
                }
                _ => unreachable!(),
            }
        }),
    }))
}

/// Provides list type
pub fn provide_type() -> Ref<Type> {
    Ref::new(Type {
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
        ]),
    })
}
