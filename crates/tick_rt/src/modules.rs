/// Imports
use crate::{refs::MutRef, rt::value::Module};
use std::collections::HashMap;

/// Modules registry
#[derive(Default)]
pub struct Modules {
    /// Imported / loaded modules mapping
    /// `name` -> `module`
    modules: HashMap<String, MutRef<Module>>,
}

/// Implementation
impl Modules {
    /// Gets module by name
    pub fn get(&self, name: &str) -> Option<MutRef<Module>> {
        self.modules.get(name).cloned()
    }

    /// Sets module by name
    pub fn set(&mut self, name: String, module: MutRef<Module>) {
        self.modules.insert(name, module);
    }
}
