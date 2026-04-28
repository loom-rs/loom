/// Modules
#[allow(unused_assignments)]
mod errors;
mod scope;

/// Imports
use crate::{errors::SemaError, scope::ScopeKind};
use geko_common::bail;
use geko_ir::{
    atom::Function,
    expr::Expression,
    stmt::{Block, Statement},
};

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
        // Matching statement
        match stmt {
            // Analyzing loop blocks
            Statement::While { condition, block, .. } => {
                self.stack.push(ScopeKind::Loop);
                self.analyze_expr(condition);
                self.analyze_block(block);
                self.stack.pop();
            }
            Statement::For { iterable, block, .. } => {
                self.stack.push(ScopeKind::Loop);
                self.analyze_expr(iterable);
                self.analyze_block(block);
                self.stack.pop();
            }
            // Analyzing then and else block
            Statement::If { condition, then, else_, .. } => {
                // Analyzing branch
                self.stack.push(ScopeKind::Block);
                self.analyze_expr(condition);
                self.analyze_block(then);
                self.stack.pop();
                // Analyzing else branch, if presented
                if let Some(branch) = else_ {
                    self.analyze_stmt(branch);
                }
            }
            // Analyzing class methods
            Statement::Class(class) => {
                for method in &class.methods {
                    self.analyze_function(&method);
                }
            }
            // Analyzing function
            Statement::Function(function) => {
                self.analyze_function(function);
            }
            // Analyzing block statements
            Statement::Block(block) => {
                self.analyze_block(block);
            }
            // Analyzing terminator statements
            Statement::Return { span, expr } => {
                // Analyzing return value
                if let Some(expr) = expr {
                    self.analyze_expr(expr)
                }
                // Checking hierarchy of scopes for function
                if !self.hierarchy_has_fn() {
                    bail!(SemaError::ReturnOutsideFn {
                        src: span.0.clone(),
                        span: span.1.clone().into()
                    })
                }
            }
            Statement::Continue(span)
                // Checking hierarchy of scopes for loop
                if !self.hierarchy_has_loop() => {
                    bail!(SemaError::ContinueOutsideLoop {
                        src: span.0.clone(),
                        span: span.1.clone().into()
                    })
                }
            Statement::Break(span)
                // Checking hierarchy of scopes for loop
                if !self.hierarchy_has_loop() => {
                    bail!(SemaError::BreakOutsideLoop {
                        src: span.0.clone(),
                        span: span.1.clone().into()
                    })
                }
            // Analyzing assignment statements
            Statement::Assign { value, .. } =>{
                self.analyze_expr(value);
            }
            Statement::Set { container, value, .. } => {
                self.analyze_expr(container);
                self.analyze_expr(value);
            },
            // Analyzing expr statement
            Statement::Expr(expr) => self.analyze_expr(expr),
            // Skipping use, enum, trait statements
            Statement::Use { .. } |
            Statement::Enum(_) |
            Statement::Trait(_) => {}
            // Skipping terminators, already checked loop condition before
            Statement::Continue(_) |
            Statement::Break(_) => {}
        }
    }

    /// Analyzes expr
    fn analyze_expr(&mut self, expr: &Expression) {
        // Matching expression
        match expr {
            // Analyzing binary and unary expression
            Expression::Bin { lhs, rhs, .. } => {
                self.analyze_expr(lhs);
                self.analyze_expr(rhs);
            }
            Expression::Unary { value, .. } => self.analyze_expr(value),
            // Analyzing field container
            Expression::Field { container, .. } => self.analyze_expr(container),
            // Analyzing arguments and callee
            Expression::Call { args, what, .. } => {
                self.analyze_expr(what);
                args.iter().for_each(|arg| self.analyze_expr(arg));
            }
            // Analyzing collections
            Expression::List { list, .. } => list.iter().for_each(|arg| self.analyze_expr(arg)),
            Expression::Dict { dict, .. } => dict.iter().for_each(|(k, v)| {
                self.analyze_expr(k);
                self.analyze_expr(v);
            }),
            // Analyzing anonymous function (lambda)
            Expression::Fun { block, .. } => {
                self.stack.push(ScopeKind::Function);
                self.analyze_block(block);
                self.stack.pop();
            }
            // Analyzing range expression
            Expression::Range { lhs, rhs, .. } => {
                self.analyze_expr(lhs);
                self.analyze_expr(rhs);
            }
            // Skipping literal and variable expressions
            Expression::Lit { .. } | Expression::Variable { .. } => {}
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
