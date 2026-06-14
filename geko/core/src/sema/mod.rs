/// Modules
mod errors;

/// Imports
use crate::{
    ast::{Block, Expr, Function, Stmt},
    sema::errors::SemaError,
};
use common::bail;

/// Defines scope kind for semantic analysis
pub enum ScopeKind {
    Block,
    Function,
    Loop,
}

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
    fn analyze_stmt(&mut self, stmt: &Stmt) {
        // Matching statement
        match stmt {
            // Analyzing loop blocks
            Stmt::While {
                condition, block, ..
            } => {
                self.stack.push(ScopeKind::Loop);
                self.analyze_expr(condition);
                self.analyze_block(block);
                self.stack.pop();
            }
            Stmt::For {
                iterable, block, ..
            } => {
                self.stack.push(ScopeKind::Loop);
                self.analyze_expr(iterable);
                self.analyze_block(block);
                self.stack.pop();
            }
            // Analyzing then and else block
            Stmt::If {
                condition,
                then,
                else_,
                ..
            } => {
                // Analyzing condition and then block
                self.stack.push(ScopeKind::Block);
                self.analyze_expr(condition);
                self.analyze_block(then);
                self.stack.pop();

                // Analyzing else stmt, if presented
                if let Some(branch) = else_ {
                    self.analyze_stmt(branch);
                }
            }
            // Analyzing class methods
            Stmt::Class(class) => {
                for method in &class.methods {
                    self.analyze_function(method);
                }
            }
            // Analyzing function
            Stmt::Function(function) => {
                self.analyze_function(function);
            }
            // Analyzing scope block statements
            Stmt::Block(block) => {
                self.analyze_block(block);
            }
            // Analyzing terminator statements
            Stmt::Return { span, value } => {
                // Analyzing return value
                self.analyze_expr(value);

                // Checking hierarchy of scopes for function
                if !self.hierarchy_has_fn() {
                    bail!(SemaError::ReturnOutsideFn {
                        src: span.0.clone(),
                        span: span.1.clone().into()
                    })
                }
            }
            // Analyzing terminators
            Stmt::Continue(span) => {
                // Checking hierarchy of scopes for loop
                if !self.hierarchy_has_loop() {
                    bail!(SemaError::ContinueOutsideLoop {
                        src: span.0.clone(),
                        span: span.1.clone().into()
                    })
                }
            }
            Stmt::Break(span) => {
                // Checking hierarchy of scopes for loop
                if !self.hierarchy_has_loop() {
                    bail!(SemaError::BreakOutsideLoop {
                        src: span.0.clone(),
                        span: span.1.clone().into()
                    })
                }
            }
            // Analyzing assignment statement
            Stmt::Assign {
                span, what, value, ..
            } => {
                // Matching lhs
                if !matches!(what, Expr::Variable { .. } | Expr::Field { .. }) {
                    bail!(SemaError::InvalidAssignLhs {
                        src: span.0.clone(),
                        span: span.1.clone().into()
                    })
                }

                // Analyzing expressions
                self.analyze_expr(what);
                self.analyze_expr(value);
            }
            // Analyzing expr statement
            Stmt::Expr(expr) => self.analyze_expr(expr),
            // Skipping use, enum, trait statements
            Stmt::Use { .. } | Stmt::Enum(_) | Stmt::Trait(_) => {}
        }
    }

    /// Analyzes expr
    fn analyze_expr(&mut self, expr: &Expr) {
        // Matching expression
        match expr {
            // Analyzing binary and unary expression
            Expr::Bin { lhs, rhs, .. } => {
                self.analyze_expr(lhs);
                self.analyze_expr(rhs);
            }
            Expr::Un { value, .. } => self.analyze_expr(value),
            // Analyzing field container
            Expr::Field { container, .. } => self.analyze_expr(container),
            // Analyzing arguments and callee
            Expr::Call {
                args, callee: what, ..
            } => {
                self.analyze_expr(what);
                args.iter().for_each(|arg| self.analyze_expr(arg));
            }
            // Analyzing collection initializers
            Expr::List { list, .. } => list.iter().for_each(|arg| self.analyze_expr(arg)),
            Expr::Dict { dict, .. } => dict.iter().for_each(|(k, v)| {
                self.analyze_expr(k);
                self.analyze_expr(v);
            }),
            // Analyzing anonymous function (lambda)
            Expr::Fun { block, .. } => {
                self.stack.push(ScopeKind::Function);
                self.analyze_block(block);
                self.stack.pop();
            }
            // Analyzing range expression
            Expr::Range { lhs, rhs, .. } => {
                self.analyze_expr(lhs);
                self.analyze_expr(rhs);
            }
            // Skipping literal and variable expressions
            Expr::Lit { .. } | Expr::Variable { .. } => {}
        }
    }

    /// Analyzes block
    fn analyze_block(&mut self, block: &Block) {
        for stmt in &block.stmts {
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
