/// Modules
mod core;
mod dict;
mod list;
mod macros;
mod mods;
mod result;
pub mod utils;

/// Imports
use crate::{
    refs::{MutRef, RealmRef},
    rt::value::Module,
};
use std::collections::HashMap;

/// Built-in definitions
pub(crate) struct Builtins {
    /// Built-in variables and functions (print, println, readln, etc.)
    pub(crate) env: RealmRef,
    /// Built-in modules
    pub(crate) modules: HashMap<String, MutRef<Module>>,
}

/// Provides builtins
pub fn provide_builtins() -> Builtins {
    Builtins {
        env: core::provide_env(),
        modules: mods::provide_modules(),
    }
}
