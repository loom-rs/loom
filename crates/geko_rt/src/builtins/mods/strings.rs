use crate::{
    builtins::utils,
    builtin_class, callable, native_fun, realm,
    refs::{RealmRef, Ref, MutRef},
    rt::{
        realm::Realm,
        value::{Native, Value},
    },
};

use geko_common::bug;
use std::{cell::RefCell, cmp};

// Helper: compare two strings lexicographically
fn cmp_strings(str1: &String, str2: &String) -> i64 {
  match str1.cmp(str2) {
      cmp::Ordering::Less => -1,
      cmp::Ordering::Equal => 0,
      cmp::Ordering::Greater => 1,
  }
}

/// Check if a string contains a substring
fn contains() -> Ref<Native> {
    native_fun! {
      arity = 2,
      fun = | _, span, values| {
        match (values.first().unwrap(), values.get(1).unwrap()) {
            (Value::String(str), Value::String(substr)) => Value::Bool(str.contains(substr)),
            _ => utils::error(span, "arguments are expected to be strings")
        }
      }
    }
}

/// Check is a string starts with a subctring
fn contains_prefix() -> Ref<Native> {
    native_fun! {
      arity = 2,
      fun = | _, span, values| {
        match (values.first().unwrap(), values.get(1).unwrap()) {
            (Value::String(str), Value::String(substr)) => Value::Bool(str.starts_with(substr)),
            _ => utils::error(span, "arguments are expected to be strings")
        }
      }
    }
}

/// Check is a string end with a subctring
fn contains_suffix() -> Ref<Native> {
    native_fun! {
      arity = 2,
      fun = | _, span, values| {
        match (values.first().unwrap(), values.get(1).unwrap()) {
            (Value::String(str), Value::String(substr)) => Value::Bool(str.ends_with(substr)),
            _ => utils::error(span, "arguments are expected to be strings")
        }
      }
    }
}

/// Checks whether a string is written in ASCII encoding.
fn is_ascii() -> Ref<Native> {
    native_fun! {
      arity = 1,
      fun = | _, span, values| {
        match values.first().unwrap() {
            Value::String(str) => {
              Value::Bool(str.is_ascii())
            },
            _ => utils::error(span, "argument is expected to be string")
        }
      }
    }
}

/// Splits a string by a substring
fn split() -> Ref<Native> {
  native_fun! {
    arity = 2,
    fun = | rt, span, values| {
      match (values.first().unwrap(), values.get(1).unwrap()) {
            (Value::String(str), Value::String(sep)) => {
              let class = builtin_class!(rt, "List");

            // Calling class
            match rt.call_class(span, Vec::new(), class) {
                Ok(val) => match val {
                    // Setting up internal vector
                    Value::Instance(list) => {
                        list.borrow_mut().fields.insert(
                            "$internal".to_string(),
                            Value::Any(MutRef::new(RefCell::new(
                                str.split(sep).map(|x| Value::String(x.to_string())).collect::<Vec<Value>>(),
                            ))),
                        );
                        Value::Instance(list)
                    }
                    _ => bug!("`call_class` returned non-instance value"),
                },
                Err(_) => bug!("control flow leak"),
            }
            },
            _ => utils::error(span, "arguments are expected to be strings")
        }
    }
  }
}

/// Splits a string at N index
fn split_at() -> Ref<Native> {
  native_fun! {
    arity = 2,
    fun = | rt, span, values| {
      match (values.first().unwrap(), values.get(1).unwrap()) {
            (Value::String(str), Value::Int(index)) => {
              let class = builtin_class!(rt, "List");

            // Calling class
            match rt.call_class(span, Vec::new(), class) {
                Ok(val) => match val {
                  
                  // Setting up internal vector
                  Value::Instance(list) => {
                        let res = str.split_at(*index as usize);
                        let vec = vec![Value::String(res.0.to_string()), Value::String(res.1.to_string())];

                        list.borrow_mut().fields.insert(
                            "$internal".to_string(),
                            Value::Any(MutRef::new(RefCell::new(
                                vec,
                            ))),
                        );
                        Value::Instance(list)
                    }
                    _ => bug!("`call_class` returned non-instance value"),
                },
                Err(_) => bug!("control flow leak"),
            }
            },
            _ => utils::error(span, "arguments are expected to be strings")
        }
    }
  }
}

