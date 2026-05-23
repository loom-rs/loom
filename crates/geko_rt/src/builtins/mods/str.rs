/// Imports
use crate::{
    arg, arg_ref, builtin_class, callable, error, expect, native_fun, realm,
    refs::{MutRef, RealmRef, Ref},
    rt::{
        realm::Realm,
        value::{Native, Value},
    },
};
use geko_common::bug;
use std::cell::RefCell;

/// Replace `from` with `to` in `str`
fn replace() -> Ref<Native> {
    native_fun! {
        arity = 3,
        fun = |_, span, values| {
            let str = expect!(&span, arg!(values, 0), Value::String);
            let from = expect!(&span, arg!(values, 1), Value::String);
            let to = expect!(&span, arg!(values, 2), Value::String);

            Value::String(str.replace(&from, &to))
        }
    }
}

/// Replacen `from` with `to` in `str`
fn replacen() -> Ref<Native> {
    native_fun! {
        arity = 4,
        fun = |_, span, values| {
            let str = expect!(&span, arg!(values, 0), Value::String);
            let from = expect!(&span, arg!(values, 1), Value::String);
            let to = expect!(&span, arg!(values, 2), Value::String);
            let count = match arg_ref!(values, 2) {
                Value::Int(int) if *int >= 0 => *int as usize,
                _ => error!(span, "`count` expected to be a positive int"),
            };

            Value::String(str.replacen(&from, &to, count))
        }
    }
}

/// Chars list
fn chars() -> Ref<Native> {
    native_fun! {
        arity = 1,
        fun = |rt, span, values| {
            // Retrieving passed string
            let str = expect!(&span, arg!(values, 0), Value::String);

            // Retrieving string chars
            let chars = str.chars().map(|c| Value::String(c.to_string())).collect::<Vec<Value>>();

            // Retrieving list class
            let class = builtin_class!(rt, "List");

            // Calling class
            match rt.call_class(span, Vec::new(), class) {
                Ok(Value::Instance(list)) => {
                    list.borrow_mut().fields.insert(
                        "$internal".to_string(),
                        Value::Any(MutRef::new(RefCell::new(chars))),
                    );
                    Value::Instance(list)
                }
                _ => bug!("invalid list call"),
            }
        }
    }
}

/// Split string by separator
fn split() -> Ref<Native> {
    native_fun! {
        arity = 2,
        fun = |rt, span, values| {
            // Retrieving passed string and sep
            let str = expect!(&span, arg!(values, 0), Value::String);
            let sep = expect!(&span, arg!(values, 1), Value::String);

            // Splitting string
            let vec = str.split(&sep).map(|s| Value::String(s.to_string())).collect::<Vec<Value>>();

            // Retrieving list class
            let class = builtin_class!(rt, "List");

            // Calling class
            match rt.call_class(span, Vec::new(), class) {
                Ok(Value::Instance(list)) => {
                    list.borrow_mut().fields.insert(
                        "$internal".to_string(),
                        Value::Any(MutRef::new(RefCell::new(vec))),
                    );
                    Value::Instance(list)
                }
                _ => bug!("invalid list call"),
            }
        }
    }
}

/// Contains string
fn contains() -> Ref<Native> {
    native_fun! {
        arity = 2,
        fun = |_, span, values| {
            let str = expect!(&span, arg!(values, 0), Value::String);
            let what = expect!(&span, arg!(values, 1), Value::String);

            Value::Bool(str.contains(&what))
        }
    }
}

/// Starts with prefix
fn starts_with() -> Ref<Native> {
    native_fun! {
        arity = 2,
        fun = |_, span, values| {
            let str = expect!(&span, arg!(values, 0), Value::String);
            let what = expect!(&span, arg!(values, 1), Value::String);

            Value::Bool(str.starts_with(&what))
        }
    }
}

/// Ends with suffix
fn ends_with() -> Ref<Native> {
    native_fun! {
        arity = 2,
        fun = |_, span, values| {
            let str = expect!(&span, arg!(values, 0), Value::String);
            let what = expect!(&span, arg!(values, 1), Value::String);

            Value::Bool(str.ends_with(&what))
        }
    }
}

/// Trims spaces
fn trim() -> Ref<Native> {
    native_fun! {
        arity = 1,
        fun = |_, span, values| {
            let str = expect!(&span, arg!(values, 0), Value::String);
            Value::String(str.trim().to_string())
        }
    }
}

