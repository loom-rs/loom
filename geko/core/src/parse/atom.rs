/// Imports
use crate::{ast::Expr, lex::token::TokenKind, parse::Parser};

/// Atoms parsing
impl<'s> Parser<'s> {
    /// Parses a series of items using `parse_item`
    /// separated by `sep`
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

    /// Parses arguments enclosed in parens
    /// separated by comma
    pub(crate) fn args(&mut self) -> Vec<Expr> {
        self.sep_by(
            TokenKind::Lparen,
            TokenKind::Rparen,
            TokenKind::Comma,
            |s| s.expr(),
        )
    }

    /// Parses parameters enclosed in parens
    /// separated by comma
    pub(crate) fn params(&mut self) -> Vec<String> {
        self.sep_by(
            TokenKind::Lparen,
            TokenKind::Rparen,
            TokenKind::Comma,
            |s| s.expect(TokenKind::Id).lexeme,
        )
    }
}
