/// Imports
use crate::refs::MutRef;
use crate::{
    builtins::utils::error,
    refs::{EnvRef, Ref},
    rt::{
        env::Environment,
        value::{Callable, Native, Value},
    },
};
use std::{cell::RefCell, rc::Rc};
use tick_common::bug;

/// Set var definition
pub fn set_var() -> Ref<Native> {
    Ref::new(Native {
        arity: 2,
        function: Box::new(|_, _, values| {
            let key = values.get(0).map(|v| v.to_string()).unwrap_or_default();
            if !key.is_empty() {
                // Safety: setting variable is safe because of single-threaded runtime
                unsafe {
                    std::env::set_var(
                        key,
                        values.get(1).map(|v| v.to_string()).unwrap_or_default(),
                    )
                };
            }
            Value::Null
        }),
    })
}

/// Get var definition
pub fn get_var() -> Ref<Native> {
    Ref::new(Native {
        arity: 1,
        function: Box::new(|_, _, values| {
            match std::env::var(values.get(0).map(|v| v.to_string()).unwrap_or_default()) {
                Ok(val) => Value::String(val),
                Err(_) => Value::Null,
            }
        }),
    })
}

/// Unset definition
pub fn unset() -> Ref<Native> {
    Ref::new(Native {
        arity: 1,
        function: Box::new(|_, _, values| unsafe {
            std::env::remove_var(values.get(0).unwrap().to_string());
            Value::Null
        }),
    })
}

/// Var definition
pub fn var() -> Ref<Native> {
    Ref::new(Native {
        arity: 1,
        function: Box::new(|_, span, values| {
            match std::env::var_os(values.get(0).map(|v| v.to_string()).unwrap_or_default()) {
                Some(val) => Value::String(val.to_string_lossy().into_owned()),
                None => error(span, "os variable is not set"),
            }
        }),
    })
}

/// Current workind directory definition
pub fn cwd() -> Ref<Native> {
    Ref::new(Native {
        arity: 0,
        function: Box::new(|_, span, _| match std::env::current_dir() {
            Ok(path) => Value::String(path.to_string_lossy().into_owned()),
            Err(_) => error(span, "failed to get current work directory"),
        }),
    })
}

/// Home directory definition
pub fn home() -> Ref<Native> {
    Ref::new(Native {
        arity: 0,
        function: Box::new(|_, span, _| match std::env::home_dir() {
            Some(path) => Value::String(path.to_string_lossy().into_owned()),
            None => error(span, "could not determine home directory"),
        }),
    })
}

/// Command line arguments
pub fn args() -> Ref<Native> {
    Ref::new(Native {
        arity: 0,
        function: Box::new(|rt, span, _| {
            // Retrieving list type
            let list_builtin = rt
                .builtins
                .env
                .borrow()
                .lookup("List")
                .unwrap_or_else(|| error(span, "list builtin is not found"));
            
            // Instantiating list instance
            match list_builtin {
                Value::Type(list_ty) => match rt.call_type(span, Vec::new(), list_ty) {
                    Ok(val) => match val {
                        // Setting up internal vector
                        Value::Instance(list) => {
                            list.borrow_mut().fields.insert(
                                "$internal".to_string(),
                                Value::Any(MutRef::new(RefCell::new(
                                    std::env::args()
                                        .map(|a| Value::String(a))
                                        .collect::<Vec<Value>>(),
                                ))),
                            );
                            Value::Instance(list)
                        }
                        _ => bug!("`call_type` returned non-instance value"),
                    },
                    Err(_) => {
                        bug!("control flow leak");
                    }
                },
                _ => error(span, "list builtin is not a type"),
            }
        }),
    })
}

/// Provides `env` module env
pub fn provide_env() -> EnvRef {
    let mut env = Environment::default();

    env.force_define("set_var", Value::Callable(Callable::Native(set_var())));
    env.force_define("get_var", Value::Callable(Callable::Native(get_var())));
    env.force_define("unset", Value::Callable(Callable::Native(unset())));
    env.force_define("var", Value::Callable(Callable::Native(var())));
    env.force_define("cwd", Value::Callable(Callable::Native(cwd())));
    env.force_define("home", Value::Callable(Callable::Native(home())));
    env.force_define("args", Value::Callable(Callable::Native(args())));
    env.force_define("arch", Value::String(std::env::consts::ARCH.to_string()));
    env.force_define("os", Value::String(std::env::consts::OS.to_string()));
    env.force_define(
        "family",
        Value::String(std::env::consts::FAMILY.to_string()),
    );
    env.force_define(
        "dll",
        Value::String(std::env::consts::DLL_EXTENSION.to_string()),
    );
    env.force_define(
        "exe",
        Value::String(std::env::consts::EXE_EXTENSION.to_string()),
    );

    Rc::new(RefCell::new(env))
}
