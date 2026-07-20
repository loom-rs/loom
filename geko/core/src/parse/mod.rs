/// Modules
mod atom;
mod errors;
mod expr;
mod stmt;

/// Imports
use crate::{
    ast::Block,
    lex::{
        Lexer,
        token::{Token, TokenKind},
    },
    parse::errors::ParseError,
};
use common::{bail, span::Span};
use miette::NamedSource;
use std::sync::Arc;

/// Defines a parser, an entity that uses tokens iterator (lexer)
///  to build an AST (abstract syntax tree)
pub struct Parser<'s> {
    /// Named source of the file
    pub(crate) source: Arc<NamedSource<String>>,

    /// Lexer used to iterate over tokens
    pub(crate) lexer: Lexer<'s>,

    /// Previously consumed token
    /// (useful for spans and error reporting)
    pub(crate) previous: Option<Token>,

    /// Current token under inspection
    pub(crate) current: Option<Token>,

    /// Lookahead token
    /// (used for predictive parsing)
    pub(crate) next: Option<Token>,
}

/// Implementation
impl<'s> Parser<'s> {
    /// Creates new parser
    pub fn new(source: Arc<NamedSource<String>>, mut lexer: Lexer<'s>) -> Self {
        let current = lexer.next();
        let next = lexer.next();
        Self {
            source,
            lexer,
            previous: None,
            current,
            next,
        }
    }

    /// Parses program
    pub fn program(&mut self) -> Block {
        // If current is `None` => return empty block
        if self.current.is_none() {
            Block {
                span: Span(self.source.clone(), 0..0),
                stmts: Vec::new(),
            }
        }
        // If current is not `None` => parse program
        else {
            // Parsing statements
            let start_span = self.peek().span.clone();
            let mut stmts = Vec::new();
            while self.current.is_some() {
                stmts.push(self.stmt());
            }
            let end_span = self.prev().span.clone();

            Block {
                span: start_span + end_span,
                stmts,
            }
        }
    }

    /// Compares current token kind with passed one
    pub fn check(&self, tk: TokenKind) -> bool {
        match &self.current {
            Some(it) => it.kind == tk,
            None => false,
        }
    }

    /// Returns current token.
    /// Bails if current token is `None`.
    pub fn peek(&self) -> &Token {
        match &self.current {
            Some(tk) => tk,
            // Note: previous token is guaranteed `Some`
            None => bail!(ParseError::UnexpectedEof {
                src: self.source.clone(),
                span: self.previous.clone().unwrap().span.1.into(),
            }),
        }
    }

    /// Returns previous token.
    /// Bails if previous token is `None`.
    pub fn prev(&self) -> &Token {
        match &self.previous {
            Some(tk) => tk,
            // Note: previous token is guaranteed `Some`
            None => bail!(ParseError::UnexpectedEof {
                src: self.source.clone(),
                span: self.previous.clone().unwrap().span.1.into(),
            }),
        }
    }

    /// Compares current token kind with expected one
    /// Bails if current token is `None` and on token kinds missmatch.
    pub fn expect(&mut self, tk: TokenKind) -> Token {
        match &self.current {
            Some(it) => {
                if it.kind == tk {
                    self.bump()
                } else {
                    bail!(ParseError::UnexpectedToken {
                        got: it.kind.clone(),
                        expected: tk,
                        src: self.source.clone(),
                        span: it.span.1.clone().into(),
                        prev: self.prev().span.1.clone().into(),
                    })
                }
            }
            // Note: previous token is guaranteed `Some`
            None => bail!(ParseError::UnexpectedEof {
                src: self.source.clone(),
                span: self.previous.clone().unwrap().span.1.into(),
            }),
        }
    }

    /// Bumps current token. Does following:
    ///
    /// 1. Take `current` token and move to `previous`
    /// 2. Take `next` token and move to `current`
    /// 3. Get a new token for `next` from lexer
    ///
    pub fn bump(&mut self) -> Token {
        self.previous = self.current.take();
        self.current = self.next.take();
        self.next = self.lexer.next();
        self.previous.clone().unwrap()
    }
}
