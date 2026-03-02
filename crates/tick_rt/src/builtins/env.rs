/// Imports
use crate::{
    builtins::utils::error,
    refs::{EnvRef, Ref},
    rt::{
        env::Environment,
        value::{Callable, Native, Value},
    },
};
use std::{cell::RefCell, rc::Rc};

/// Set var definition
pub fn set_var() -> Ref<Native> {
    Ref::new(Native {
        arity: 2,
        function: Box::new(|_, _, values| {
            let key = values.get(0).map(|v| v.to_string()).unwrap_or_default();
            if !key.is_empty() {
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

/// Home directory definitionn
pub fn home() -> Ref<Native> {
    Ref::new(Native {
        arity: 0,
        function: Box::new(|_, span, _| match std::env::home_dir() {
            Some(path) => Value::String(path.to_string_lossy().into_owned()),
            None => error(span, "could not determine home directory"),
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
