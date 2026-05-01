/// Imports
use crate::refs::MutRef;
use crate::{builtin_class, callable, native_fun, realm};
use crate::{
    builtins::utils::error,
    refs::{RealmRef, Ref},
    rt::{
        realm::Realm,
        value::{Native, Value},
    },
};
use geko_common::bug;
use std::cell::RefCell;

/// Set var definition
pub fn set_var() -> Ref<Native> {
    native_fun! {
        arity = 2,
        fun = |_, _, values| {
            let key = values.first().map(|v| v.to_string()).unwrap_or_default();
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
        }
    }
}

/// Get var definition
pub fn get_var() -> Ref<Native> {
    native_fun! {
        arity = 1,
        fun = |_, _, values| {
            match std::env::var(values.first().map(|v| v.to_string()).unwrap_or_default()) {
                Ok(val) => Value::String(val),
                Err(_) => Value::Null,
            }
        }
    }
}

/// Unset definition
pub fn unset() -> Ref<Native> {
    native_fun! {
        arity = 1,
        fun = |_, _, values| { unsafe {
            std::env::remove_var(values.first().unwrap().to_string());
            Value::Null
        }}
    }
}

/// Var definition
pub fn var() -> Ref<Native> {
    native_fun! {
        arity = 1,
        fun = |_, span, values| {
            match std::env::var_os(values.first().map(|v| v.to_string()).unwrap_or_default()) {
                Some(val) => Value::String(val.to_string_lossy().into_owned()),
                None => error(span, "os variable is not set"),
            }
        }
    }
}

/// Current workind directory definition
pub fn cwd() -> Ref<Native> {
    native_fun! {
        arity = 0,
        fun = |_, span, _| {
            match std::env::current_dir() {
                Ok(path) => Value::String(path.to_string_lossy().into_owned()),
                Err(_) => error(span, "failed to get current work directory"),
            }
        }
    }
}

/// Home directory definition
pub fn home() -> Ref<Native> {
    native_fun! {
        arity = 0,
        fun = |_, span, _| {
            match std::env::home_dir() {
                Some(path) => Value::String(path.to_string_lossy().into_owned()),
                None => error(span, "could not determine home directory"),
            }
        }
    }
}

/// Command line arguments
pub fn args() -> Ref<Native> {
    native_fun! {
        arity = 0,
        fun = |rt, span, _| {
            // Retrieving list class
            let class = builtin_class!(rt, "List");

            // Calling class
            match rt.call_class(span, Vec::new(), class) {
                Ok(val) => match val {
                    // Setting up internal vector
                    Value::Instance(list) => {
                        list.borrow_mut().fields.insert(
                            "$internal".to_string(),
                            Value::Any(MutRef::new(RefCell::new(
                                std::env::args().map(Value::String).collect::<Vec<Value>>(),
                            ))),
                        );
                        Value::Instance(list)
                    }
                    _ => bug!("`call_class` returned non-instance value"),
                },
                Err(_) => bug!("control flow leak"),
            }
        }
    }
}

/// Provides `env` module env
pub fn provide_env() -> RealmRef {
    realm! {
        set_var => callable!(set_var()),
        get_var => callable!(get_var()),
        unset => callable!(unset()),
        var => callable!(var()),
        cwd => callable!(cwd()),
        home => callable!(home()),
        args => callable!(args())
    }
}