/// Rearranges the first N substrings from a string to new ones
fn replacen() -> Ref<Native> {
  native_fun! {
      arity = 4,
      fun = | _, span, values| {
        match (values.first().unwrap(), values.get(1).unwrap(), values.get(2).unwrap(),values.get(3).unwrap()) {
            (Value::String(str), Value::String(old), Value::String(new), Value::Int(n)) => {
              Value::String(str.replacen(old, new, *n as usize))
            },
            (Value::String(str), Value::String(old), Value::String(new), Value::Float(n)) => {
              Value::String(str.replacen(old, new, *n as usize))
            },
            _ => utils::error(span, "first three arguments are expected to be strings and last is expected to be number")
        }
      }
    }
}

/// Rearranges all substrings from a string into new ones.
fn replace() -> Ref<Native> {
  native_fun! {
      arity = 3,
      fun = | _, span, values| {
        match (values.first().unwrap(), values.get(1).unwrap(), values.get(2).unwrap(),) {
            (Value::String(str), Value::String(old), Value::String(new)) => {
              Value::String(str.replace(old, new))
            },
            _ => utils::error(span, "arguments are expected to be strings")
        }
      }
    }
}

/// Creates a new string by repeating a string N times
fn repeat() -> Ref<Native> {
  native_fun! {
      arity = 2,
      fun = | _, span, values| {
        match (values.first().unwrap(), values.get(1).unwrap()) {
            (Value::String(str), Value::Int(n)) => {
              Value::String(str.repeat(*n as usize))
            },
            (Value::String(str), Value::Float(n)) => {
              Value::String(str.repeat(*n as usize))
            },
            _ => utils::error(span, "arguments are expected to be strings")
        }
      }
    }
}

/// Returns a substring of the string s,
/// with all leading and trailing white space removed, as defined by Unicode.
fn trim() -> Ref<Native> {
  native_fun! {
      arity = 1,
      fun = | _, span, values| {
        match values.first().unwrap() {
            Value::String(str) => {
              Value::String(str.trim().to_string())
            },
            _ => utils::error(span, "argument is expected to be string")
        }
      }
    }
}

/// Returns the lowercase equivalent of this string
fn to_lowercase() -> Ref<Native> {
  native_fun! {
      arity = 1,
      fun = | _, span, values| {
        match values.first().unwrap() {
            Value::String(str) => {
              Value::String(str.to_lowercase())
            },
            _ => utils::error(span, "argument is expected to be string")
        }
      }
    }
}

/// Returns the uppercase equivalent of this string
fn to_uppercase() -> Ref<Native> {
  native_fun! {
      arity = 1,
      fun = | _, span, values| {
        match values.first().unwrap() {
            Value::String(str) => {
              Value::String(str.to_uppercase())
            },
            _ => utils::error(span, "argument is expected to be string")
        }
      }
    }
}

fn to_titlecase() -> Ref<Native> {
  native_fun! {
    arity = 1,
    fun = | _, span, values| {
      match values.first().unwrap() {
        Value::String(str) => {
          Value::String(titlecase::titlecase(str))
        },
        
        _ => utils::error(span, "argument is expected to be string")
      }
    }  
  }
}

/// Returns an integer comparing two strings lexicographically
fn compare() -> Ref<Native> {
  native_fun! {
      arity = 2,
      fun = | _, span, values| {
        match (values.first().unwrap(), values.get(1).unwrap()) {
            (Value::String(str1), Value::String(str2)) => {
              Value::Int(cmp_strings(str1, str2))
            },
            _ => utils::error(span, "arguments are expected to be strings")
        }
      }
    }
}

fn count() -> Ref<Native> {
  native_fun! {
    arity = 2,
    fun = | _, span, values| {
      match (values.first().unwrap(), values.get(1).unwrap()) {
          (Value::String(str), Value::String(substr)) => {
            Value::Int(str.matches(substr).count() as i64)
          },
          _ => utils::error(span, "arguments are expected to be strings")
      }
    }
  }
}



/// Provides `strings` module env
pub fn provide_env() -> RealmRef {
    realm!(
      contains => callable!(contains()),
      contains_prefix => callable!(contains_prefix()),
      contains_suffix => callable!(contains_suffix()),
      is_ascii => callable!(is_ascii()),
      split => callable!(split()),
      split_at => callable!(split_at()),
      replacen => callable!(replacen()),
      replace => callable!(replace()),
      repeat => callable!(repeat()),
      trim => callable!(trim()),
      to_lowercase => callable!(to_lowercase()),
      to_uppercase => callable!(to_uppercase()),
      to_titlecase => callable!(to_titlecase()),
      compare => callable!(compare()),
      count => callable!(count()),
    )
}
