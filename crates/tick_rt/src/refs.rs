/// Imports
use crate::rt::env::Environment;
use std::{cell::RefCell, rc::Rc};

/// Ref types
pub type MutRef<T> = Rc<RefCell<T>>;
pub type Ref<T> = Rc<T>;
pub type EnvRef = Rc<RefCell<Environment>>;
