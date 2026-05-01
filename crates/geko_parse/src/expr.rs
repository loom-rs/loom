/// Imports
use crate::{Parser, errors::ParseError};
use geko_common::bail;
use geko_ir::{
    atom::{BinOp, Lit, UnaryOp},
    expr::Expression,
    stmt::{Block, Statement},
};
use geko_lex::token::TokenKind;

/// Expressions parsing
impl<'s> Parser<'s> {
    /// Variable parsing
    pub fn variable_expr(&mut self) -> Expression {
        // Bumping base identifier
        let start_span = self.peek().span.clone();
        let id = self.bump().lexeme;

        // Result node
        let mut result = Expression::Variable {
            span: start_span.clone(),
            name: id,
        };

        // Checking for dots and parens
        loop {
            // Checking for field access
            if self.check(TokenKind::Dot) {
                self.bump();
                let id = self.expect(TokenKind::Id).lexeme;
                let end_span = self.prev().span.clone();
                result = Expression::Field {
                    span: start_span.clone() + end_span,
                    container: Box::new(result),
                    name: id,
                };
                continue;
            }
            // Checking for call
            if self.check(TokenKind::Lparen) {
                let args = self.args();
                let end_span = self.prev().span.clone();
                result = Expression::Call {
                    span: start_span.clone() + end_span,
                    what: Box::new(result),
                    args,
                };
                continue;
            }
            // Breaking loop
            break;
        }
        result
    }

    /// Group expression parsing
    fn group_expr(&mut self) -> Expression {
        self.expect(TokenKind::Lparen);
        let expr = self.expr();
        self.expect(TokenKind::Rparen);
        expr
    }

    /// List expression parsing
    fn list_expr(&mut self) -> Expression {
        let start_span = self.peek().span.clone();
        let list = self.sep_by(
            TokenKind::Lbracket,
            TokenKind::Rbracket,
            TokenKind::Comma,
            |p| p.expr(),
        );
        let end_span = self.prev().span.clone();

        Expression::List {
            span: start_span + end_span,
            list,
        }
    }

    /// Single dict pair parsing
    fn dict_pair(&mut self) -> (Expression, Expression) {
        let key = self.expr();
        self.expect(TokenKind::Colon);
        let value = self.expr();

        (key, value)
    }

    /// Dict expression parsing
    fn dict_expr(&mut self) -> Expression {
        let start_span = self.peek().span.clone();
        let dict = self.sep_by(
            TokenKind::Lbrace,
            TokenKind::Rbrace,
            TokenKind::Comma,
            |p| p.dict_pair(),
        );
        let end_span = self.prev().span.clone();

        Expression::Dict {
            span: start_span + end_span,
            dict,
        }
    }

    /// Anonymous function parsing
    fn anon_fun_expr(&mut self) -> Expression {
        let start_span = self.bump().span.clone();

        // Parsing function params
        let params = self.params();

        // Parsing function body
        let block = if self.check(TokenKind::Arrow) {
            self.bump();
            let start_span = self.peek().span.clone();
            let expr = self.expr();
            let end_span = self.prev().span.clone();

            Block {
                span: start_span.clone() + end_span.clone(),
                statements: vec![Statement::Return {
                    span: start_span + end_span,
                    expr: Some(expr),
                }],
            }
        } else {
            self.block()
        };

        let end_span = self.prev().span.clone();
        Expression::Fun {
            span: start_span + end_span,
            params,
            block,
        }
    }

    /// Atom expression parsing
    fn atom_expr(&mut self) -> Expression {
        let tk = self.peek().clone();
        match tk.kind {
            TokenKind::Lparen => self.group_expr(),
            TokenKind::Number => {
                let expr = Expression::Lit {
                    span: tk.span,
                    lit: Lit::Number(tk.lexeme),
                };
                self.bump();
                expr
            }
            TokenKind::String => {
                let expr = Expression::Lit {
                    span: tk.span,
                    lit: Lit::String(tk.lexeme),
                };
                self.bump();
                expr
            }
            TokenKind::Bool => {
                let expr = Expression::Lit {
                    span: tk.span,
                    lit: Lit::Bool(tk.lexeme),
                };
                self.bump();
                expr
            }
            TokenKind::Null => {
                let expr = Expression::Lit {
                    span: tk.span,
                    lit: Lit::Null,
                };
                self.bump();
                expr
            }
            TokenKind::Id => self.variable_expr(),
            TokenKind::Lbracket => self.list_expr(),
            TokenKind::Lbrace => self.dict_expr(),
            TokenKind::Fun => self.anon_fun_expr(),
            _ => bail!(ParseError::UnexpectedExprToken {
                got: tk.kind,
                src: self.source.clone(),
                span: tk.span.1.into(),
            }),
        }
    }

