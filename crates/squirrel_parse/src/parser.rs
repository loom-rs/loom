/// Import
use crate::errors::ParseError;
use miette::NamedSource;
use squirrel_ast::{
    atom::{AssignOp, BinOp, Function, Lit, TraitFunction, UnaryOp},
    expr::Expression,
    stmt::{Block, Statement, UsageKind},
};
use squirrel_common::bail;
use squirrel_lex::{
    lexer::Lexer,
    token::{Span, Token, TokenKind},
};
use std::sync::Arc;

/// Parser converts a stream of tokens
/// produced by the lexer into an abstract syntax tree (AST).
pub struct Parser<'s> {
    /// Named source of the file
    source: Arc<NamedSource<String>>,

    /// Lexer used to iterate over tokens
    lexer: Lexer<'s>,

    /// Previously consumed token
    /// (useful for spans and error reporting)
    previous: Option<Token>,

    /// Current token under inspection
    current: Option<Token>,

    /// Lookahead token
    /// (used for predictive parsing)
    next: Option<Token>,
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

        self.expect(TokenKind::If);
        let condition = self.expr();
        let then = self.block();
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

    /// Let statement parsing
    fn let_stmt(&mut self) -> Statement {
        let start_span = self.peek().span.clone();

        self.expect(TokenKind::Let);
        let name = self.expect(TokenKind::Id).lexeme;
        self.expect(TokenKind::Eq);
        let value = self.expr();

        let end_span = self.prev().span.clone();

        Statement::Let {
            span: start_span + end_span,
            name,
            value,
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

        Statement::Class {
            span: start_span + end_span,
            name_span,
            name: name.lexeme,
            methods,
        }
    }

    /// Enum declaration parsing
    fn enum_stmt(&mut self) -> Statement {
        let start_span = self.peek().span.clone();

        // Parsing enum name
        self.expect(TokenKind::Enum);
        let name = self.expect(TokenKind::Id);

        // Parsing variants
        let variants = self.sep_by(
            TokenKind::Lbrace,
            TokenKind::Rbrace,
            TokenKind::Comma,
            |p| p.expect(TokenKind::Id).lexeme,
        );

        let end_span = self.prev().span.clone();

        Statement::Enum {
            span: start_span + end_span,
            name: name.lexeme,
            variants,
        }
    }

    /// Trait function parsing
    fn trait_fn(&mut self) -> TraitFunction {
        let start_span = self.peek().span.clone();

        // Parsing trait signature
        self.expect(TokenKind::Fn);
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
            |p| p.trait_fn(),
        );

        let end_span = self.prev().span.clone();

        Statement::Trait {
            span: start_span + end_span,
            name,
            functions,
        }
    }

    /// Assignment statement
    fn assignment_stmt(&mut self) -> Statement {
        let start_span = self.peek().span.clone();
        let variable = self.variable();
        // Checking for ssignment operator
        let op = match self.peek().kind {
            TokenKind::PlusEq => Some(AssignOp::Add),
            TokenKind::MinusEq => Some(AssignOp::Sub),
            TokenKind::StarEq => Some(AssignOp::Mul),
            TokenKind::SlashEq => Some(AssignOp::Div),
            TokenKind::PercentEq => Some(AssignOp::Mod),
            TokenKind::AmpersandEq => Some(AssignOp::BitAnd),
            TokenKind::BarEq => Some(AssignOp::BitOr),
            TokenKind::CaretEq => Some(AssignOp::Xor),
            TokenKind::Eq => Some(AssignOp::Assign),
            _ => None,
        };
        // Checking assignment operator existence
        match op {
            // If operator found
            Some(op) => {
                // Bumping operator
                self.bump();
                let value = self.expr();
                let end_span = self.prev().span.clone();
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

        if self.check(TokenKind::Semi) {
            let end_span = self.prev().span.clone();
            Statement::Return {
                span: start_span + end_span,
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

    /// Use statement
    fn use_stmt(&mut self) -> Statement {
        let start_span = self.peek().span.clone();
        self.expect(TokenKind::Use);

        // Import path
        let mut path = String::new();
        path.push_str(&self.expect(TokenKind::Id).lexeme);
        while self.check(TokenKind::Slash) {
            self.bump();
            path.push_str(&self.expect(TokenKind::Id).lexeme);
        }

        // Import kind
        let kind = if self.check(TokenKind::As) {
            self.bump();
            UsageKind::As(self.expect(TokenKind::Id).lexeme)
        } else if self.check(TokenKind::For) {
            self.bump();
            if self.check(TokenKind::Star) {
                self.bump();
                UsageKind::All
            } else {
                let mut items = Vec::new();
                items.push(self.expect(TokenKind::Id).lexeme);
                while self.check(TokenKind::Comma) {
                    self.bump();
                    items.push(self.expect(TokenKind::Id).lexeme);
                }
                UsageKind::For(items)
            }
        } else {
            UsageKind::Just
        };
        let end_span = self.prev().span.clone();

        Statement::Use {
            span: start_span + end_span,
            path,
            kind,
        }
    }

    /// Bail statement
    fn bail_stmt(&mut self) -> Statement {
        let start_span = self.peek().span.clone();
        self.expect(TokenKind::Bail);
        let message = self.expr();
        let end_span = self.prev().span.clone();

        Statement::Bail {
            span: start_span + end_span,
            message,
        }
    }

    /// Satement parsing
    fn stmt(&mut self) -> Statement {
        // Parsing statement
        let stmt = match self.peek().kind {
            TokenKind::For => self.for_stmt(),
            TokenKind::While => self.while_stmt(),
            TokenKind::If => self.if_stmt(),
            TokenKind::Let => self.let_stmt(),
            TokenKind::Class => self.class_stmt(),
            TokenKind::Enum => self.enum_stmt(),
            TokenKind::Trait => self.trait_stmt(),
            TokenKind::Fn => Statement::Function(self.function()),
            TokenKind::Return => self.return_stmt(),
            TokenKind::Continue => self.continue_stmt(),
            TokenKind::Break => self.break_stmt(),
            TokenKind::Id => self.assignment_stmt(),
            TokenKind::Use => self.use_stmt(),
            TokenKind::Bail => self.bail_stmt(),
            _ => Statement::Expr(self.expr()),
        };
        // If statement requires semicolon
        if stmt.requires_semi() {
            self.expect(TokenKind::Semi);
        }
        stmt
    }

    /// Block parsing
    fn block(&mut self) -> Block {
        let mut statements = Vec::new();

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

    /// Variable parsing
    fn variable(&mut self) -> Expression {
        // parsing base identifier
        let start_span = self.peek().span.clone();
        let id = self.expect(TokenKind::Id).lexeme;

        // result node
        let mut result = Expression::Variable {
            span: start_span.clone(),
            name: id,
        };

        // checking for dots and parens
        loop {
            // checking for chain `a.b.c.d`
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
            // checking for call
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
            // breaking cycle
            break;
        }
        result
    }

    /// Group expression parsing
    fn group(&mut self) -> Expression {
        self.expect(TokenKind::Lparen);
        let expr = self.expr();
        self.expect(TokenKind::Rparen);
        expr
    }

    /// Function parsing
    fn function(&mut self) -> Function {
        // Parsing function name
        let start_span = self.peek().span.clone();
        self.expect(TokenKind::Fn);
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

    /// List expression parsing
    fn list(&mut self) -> Expression {
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
    fn pair(&mut self) -> (Expression, Expression) {
        let key = self.expr();
        self.expect(TokenKind::Colon);
        let value = self.expr();

        (key, value)
    }

    /// Dict expression parsing
    fn dict(&mut self) -> Expression {
        let start_span = self.peek().span.clone();
        let dict = self.sep_by(
            TokenKind::Lbrace,
            TokenKind::Rbrace,
            TokenKind::Comma,
            |p| p.pair(),
        );
        let end_span = self.prev().span.clone();

        Expression::Dict {
            span: start_span + end_span,
            dict,
        }
    }

    /// Anonymous function parsing
    fn anon_fn(&mut self) -> Expression {
        let start_span = self.peek().span.clone();

        // Parsing function params
        let params = if self.check(TokenKind::Bar) {
            self.sep_by(TokenKind::Bar, TokenKind::Bar, TokenKind::Comma, |p| {
                p.expect(TokenKind::Id).lexeme
            })
        } else {
            self.expect(TokenKind::DoubleBar);
            Vec::new()
        };

        // Parsing function body
        let block = if self.check(TokenKind::Lbrace) {
            self.block()
        } else {
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
        };

        let end_span = self.prev().span.clone();
        Expression::Fn {
            span: start_span + end_span,
            params,
            block,
        }
    }

    /// Atom expression parsing
    fn atom(&mut self) -> Expression {
        let tk = self.peek().clone();
        match tk.kind {
            TokenKind::Lparen => self.group(),
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
            TokenKind::Id => self.variable(),
            TokenKind::Lbracket => self.list(),
            TokenKind::Lbrace => self.dict(),
            TokenKind::Bar | TokenKind::DoubleBar => self.anon_fn(),
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
        self.atom()
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

    /// `bitwise and` expression parsing
    fn bitwise_and_expr(&mut self) -> Expression {
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

    /// `bitwise xor` expression parsing
    fn bitwise_xor_expr(&mut self) -> Expression {
        let start_span = self.peek().span.clone();
        let mut left = self.bitwise_and_expr();
        while self.check(TokenKind::Caret) {
            self.bump();
            let right = self.bitwise_and_expr();
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

    /// `bitwise or` expression parsing
    fn bitwise_or_expr(&mut self) -> Expression {
        let start_span = self.peek().span.clone();
        let mut left = self.bitwise_xor_expr();
        while self.check(TokenKind::Bar) {
            self.bump();
            let right = self.bitwise_xor_expr();
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
        let mut left = self.bitwise_or_expr();
        while self.check(TokenKind::DoubleAmp) {
            self.bump();
            let right = self.bitwise_or_expr();
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
    fn expr(&mut self) -> Expression {
        self.logical_or_expr()
    }

    /// Checks token match
    fn check(&self, tk: TokenKind) -> bool {
        match &self.current {
            Some(it) => it.kind == tk,
            None => false,
        }
    }

    /// Retrieves current token
    fn peek(&self) -> &Token {
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
    fn prev(&self) -> &Token {
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
    fn expect(&mut self, tk: TokenKind) -> Token {
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
    fn bump(&mut self) -> Token {
        self.previous = self.current.take();
        self.current = self.next.take();
        self.next = self.lexer.next();
        self.previous.clone().unwrap()
    }
}
