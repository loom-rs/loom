/// Modules
pub mod convert;
pub mod crypto;
pub mod env;
pub mod fs;
pub mod is;
pub mod math;
pub mod mem;
pub mod process;
pub mod time;
pub mod strings;

/// Imports
use crate::{modules, refs::MutRef, rt::value::Module};
use std::collections::HashMap;

/// Provides modules
pub fn provide_modules() -> HashMap<String, MutRef<Module>> {
    modules! {
        convert,
        crypto,
        env,
        fs,
        is,
        math,
        mem,
        process,
        time,
        strings
    }
}
