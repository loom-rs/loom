/// Imports
use crate::{
    lex::token::TokenKind,
    parse::Parser,
    parse::ast::{Block, Class, Enum, Function, Stmt, Trait, TraitFunction, UseKind},
};

/// Stmts parsing
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

    /// For Stmt parsing
    fn for_stmt(&mut self) -> Stmt {
        let start_span = self.bump().span.clone();

        let var = self.expect(TokenKind::Id).lexeme;
        self.expect(TokenKind::In);
        let iterable = self.expr();
        let block = self.block();

        let end_span = self.prev().span.clone();

        Stmt::For {
            span: start_span + end_span,
            var,
            iterable,
            block,
        }
    }

    /// While statement parsing
    fn while_stmt(&mut self) -> Stmt {
        let start_span = self.bump().span.clone();

        let condition = self.expr();
        let block = self.block();

        let end_span = self.prev().span.clone();

        Stmt::While {
            span: start_span + end_span,
            condition,
            block,
        }
    }

    /// Until statement parsing
    fn until_stmt(&mut self) -> Stmt {
        let start_span = self.bump().span.clone();

        let condition = self.expr();
        let block = self.block();

        let end_span = self.prev().span.clone();

        Stmt::Until {
            span: start_span + end_span,
            condition,
            block,
        }
    }

    /// Else branch
    fn else_branch(&mut self) -> Stmt {
        self.expect(TokenKind::Else);
        if self.check(TokenKind::If) {
            self.if_stmt()
        } else {
            Stmt::Block(self.block())
        }
    }

    /// If Stmt parsing
    fn if_stmt(&mut self) -> Stmt {
        // Parsing if clause
        let start_span = self.bump().span.clone();
        let condition = self.expr();
        let then = self.block();

        // Parsing else clause
        let else_ = if self.check(TokenKind::Else) {
            Some(Box::new(self.else_branch()))
        } else {
            None
        };

        let end_span = self.prev().span.clone();

        Stmt::If {
            span: start_span + end_span,
            condition,
            then,
            else_,
        }
    }

    /// Class declaration parsing
    fn class_stmt(&mut self) -> Stmt {
        // Parsing class name
        let start_span = self.bump().span.clone();
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

        Stmt::Class(Class {
            span: start_span + end_span,
            name_span,
            name: name.lexeme,
            methods,
        })
    }

    /// Enum declaration parsing
    fn enum_stmt(&mut self) -> Stmt {
        // Parsing enum name
        let start_span = self.bump().span.clone();
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

        Stmt::Enum(Enum {
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
    fn trait_stmt(&mut self) -> Stmt {
        // Parsing trait name
        let start_span = self.bump().span.clone();
        let name = self.expect(TokenKind::Id).lexeme;

        // Parsing functions
        let functions = self.sep_by(
            TokenKind::Lbrace,
            TokenKind::Rbrace,
            TokenKind::Comma,
            |p| p.trait_function(),
        );

        let end_span = self.prev().span.clone();

        Stmt::Trait(Trait {
            span: start_span + end_span,
            name,
            functions,
        })
    }

    /// Break Stmt
    fn break_stmt(&mut self) -> Stmt {
        let span = self.bump().span;
        Stmt::Break(span)
    }

    /// Continue Stmt
    fn continue_stmt(&mut self) -> Stmt {
        let span = self.bump().span;
        Stmt::Continue(span)
    }

    /// Return Stmt
    fn return_stmt(&mut self) -> Stmt {
        let start_span = self.bump().span.clone();
        let value = self.expr();
        let end_span = self.prev().span.clone();

        Stmt::Return {
            span: start_span + end_span,
            value,
        }
    }

    /// Use kind
    fn use_kind(&mut self) -> UseKind {
        if self.check(TokenKind::As) {
            self.bump();
            UseKind::As(self.expect(TokenKind::Id).lexeme)
        } else if self.check(TokenKind::Pick) {
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
                UseKind::Pick(items)
            }
        } else {
            UseKind::Just
        }
    }

    /// Use Stmt
    fn use_stmt(&mut self) -> Stmt {
        let start_span = self.bump().span.clone();
        let path = self.expect(TokenKind::String).lexeme;
        let kind = self.use_kind();
        let end_span = self.prev().span.clone();

        Stmt::Use {
            span: start_span + end_span,
            path,
            kind,
        }
    }

    /// Satement parsing
    pub fn stmt(&mut self) -> Stmt {
        match self.peek().kind {
            TokenKind::For => self.for_stmt(),
            TokenKind::While => self.while_stmt(),
            TokenKind::Until => self.until_stmt(),
            TokenKind::If => self.if_stmt(),
            TokenKind::Class => self.class_stmt(),
            TokenKind::Enum => self.enum_stmt(),
            TokenKind::Trait => self.trait_stmt(),
            TokenKind::Fun => Stmt::Function(self.function()),
            TokenKind::Return => self.return_stmt(),
            TokenKind::Continue => self.continue_stmt(),
            TokenKind::Break => self.break_stmt(),
            TokenKind::Use => self.use_stmt(),
            _ => Stmt::Expr(self.expr()),
        }
    }

    /// Block parsing
    pub fn block(&mut self) -> Block {
        // Preparing vector for Stmts
        let mut stmts = Vec::new();

        // Parsing Stmts
        let start_span = self.peek().span.clone();
        self.expect(TokenKind::Lbrace);
        while !self.check(TokenKind::Rbrace) {
            stmts.push(self.stmt());
        }
        self.expect(TokenKind::Rbrace);
        let end_span = self.prev().span.clone();

        Block {
            span: start_span + end_span,
            stmts,
        }
    }
}
