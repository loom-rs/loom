/// Imports
use crate::lex::token::TokenKind;
use crate::parse::ast::{AssignOp, BinOp, Block, Expr, Lit, Stmt, UnOp};
use crate::parse::{Parser, errors::ParseError};
use common::bail;

/// Expressions parsing
impl<'s> Parser<'s> {
    /// Variable parsing
    pub fn variable_expr(&mut self) -> Expr {
        // Bumping base identifier
        let start_span = self.peek().span.clone();
        let id = self.bump().lexeme;

        // Result node
        let mut result = Expr::Variable {
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
                result = Expr::Field {
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
                result = Expr::Call {
                    span: start_span.clone() + end_span,
                    callee: Box::new(result),
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
    fn group_expr(&mut self) -> Expr {
        self.expect(TokenKind::Lparen);
        let expr = self.expr();
        self.expect(TokenKind::Rparen);
        expr
    }

    /// List expression parsing
    fn list_expr(&mut self) -> Expr {
        let start_span = self.peek().span.clone();
        let list = self.sep_by(
            TokenKind::Lbracket,
            TokenKind::Rbracket,
            TokenKind::Comma,
            |p| p.expr(),
        );
        let end_span = self.prev().span.clone();

        Expr::List {
            span: start_span + end_span,
            list,
        }
    }

    /// Single dict pair parsing
    fn dict_pair(&mut self) -> (Expr, Expr) {
        let key = self.expr();
        self.expect(TokenKind::Colon);
        let value = self.expr();

        (key, value)
    }

    /// Dict expression parsing
    fn dict_expr(&mut self) -> Expr {
        let start_span = self.peek().span.clone();
        let dict = self.sep_by(
            TokenKind::Lbrace,
            TokenKind::Rbrace,
            TokenKind::Comma,
            |p| p.dict_pair(),
        );
        let end_span = self.prev().span.clone();

        Expr::Dict {
            span: start_span + end_span,
            dict,
        }
    }

    /// Anonymous function parsing
    fn anon_fun_expr(&mut self) -> Expr {
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
                stmts: vec![Stmt::Return {
                    span: start_span + end_span,
                    value: expr,
                }],
            }
        } else {
            self.block()
        };

        let end_span = self.prev().span.clone();
        Expr::Function {
            span: start_span + end_span,
            params,
            block,
        }
    }

    /// Atom expression parsing
    fn atom_expr(&mut self) -> Expr {
        let tk = self.peek().clone();
        match tk.kind {
            TokenKind::Lparen => self.group_expr(),
            TokenKind::Number => {
                let expr = Expr::Lit {
                    span: tk.span,
                    lit: Lit::Number(tk.lexeme),
                };
                self.bump();
                expr
            }
            TokenKind::String => {
                let expr = Expr::Lit {
                    span: tk.span,
                    lit: Lit::String(tk.lexeme),
                };
                self.bump();
                expr
            }
            TokenKind::Bool => {
                let expr = Expr::Lit {
                    span: tk.span,
                    lit: Lit::Bool(tk.lexeme),
                };
                self.bump();
                expr
            }
            TokenKind::Null => {
                let expr = Expr::Lit {
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
    fn unary_expr(&mut self) -> Expr {
        if self.check(TokenKind::Minus) || self.check(TokenKind::Bang) {
            let start_span = self.peek().span.clone();
            let op = self.bump();
            let value = self.unary_expr();
            let end_span = self.prev().span.clone();
            Expr::Un {
                span: start_span.clone() + end_span,
                op: match op.kind {
                    TokenKind::Minus => UnOp::Neg,
                    TokenKind::Bang => UnOp::Bang,
                    _ => unreachable!(),
                },
                value: Box::new(value),
            }
        } else {
            self.atom_expr()
        }
    }

    /// Factor expression parsing
    fn factor_expr(&mut self) -> Expr {
        let start_span = self.peek().span.clone();
        let mut left = self.unary_expr();

        while self.check(TokenKind::Star)
            || self.check(TokenKind::Slash)
            || self.check(TokenKind::Percent)
        {
            let op = self.bump();
            let right = self.unary_expr();
            let end_span = self.prev().span.clone();
            left = Expr::Bin {
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
    fn term_expr(&mut self) -> Expr {
        let start_span = self.peek().span.clone();
        let mut left = self.factor_expr();

        while self.check(TokenKind::Plus) || self.check(TokenKind::Minus) {
            let op = self.bump();
            let right = self.factor_expr();
            let end_span = self.prev().span.clone();
            left = Expr::Bin {
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
    fn range_expr(&mut self) -> Expr {
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
            left = Expr::Range {
                span: start_span.clone() + end_span,
                lhs: Box::new(left),
                rhs: Box::new(right),
                includes_end,
            }
        }

        left
    }

    /// Impls expression parsing
    fn impls_expr(&mut self) -> Expr {
        let start_span = self.peek().span.clone();
        let mut left = self.range_expr();

        while self.check(TokenKind::GtColon) | self.check(TokenKind::GtBang) {
            let op = self.bump();
            let right = self.range_expr();
            let end_span = self.prev().span.clone();
            left = Expr::Bin {
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
    fn compare_expr(&mut self) -> Expr {
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
            left = Expr::Bin {
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
    fn equality_expr(&mut self) -> Expr {
        let start_span = self.peek().span.clone();
        let mut left = self.compare_expr();

        while self.check(TokenKind::DoubleEq) || self.check(TokenKind::BangEq) {
            let op = self.bump();
            let right = self.compare_expr();
            let end_span = self.prev().span.clone();
            left = Expr::Bin {
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
    fn bit_and_expr(&mut self) -> Expr {
        let start_span = self.peek().span.clone();
        let mut left = self.equality_expr();

        while self.check(TokenKind::Amp) {
            self.bump();
            let right = self.equality_expr();
            let end_span = self.prev().span.clone();
            left = Expr::Bin {
                span: start_span.clone() + end_span,
                op: BinOp::BitAnd,
                lhs: Box::new(left),
                rhs: Box::new(right),
            }
        }

        left
    }

    /// `Bitwise xor` expression parsing
    fn bit_xor_expr(&mut self) -> Expr {
        let start_span = self.peek().span.clone();
        let mut left = self.bit_and_expr();

        while self.check(TokenKind::Caret) {
            self.bump();
            let right = self.bit_and_expr();
            let end_span = self.prev().span.clone();
            left = Expr::Bin {
                span: start_span.clone() + end_span,
                op: BinOp::Xor,
                lhs: Box::new(left),
                rhs: Box::new(right),
            };
        }

        left
    }

    /// `Bitwise or` expression parsing
    fn bir_or_expr(&mut self) -> Expr {
        let start_span = self.peek().span.clone();
        let mut left = self.bit_xor_expr();

        while self.check(TokenKind::Bar) {
            self.bump();
            let right = self.bit_xor_expr();
            let end_span = self.prev().span.clone();
            left = Expr::Bin {
                span: start_span.clone() + end_span,
                op: BinOp::BitOr,
                lhs: Box::new(left),
                rhs: Box::new(right),
            }
        }

        left
    }

    /// `Logical and` expression parsing
    fn logical_and_expr(&mut self) -> Expr {
        let start_span = self.peek().span.clone();
        let mut left = self.bir_or_expr();

        while self.check(TokenKind::DoubleAmp) {
            self.bump();
            let right = self.bir_or_expr();
            let end_span = self.prev().span.clone();
            left = Expr::Bin {
                span: start_span.clone() + end_span,
                op: BinOp::And,
                lhs: Box::new(left),
                rhs: Box::new(right),
            }
        }

        left
    }

    /// `Logical or` expression parsing
    fn logical_or_expr(&mut self) -> Expr {
        let start_span = self.peek().span.clone();
        let mut left = self.logical_and_expr();

        while self.check(TokenKind::DoubleBar) {
            self.bump();
            let right = self.logical_and_expr();
            let end_span = self.prev().span.clone();
            left = Expr::Bin {
                span: start_span.clone() + end_span,
                op: BinOp::Or,
                lhs: Box::new(left),
                rhs: Box::new(right),
            }
        }

        left
    }

    /// Assign expression parsing
    fn assign_expr(&mut self) -> Expr {
        let start_span = self.peek().span.clone();
        let mut left = self.logical_or_expr();

        while self.check(TokenKind::PlusEq)
            | self.check(TokenKind::MinusEq)
            | self.check(TokenKind::StarEq)
            | self.check(TokenKind::SlashEq)
            | self.check(TokenKind::PercentEq)
            | self.check(TokenKind::AmpEq)
            | self.check(TokenKind::BarEq)
            | self.check(TokenKind::CaretEq)
            | self.check(TokenKind::Eq)
            | self.check(TokenKind::Walrus)
        {
            let op = self.bump();
            let right = self.logical_or_expr();
            let end_span = self.prev().span.clone();
            left = Expr::Assign {
                span: start_span.clone() + end_span,
                what: Box::new(left),
                op: match op.kind {
                    TokenKind::PlusEq => AssignOp::Add,
                    TokenKind::MinusEq => AssignOp::Sub,
                    TokenKind::StarEq => AssignOp::Mul,
                    TokenKind::SlashEq => AssignOp::Div,
                    TokenKind::PercentEq => AssignOp::Mod,
                    TokenKind::AmpEq => AssignOp::BitAnd,
                    TokenKind::BarEq => AssignOp::BitOr,
                    TokenKind::CaretEq => AssignOp::Xor,
                    TokenKind::Eq => AssignOp::Assign,
                    TokenKind::Walrus => AssignOp::Define,
                    _ => unreachable!(),
                },
                value: Box::new(right),
            }
        }

        left
    }

    /// Parses expression
    pub fn expr(&mut self) -> Expr {
        self.assign_expr()
    }
}
