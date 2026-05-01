/// Imports
use crate::{
    builtins::{self, Builtins},
    modules::Modules,
    refs::{MutRef, RealmRef},
    rt::{realm::Realm, value::Module},
};
use geko_common::io::IO;
use geko_ir::stmt::Block;
use geko_lex::lexer::Lexer;
use geko_parse::Parser;
use geko_sema::Analyzer;
use miette::NamedSource;
use std::{cell::RefCell, sync::Arc};

/// The interpreting orchestra
pub struct Interpreter<'io> {
    /// Builtins realm
    pub(crate) builtins: Builtins,
    /// Current realm
    pub(crate) realm: RealmRef,
    /// Modules registry
    pub(crate) modules: Modules,
    /// IO
    pub(crate) io: &'io dyn IO,
}

/// Base interpreter implementation
impl<'io> Interpreter<'io> {
    /// Creates new interpreter
    pub fn new(io: &'io dyn IO) -> Self {
        Interpreter {
            builtins: builtins::provide_builtins(),
            realm: RealmRef::new(RefCell::new(Realm::default())),
            modules: Modules::default(),
            io,
        }
    }

    /// Parses module
    pub(crate) fn parse_module(&mut self, name: &str, content: &str) -> Block {
        // Creating named source
        let src = Arc::new(NamedSource::new(name, content.to_string()));

        // Creating lexer and parser
        let lexer = Lexer::new(src.clone(), content);
        let mut parser = Parser::new(src, lexer);

        // Parsing module text into AST
        let ast = parser.parse();

        // Performing semantic analysis
        let mut analyzer = Analyzer::default();
        analyzer.analyze_module(&ast);

        ast
    }

    /// Executes module into given realm
    fn exec_module_into(&mut self, name: &str, content: &str, env: RealmRef) {
        // Loading module
        let block = self.parse_module(name, content);

        // Pushing scope
        let previous = self.realm.clone();
        self.realm = env;

        // Executing statements
        for stmt in &block.statements {
            let _ = self.exec(stmt);
        }

        // Popping scope
        self.realm = previous;
    }

    /// Loads and executes module, if module with same name is not already executed.
    pub fn interpret_module(&mut self, name: &str, content: &str) -> MutRef<Module> {
        // Checking module is already loaded
        match self.modules.get(name) {
            // If already loaded, returning it
            Some(module) => module,
            // If not, executing it and saving to modules registry
            None => {
                // Creating realm and module
                let env = RealmRef::new(RefCell::new(Realm::default()));
                let module = MutRef::new(RefCell::new(Module { env: env.clone() }));
                // Registering module before executing it
                self.modules.set(name.to_string(), module.clone());
                // Executing module
                self.exec_module_into(name, content, env);
                // Done
                module
            }
        }
    }

    /// Loads builtin module
    pub fn load_builtin_module(&mut self, name: &str) -> Option<MutRef<Module>> {
        self.builtins.modules.get(name).cloned()
    }
}