    /// Unary expression parsing
    fn unary_expr(&mut self) -> Expression {
        if self.check(TokenKind::Minus) || self.check(TokenKind::Bang) {
            let start_span = self.peek().span.clone();
            let op = self.bump();
            let value = self.unary_expr();
            let end_span = self.prev().span.clone();
            return Expression::Unary {
                span: start_span.clone() + end_span,
                op: match op.kind {
                    TokenKind::Minus => UnaryOp::Neg,
                    TokenKind::Bang => UnaryOp::Bang,
                    _ => unreachable!(),
                },
                value: Box::new(value),
            };
        }
        self.atom_expr()
    }

    /// Factor expression parsing
    fn factor_expr(&mut self) -> Expression {
        let start_span = self.peek().span.clone();
        let mut left = self.unary_expr();

        while self.check(TokenKind::Star)
            || self.check(TokenKind::Slash)
            || self.check(TokenKind::Percent)
        {
            let op = self.bump();
            let right = self.unary_expr();
            let end_span = self.prev().span.clone();
            left = Expression::Bin {
                span: start_span.clone() + end_span,
                op: match op.kind {
                    TokenKind::Star => BinOp::Mul,
                    TokenKind::Slash => BinOp::Div,
                    TokenKind::Percent => BinOp::Mod,
                    _ => unreachable!(),
                },
                lhs: Box::new(left),
                rhs: Box::new(right),
            }
        }

        left
    }

    /// Term expression parsing
    fn term_expr(&mut self) -> Expression {
        let start_span = self.peek().span.clone();
        let mut left = self.factor_expr();

        while self.check(TokenKind::Plus) || self.check(TokenKind::Minus) {
            let op = self.bump();
            let right = self.factor_expr();
            let end_span = self.prev().span.clone();
            left = Expression::Bin {
                span: start_span.clone() + end_span,
                op: match op.kind {
                    TokenKind::Plus => BinOp::Add,
                    TokenKind::Minus => BinOp::Sub,
                    _ => unreachable!(),
                },
                lhs: Box::new(left),
                rhs: Box::new(right),
            }
        }

        left
    }

    /// Range expression parsing
    fn range_expr(&mut self) -> Expression {
        let start_span = self.peek().span.clone();
        let mut left = self.term_expr();

        if self.check(TokenKind::DoubleDot) {
            let includes_end = {
                self.bump();
                if self.check(TokenKind::Eq) {
                    self.bump();
                    true
                } else {
                    false
                }
            };
            let right = self.term_expr();
            let end_span = self.prev().span.clone();
            left = Expression::Range {
                span: start_span.clone() + end_span,
                lhs: Box::new(left),
                rhs: Box::new(right),
                includes_end,
            }
        }

        left
    }

    /// Impls expression parsing
    fn impls_expr(&mut self) -> Expression {
        let start_span = self.peek().span.clone();
        let mut left = self.range_expr();

        while self.check(TokenKind::GtColon) | self.check(TokenKind::GtBang) {
            let op = self.bump();
            let right = self.range_expr();
            let end_span = self.prev().span.clone();
            left = Expression::Bin {
                span: start_span.clone() + end_span,
                op: match op.kind {
                    TokenKind::GtColon => BinOp::Impls,
                    TokenKind::GtBang => BinOp::NotImpls,
                    _ => unreachable!(),
                },
                lhs: Box::new(left),
                rhs: Box::new(right),
            }
        }

        left
    }

