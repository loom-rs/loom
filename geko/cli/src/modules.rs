/// Imports
use crate::io::CliIO;
use camino::Utf8PathBuf;
use common::{bail, io::IO, span::Span};
use dune::{
    ModuleRegistry,
    ops::Chunk,
    refs::{MutRef, Ref},
    value::Module,
};
use geko_core::{Flags, emit};
use miette::{Diagnostic, NamedSource, SourceSpan};
use std::{collections::HashMap, sync::Arc};
use thiserror::Error;

/// Modules registry error
#[derive(Error, Diagnostic, Debug)]
pub enum ModsError {
    /// Failed to find module
    #[error("failed to find `{path}` module")]
    #[diagnostic(code(mods::failed_to_find_module))]
    FailedToFindModule {
        path: String,
        #[source_code]
        src: Arc<NamedSource<String>>,
        #[label("variable access here...")]
        span: SourceSpan,
    },
}

/// Defines CLI module registry
pub struct CliModuleRegistry<'io> {
    /// IO for fs manipulation
    io: &'io CliIO,

    /// Loaded modules mapping:
    /// Id -> Module
    modules: HashMap<String, MutRef<Module>>,
}

/// Cli module registry implementation
impl<'io> CliModuleRegistry<'io> {
    /// Creates new module registry
    pub fn new(io: &'io CliIO) -> Self {
        Self {
            io,
            modules: HashMap::new(),
        }
    }
}

/// Module registry trait implementation
impl<'io> ModuleRegistry for CliModuleRegistry<'io> {
    /// Resolves module by id
    fn resolve(&self, span: Span, path: &str) -> (String, Ref<Chunk>) {
        // Resolving module path
        let path_to_module = if path.starts_with("@/") {
            Some(
                Utf8PathBuf::from(span.0.name())
                    .parent()
                    .unwrap()
                    .join(&path[2..])
                    .with_extension("gk"),
            )
        } else {
            match self.io.cwd() {
                Some(cwd) => Some(cwd.join(path).with_extension("gk")),
                None => None,
            }
        };

        // Resolving module by path
        match path_to_module {
            Some(path) if path.exists() => {
                // Preparing module id
                let id = &self.io.canonicalize(&path).unwrap().to_string();

                // Reading module
                let code = self.io.read(&path);
                let source = Arc::new(NamedSource::new(id, code.clone()));

                // Compiling module
                let chunk = emit(source, &code, Flags::default());
                (id.into(), chunk)
            }
            _ => bail!(ModsError::FailedToFindModule {
                path: path.to_string(),
                src: span.0.clone(),
                span: span.1.clone().into()
            }),
        }
    }

    /// Inserts module by id
    fn insert(&mut self, id: &str, module: MutRef<Module>) {
        self.modules.insert(id.into(), module);
    }
}
