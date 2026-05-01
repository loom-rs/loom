/// Modules
#[allow(unused_assignments)]
pub mod errors;
pub mod expr;
pub mod stmt;

/// Import
use crate::errors::ParseError;
use geko_common::bail;
use geko_ir::{expr::Expression, stmt::Block};
use geko_lex::{
    lexer::Lexer,
    token::{Span, Token, TokenKind},
};
use miette::NamedSource;
use std::sync::Arc;

/// Parser converts a stream of tokens
/// produced by the lexer into an abstract syntax tree (AST).
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

    /// Parses module
    pub fn parse(&mut self) -> Block {
        // If end of file
        if self.current.is_none() {
            Block {
                span: Span(self.source.clone(), 0..0),
                statements: Vec::new(),
            }
        }
        // Else
        else {
            // Parsing statements
            let start_span = self.peek().span.clone();
            let mut statements = Vec::new();
            while self.current.is_some() {
                statements.push(self.stmt());
            }
            let end_span = self.prev().span.clone();

            Block {
                span: start_span + end_span,
                statements,
            }
        }
    }

    /// Sep by parsing
    pub(crate) fn sep_by<T>(
        &mut self,
        open: TokenKind,
        close: TokenKind,
        sep: TokenKind,
        mut parse_item: impl FnMut(&mut Self) -> T,
    ) -> Vec<T> {
        let mut items = Vec::new();
        self.expect(open);

        if !self.check(close.clone()) {
            loop {
                items.push(parse_item(self));
                if self.check(sep.clone()) {
                    self.expect(sep.clone());
                    if self.check(close.clone()) {
                        break;
                    }
                } else {
                    break;
                }
            }
        }

        self.expect(close);
        items
    }

    /// Arguments parsing
    pub(crate) fn args(&mut self) -> Vec<Expression> {
        self.sep_by(
            TokenKind::Lparen,
            TokenKind::Rparen,
            TokenKind::Comma,
            |s| s.expr(),
        )
    }

    /// Parameters parsing
    pub(crate) fn params(&mut self) -> Vec<String> {
        self.sep_by(
            TokenKind::Lparen,
            TokenKind::Rparen,
            TokenKind::Comma,
            |s| s.expect(TokenKind::Id).lexeme,
        )
    }

    /// Checks token match
    pub fn check(&self, tk: TokenKind) -> bool {
        match &self.current {
            Some(it) => it.kind == tk,
            None => false,
        }
    }

    /// Retrieves current token
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

    /// Retrieves previous token
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

    /// Expects token with kind
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

    /// Advances current token
    pub fn bump(&mut self) -> Token {
        self.previous = self.current.take();
        self.current = self.next.take();
        self.next = self.lexer.next();
        self.previous.clone().unwrap()
    }
}
