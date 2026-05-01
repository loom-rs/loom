/// Imports
use crate::{Parser, errors::ParseError};
use geko_common::bail;
use geko_ir::{
    atom::{AssignOp, Class, Enum, Function, Trait, TraitFunction},
    expr::Expression,
    stmt::{Block, Statement, UseKind},
};
use geko_lex::token::TokenKind;

/// Statements parsing
impl<'s> Parser<'s> {
    /// Function parsing
    fn function(&mut self) -> Function {
        // Parsing function name
        let start_span = self.peek().span.clone();
        self.expect(TokenKind::Fun);
        let name = self.expect(TokenKind::Id).lexeme;

        // Parsing params
        let params = self.params();

        // Signature span
        let sign_span = start_span.clone() + self.prev().span.clone();

        // Parsing body
        let block = self.block();
        let end_span = self.prev().span.clone();

        // Done
        Function {
            name,
            span: start_span + end_span,
            sign_span,
            params,
            block,
        }
    }

    /// For statement parsing
    fn for_stmt(&mut self) -> Statement {
        let start_span = self.peek().span.clone();

        self.expect(TokenKind::For);
        let var = self.expect(TokenKind::Id).lexeme;
        self.expect(TokenKind::In);
        let iterable = self.expr();
        let block = self.block();

        let end_span = self.prev().span.clone();

        Statement::For {
            span: start_span + end_span,
            var,
            iterable,
            block,
        }
    }

    /// While statement parsing
    fn while_stmt(&mut self) -> Statement {
        let start_span = self.peek().span.clone();

        self.expect(TokenKind::While);
        let condition = self.expr();
        let block = self.block();

        let end_span = self.prev().span.clone();

        Statement::While {
            span: start_span + end_span,
            condition,
            block,
        }
    }

    /// Else branch
    fn else_branch(&mut self) -> Statement {
        self.expect(TokenKind::Else);
        if self.check(TokenKind::If) {
            self.if_stmt()
        } else {
            Statement::Block(Box::new(self.block()))
        }
    }

    /// If statement parsing
    fn if_stmt(&mut self) -> Statement {
        let start_span = self.peek().span.clone();

        // Parsing if clause
        self.expect(TokenKind::If);
        let condition = self.expr();
        let then = self.block();

        // Parsing else clause
        let else_ = if self.check(TokenKind::Else) {
            Some(Box::new(self.else_branch()))
        } else {
            None
        };

        let end_span = self.prev().span.clone();

        Statement::If {
            span: start_span + end_span,
            condition,
            then,
            else_,
        }
    }

    /// Class declaration parsing
    fn class_stmt(&mut self) -> Statement {
        let start_span = self.peek().span.clone();

        // Parsing class name
        self.expect(TokenKind::Class);
        let name = self.expect(TokenKind::Id);
        let name_span = start_span.clone() + name.span;
        self.expect(TokenKind::Lbrace);

        // Parsing methods
        let mut methods = Vec::new();
        while !self.check(TokenKind::Rbrace) {
            methods.push(self.function())
        }
        self.expect(TokenKind::Rbrace);

        let end_span = self.prev().span.clone();

        Statement::Class(Class {
            span: start_span + end_span,
            name_span,
            name: name.lexeme,
            methods,
        })
    }

    /// Enum declaration parsing
    fn enum_stmt(&mut self) -> Statement {
        let start_span = self.peek().span.clone();

        // Parsing enum name
        self.expect(TokenKind::Enum);
        let name = self.expect(TokenKind::Id);
        let name_span = start_span.clone() + name.span;

        // Parsing variants
        let variants = self.sep_by(
            TokenKind::Lbrace,
            TokenKind::Rbrace,
            TokenKind::Comma,
            |p| p.expect(TokenKind::Id).lexeme,
        );

        let end_span = self.prev().span.clone();

        Statement::Enum(Enum {
            span: start_span + end_span,
            name_span,
            name: name.lexeme,
            variants,
        })
    }

    /// Trait function parsing
    fn trait_function(&mut self) -> TraitFunction {
        let start_span = self.peek().span.clone();

        // Parsing trait signature
        self.expect(TokenKind::Fun);
        let name = self.expect(TokenKind::Id).lexeme;
        let params = self.params();

        let end_span = self.prev().span.clone();

        TraitFunction {
            span: start_span + end_span,
            name,
            params,
        }
    }

    /// Trait declaration parsing
    fn trait_stmt(&mut self) -> Statement {
        let start_span = self.peek().span.clone();

        // Parsing trait name
        self.expect(TokenKind::Trait);
        let name = self.expect(TokenKind::Id).lexeme;

        // Parsing functions
        let functions = self.sep_by(
            TokenKind::Lbrace,
            TokenKind::Rbrace,
            TokenKind::Comma,
            |p| p.trait_function(),
        );

        let end_span = self.prev().span.clone();

        Statement::Trait(Trait {
            span: start_span + end_span,
            name,
            functions,
        })
    }

