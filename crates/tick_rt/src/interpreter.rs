/// Imports
use crate::{
    builtins::{self, Builtins},
    modules::Modules,
    refs::{EnvRef, MutRef},
    rt::{env::Environment, value::Module},
};
use miette::NamedSource;
use std::{cell::RefCell, sync::Arc};
use tick_ast::stmt::Block;
use tick_common::io::IO;
use tick_lex::lexer::Lexer;
use tick_parse::parser::Parser;
use tick_sema::Analyzer;

/// Interpreter
pub struct Interpreter<'io> {
    /// Builtins environment
    pub(crate) builtins: Builtins,
    /// Current environment
    pub(crate) env: EnvRef,
    /// Modules registry
    pub(crate) modules: Modules,
    /// IO
    pub(crate) io: &'io dyn IO,
}

/// Implementation
impl<'io> Interpreter<'io> {
    /// Creates new interpreter
    pub fn new(io: &'io dyn IO) -> Self {
        Interpreter {
            builtins: builtins::provide_builtins(),
            env: EnvRef::new(RefCell::new(Environment::default())),
            modules: Modules::default(),
            io,
        }
    }

    /// Parses module
    pub(crate) fn parse_module(&mut self, name: &str, content: &str) -> Block {
        // Creating named source
        let src = Arc::new(NamedSource::new(name, content.to_string()));

        // Creating lexer and parser
        let lexer = Lexer::new(src.clone(), &content);
        let mut parser = Parser::new(src, lexer);

        // Parsing module text into AST
        let ast = parser.parse();

        // Performing semantic analysis
        let mut analyzer = Analyzer::default();
        analyzer.analyze_module(&ast);

        ast
    }

    /// Executes module into given environment
    fn exec_module_into(&mut self, name: &str, content: &str, env: EnvRef) {
        // Loading module
        let block = self.parse_module(name, content);

        // Pushing scope
        let previous = self.env.clone();
        self.env = env;

        // Executing statements
        for stmt in &block.statements {
            let _ = self.exec(stmt);
        }

        // Popping scope
        self.env = previous;
    }

    /// Loads and executes module, if module with same name is not already executed.
    pub fn interpret_module(&mut self, name: &str, content: &str) -> MutRef<Module> {
        // Checking module is already loaded
        match self.modules.get(&name) {
            // If already loaded, returning it
            Some(module) => module,
            // If not, executing it and saving to modules registry
            None => {
                // Creating environment and module
                let env = EnvRef::new(RefCell::new(Environment::default()));
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
