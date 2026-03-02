/// Imports
use crate::{
    builtins::{env, is, math},
    refs::MutRef,
    rt::value::Module,
};
use std::{cell::RefCell, collections::HashMap};

/// Provides modules
pub fn provide_modules() -> HashMap<String, MutRef<Module>> {
    HashMap::from([
        (
            "math".to_string(),
            MutRef::new(RefCell::new(Module {
                env: math::provide_env(),
            })),
        ),
        (
            "is".to_string(),
            MutRef::new(RefCell::new(Module {
                env: is::provide_env(),
            })),
        ),
        (
            "env".to_string(),
            MutRef::new(RefCell::new(Module {
                env: env::provide_env(),
            })),
        ),
    ])
}