    /// Assignment statement
    fn assign_stmt(&mut self) -> Statement {
        // Parsing lhs
        let start_span = self.peek().span.clone();
        let variable = self.variable_expr();

        // Checking for ssignment operator
        let op = match self.current.clone().map(|it| it.kind) {
            Some(TokenKind::PlusEq) => Some(AssignOp::Add),
            Some(TokenKind::MinusEq) => Some(AssignOp::Sub),
            Some(TokenKind::StarEq) => Some(AssignOp::Mul),
            Some(TokenKind::SlashEq) => Some(AssignOp::Div),
            Some(TokenKind::PercentEq) => Some(AssignOp::Mod),
            Some(TokenKind::AmpersandEq) => Some(AssignOp::BitAnd),
            Some(TokenKind::BarEq) => Some(AssignOp::BitOr),
            Some(TokenKind::CaretEq) => Some(AssignOp::Xor),
            Some(TokenKind::Eq) => Some(AssignOp::Assign),
            Some(TokenKind::Walrus) => Some(AssignOp::Define),
            Some(_) => None,
            _ => return Statement::Expr(variable),
        };

        // Checking assignment operator existence
        match op {
            // If operator found
            Some(op) => {
                // Bumping operator
                self.bump();
                let value = self.expr();
                let end_span = self.prev().span.clone();

                // Matching lhs
                match variable {
                    Expression::Variable { name, .. } => Statement::Assign {
                        span: start_span + end_span,
                        name,
                        op,
                        value,
                    },
                    Expression::Field {
                        name, container, ..
                    } => Statement::Set {
                        span: start_span + end_span,
                        container: *container,
                        name,
                        op,
                        value,
                    },
                    _ => bail!(ParseError::InvalidUseOfAssignOp {
                        src: self.source.clone(),
                        first_span: (start_span + end_span).1.into()
                    }),
                }
            }
            // Else
            None => Statement::Expr(variable),
        }
    }

    /// Break statement
    fn break_stmt(&mut self) -> Statement {
        let span = self.expect(TokenKind::Break).span;
        Statement::Break(span)
    }

    /// Continue statement
    fn continue_stmt(&mut self) -> Statement {
        let span = self.expect(TokenKind::Continue).span;
        Statement::Continue(span)
    }

    /// Return statement
    fn return_stmt(&mut self) -> Statement {
        let start_span = self.peek().span.clone();
        self.expect(TokenKind::Return);

        if self.check(TokenKind::Rbrace) {
            Statement::Return {
                span: start_span,
                expr: None,
            }
        } else {
            let value = self.expr();
            let end_span = self.prev().span.clone();
            Statement::Return {
                span: start_span + end_span,
                expr: Some(value),
            }
        }
    }

    /// Use path
    fn use_path(&mut self) -> String {
        let mut path = String::new();
        path.push_str(&self.expect(TokenKind::Id).lexeme);

        while self.check(TokenKind::Slash) {
            self.bump();
            path.push_str(&self.expect(TokenKind::Id).lexeme);
        }

        path
    }

    /// Use kind
    fn use_kind(&mut self) -> UseKind {
        if self.check(TokenKind::As) {
            self.bump();
            UseKind::As(self.expect(TokenKind::Id).lexeme)
        } else if self.check(TokenKind::For) {
            self.bump();
            if self.check(TokenKind::Star) {
                self.bump();
                UseKind::All
            } else {
                let mut items = Vec::new();
                items.push(self.expect(TokenKind::Id).lexeme);
                while self.check(TokenKind::Comma) {
                    self.bump();
                    items.push(self.expect(TokenKind::Id).lexeme);
                }
                UseKind::For(items)
            }
        } else {
            UseKind::Just
        }
    }

    /// Use statement
    fn use_stmt(&mut self) -> Statement {
        let start_span = self.peek().span.clone();
        self.expect(TokenKind::Use);
        let path = self.use_path();
        let kind = self.use_kind();
        let end_span = self.prev().span.clone();

        Statement::Use {
            span: start_span + end_span,
            path,
            kind,
        }
    }

    /// Satement parsing
    pub fn stmt(&mut self) -> Statement {
        match self.peek().kind {
            TokenKind::For => self.for_stmt(),
            TokenKind::While => self.while_stmt(),
            TokenKind::If => self.if_stmt(),
            TokenKind::Class => self.class_stmt(),
            TokenKind::Enum => self.enum_stmt(),
            TokenKind::Trait => self.trait_stmt(),
            TokenKind::Fun => Statement::Function(self.function()),
            TokenKind::Return => self.return_stmt(),
            TokenKind::Continue => self.continue_stmt(),
            TokenKind::Break => self.break_stmt(),
            TokenKind::Id => self.assign_stmt(),
            TokenKind::Use => self.use_stmt(),
            _ => Statement::Expr(self.expr()),
        }
    }

    /// Block parsing
    pub fn block(&mut self) -> Block {
        // Preparing vector for statements
        let mut statements = Vec::new();

        // Parsing statements
        let start_span = self.peek().span.clone();
        self.expect(TokenKind::Lbrace);
        while !self.check(TokenKind::Rbrace) {
            statements.push(self.stmt());
        }
        self.expect(TokenKind::Rbrace);
        let end_span = self.prev().span.clone();

        Block {
            span: start_span + end_span,
            statements,
        }
    }
}