    /// Compare expression parsing
    fn compare_expr(&mut self) -> Expression {
        let start_span = self.peek().span.clone();
        let mut left = self.impls_expr();

        while self.check(TokenKind::Ge)
            || self.check(TokenKind::Gt)
            || self.check(TokenKind::Le)
            || self.check(TokenKind::Lt)
        {
            let op = self.bump();
            let right = self.impls_expr();
            let end_span = self.prev().span.clone();
            left = Expression::Bin {
                span: start_span.clone() + end_span,
                op: match op.kind {
                    TokenKind::Ge => BinOp::Ge,
                    TokenKind::Gt => BinOp::Gt,
                    TokenKind::Le => BinOp::Le,
                    TokenKind::Lt => BinOp::Lt,
                    _ => unreachable!(),
                },
                lhs: Box::new(left),
                rhs: Box::new(right),
            }
        }

        left
    }

    /// Equality expression parsing
    fn equality_expr(&mut self) -> Expression {
        let start_span = self.peek().span.clone();
        let mut left = self.compare_expr();

        while self.check(TokenKind::DoubleEq) || self.check(TokenKind::BangEq) {
            let op = self.bump();
            let right = self.compare_expr();
            let end_span = self.prev().span.clone();
            left = Expression::Bin {
                span: start_span.clone() + end_span,
                op: match op.kind {
                    TokenKind::DoubleEq => BinOp::Eq,
                    TokenKind::BangEq => BinOp::Ne,
                    _ => unreachable!(),
                },
                lhs: Box::new(left),
                rhs: Box::new(right),
            }
        }

        left
    }

    /// `Bitwise and` expression parsing
    fn bit_and_expr(&mut self) -> Expression {
        let start_span = self.peek().span.clone();
        let mut left = self.equality_expr();

        while self.check(TokenKind::Ampersand) {
            self.bump();
            let right = self.equality_expr();
            let end_span = self.prev().span.clone();
            left = Expression::Bin {
                span: start_span.clone() + end_span,
                op: BinOp::BitAnd,
                lhs: Box::new(left),
                rhs: Box::new(right),
            }
        }

        left
    }

    /// `Bitwise xor` expression parsing
    fn bit_xor_expr(&mut self) -> Expression {
        let start_span = self.peek().span.clone();
        let mut left = self.bit_and_expr();

        while self.check(TokenKind::Caret) {
            self.bump();
            let right = self.bit_and_expr();
            let end_span = self.prev().span.clone();
            left = Expression::Bin {
                span: start_span.clone() + end_span,
                op: BinOp::Xor,
                lhs: Box::new(left),
                rhs: Box::new(right),
            };
        }

        left
    }

    /// `Bitwise or` expression parsing
    fn bir_or_expr(&mut self) -> Expression {
        let start_span = self.peek().span.clone();
        let mut left = self.bit_xor_expr();

        while self.check(TokenKind::Bar) {
            self.bump();
            let right = self.bit_xor_expr();
            let end_span = self.prev().span.clone();
            left = Expression::Bin {
                span: start_span.clone() + end_span,
                op: BinOp::BitOr,
                lhs: Box::new(left),
                rhs: Box::new(right),
            }
        }

        left
    }

    /// `Logical and` expression parsing
    fn logical_and_expr(&mut self) -> Expression {
        let start_span = self.peek().span.clone();
        let mut left = self.bir_or_expr();

        while self.check(TokenKind::DoubleAmp) {
            self.bump();
            let right = self.bir_or_expr();
            let end_span = self.prev().span.clone();
            left = Expression::Bin {
                span: start_span.clone() + end_span,
                op: BinOp::And,
                lhs: Box::new(left),
                rhs: Box::new(right),
            }
        }

        left
    }

    /// `Logical or` expression parsing
    fn logical_or_expr(&mut self) -> Expression {
        let start_span = self.peek().span.clone();
        let mut left = self.logical_and_expr();

        while self.check(TokenKind::DoubleBar) {
            self.bump();
            let right = self.logical_and_expr();
            let end_span = self.prev().span.clone();
            left = Expression::Bin {
                span: start_span.clone() + end_span,
                op: BinOp::Or,
                lhs: Box::new(left),
                rhs: Box::new(right),
            }
        }

        left
    }

    /// Parses expression
    pub fn expr(&mut self) -> Expression {
        self.logical_or_expr()
    }
}