/// Trims start spaces
fn trim_start() -> Ref<Native> {
    native_fun! {
        arity = 1,
        fun = |_, span, values| {
            let str = expect!(&span, arg!(values, 0), Value::String);
            Value::String(str.trim_start().to_string())
        }
    }
}

/// Trims end spaces
fn trim_end() -> Ref<Native> {
    native_fun! {
        arity = 1,
        fun = |_, span, values| {
            let str = expect!(&span, arg!(values, 0), Value::String);
            Value::String(str.trim_end().to_string())
        }
    }
}

/// Strip prefix of string
fn strip_prefix() -> Ref<Native> {
    native_fun! {
        arity = 2,
        fun = |_, span, values| {
            let str = expect!(&span, arg!(values, 0), Value::String);
            let prefix = expect!(&span, arg!(values, 1), Value::String);

            Value::String(str.strip_prefix(&prefix).unwrap_or(&str).to_string())
        }
    }
}

/// Strip suffix of string
fn strip_suffix() -> Ref<Native> {
    native_fun! {
        arity = 2,
        fun = |_, span, values| {
            let str = expect!(&span, arg!(values, 0), Value::String);
            let prefix = expect!(&span, arg!(values, 1), Value::String);

            Value::String(str.strip_suffix(&prefix).unwrap_or(&str).to_string())
        }
    }
}

/// Convert string to uppercase
fn to_upper() -> Ref<Native> {
    native_fun! {
        arity = 1,
        fun = |_, span, values| {
            let str = expect!(&span, arg!(values, 0), Value::String);

            Value::String(str.to_uppercase())
        }
    }
}

/// Convert string to lowercase
fn to_lower() -> Ref<Native> {
    native_fun! {
        arity = 1,
        fun = |_, span, values| {
            let str = expect!(&span, arg!(values, 0), Value::String);

            Value::String(str.to_lowercase())
        }
    }
}

/// Convert string to ascii uppercase
fn to_ascii_upper() -> Ref<Native> {
    native_fun! {
        arity = 1,
        fun = |_, span, values| {
            let str = expect!(&span, arg!(values, 0), Value::String);

            Value::String(str.to_ascii_uppercase())
        }
    }
}

/// Convert string to ascii lowercase
fn to_ascii_lower() -> Ref<Native> {
    native_fun! {
        arity = 1,
        fun = |_, span, values| {
            let str = expect!(&span, arg!(values, 0), Value::String);

            Value::String(str.to_ascii_lowercase())
        }
    }
}

/// Matches of `a` in `b`
fn matches() -> Ref<Native> {
    native_fun! {
        arity = 2,
        fun = |_, span, values| {
            let a = expect!(&span, arg!(values, 0), Value::String);
            let b = expect!(&span, arg!(values, 1), Value::String);

            Value::Int(a.matches(&b).count() as i64)
        }
    }
}

/// Repeats string `n` times
fn repeat() -> Ref<Native> {
    native_fun! {
        arity = 2,
        fun = |_, span, values| {
            let str = expect!(&span, arg!(values, 0), Value::String);
            let n = match arg_ref!(values, 1) {
                Value::Int(int) if *int >= 0 => *int as usize,
                _ => error!(span, "`n` expected to be a positive int"),
            };

            Value::String(str.repeat(n))
        }
    }
}

/// Provides `mem` module env
pub fn provide_env() -> RealmRef {
    realm! {
        replace => callable!(replace()),
        replacen => callable!(replacen()),
        chars => callable!(chars()),
        split => callable!(split()),
        contains => callable!(contains()),
        starts_with => callable!(starts_with()),
        ends_with => callable!(ends_with()),
        trim => callable!(trim()),
        trim_start => callable!(trim_start()),
        trim_end => callable!(trim_end()),
        strip_prefix => callable!(strip_prefix()),
        strip_suffix => callable!(strip_suffix()),
        to_upper => callable!(to_upper()),
        to_lower => callable!(to_lower()),
        to_ascii_upper => callable!(to_ascii_upper()),
        to_ascii_lower => callable!(to_ascii_lower()),
        matches => callable!(matches()),
        repeat => callable!(repeat())
    }
}
