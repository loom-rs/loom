/// Modules
#[allow(unused_assignments)]
mod errors;
mod scope;

/// Imports
use crate::{errors::SemaError, scope::ScopeKind};
use tick_ast::{
    atom::Function,
    stmt::{Block, Statement},
};
use tick_common::bail;

/// Semantic analyzer
#[derive(Default)]
pub struct Analyzer {
    /// Scope stack
    stack: Vec<ScopeKind>,
}

/// Implementation
impl Analyzer {
    /// Analyzes module
    pub fn analyze_module(&mut self, block: &Block) {
        self.stack.push(ScopeKind::Block);
        self.analyze_block(block);
        self.stack.pop();
    }

    /// Analyzes function
    fn analyze_function(&mut self, function: &Function) {
        self.stack.push(ScopeKind::Function);
        self.analyze_block(&function.block);
        self.stack.pop();
    }

    /// Analyzes statement
    fn analyze_stmt(&mut self, stmt: &Statement) {
        match stmt {
            // Analyzing while block
            Statement::While { block, .. } => {
                self.stack.push(ScopeKind::Loop);
                self.analyze_block(block);
                self.stack.pop();
            }
            // Analyzing if
            Statement::If { then, else_, .. } => {
                // Analyzing branch
                self.stack.push(ScopeKind::Block);
                self.analyze_block(then);
                self.stack.pop();
                // Analyzing else branch, if presented
                if let Some(branch) = else_ {
                    self.analyze_stmt(branch);
                }
            }
            // Analyzing for block
            Statement::For { block, .. } => {
                self.stack.push(ScopeKind::Loop);
                self.analyze_block(block);
                self.stack.pop();
            }
            // Analyzing type methods
            Statement::Type { methods, .. } => {
                for method in methods {
                    self.analyze_function(&method);
                }
            }
            // Analyzing function
            Statement::Function(function) => {
                self.analyze_function(&function);
            }
            // Analyzing block
            Statement::Block(block) => {
                self.analyze_block(block);
            }
            Statement::Return { span, .. } => {
                // Checking hierarchy of scopes for function
                if !self.hierarchy_has_fn() {
                    bail!(SemaError::ReturnOutsideFn {
                        src: span.0.clone(),
                        span: span.1.clone().into()
                    })
                }
            }
            Statement::Continue(span) => {
                // Checking hierarchy of scopes for loop
                if !self.hierarchy_has_loop() {
                    bail!(SemaError::ContinueOutsideLoop {
                        src: span.0.clone(),
                        span: span.1.clone().into()
                    })
                }
            }
            Statement::Break(span) => {
                // Checking hierarchy of scopes for loop
                if !self.hierarchy_has_loop() {
                    bail!(SemaError::BreakOutsideLoop {
                        src: span.0.clone(),
                        span: span.1.clone().into()
                    })
                }
            }
            _ => {}
        }
    }

    /// Analyzes block
    fn analyze_block(&mut self, block: &Block) {
        for stmt in &block.statements {
            self.analyze_stmt(stmt);
        }
    }

    /// Checks if scopes stack has loop in hierarchy
    fn hierarchy_has_loop(&self) -> bool {
        for node in self.stack.iter().rev() {
            match node {
                ScopeKind::Loop => return true,
                ScopeKind::Function => break,
                _ => {}
            }
        }
        false
    }

    /// Checks if scopes stack has fn in hierarchy
    fn hierarchy_has_fn(&self) -> bool {
        for node in self.stack.iter().rev() {
            if let ScopeKind::Function = node {
                return true;
            }
        }
        false
    }
}
