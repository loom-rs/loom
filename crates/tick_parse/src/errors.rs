/// Imports
use tick_lex::token::TokenKind;
use miette::{Diagnostic, NamedSource, SourceSpan};
use std::sync::Arc;
use thiserror::Error;

/// Parser error
#[derive(Error, Diagnostic, Debug)]
pub enum ParseError {
    /// Unexpected token
    #[error("unexpected token `{got:?}`. expected `{expected:?}`")]
    #[diagnostic(code(parse::unexpected_tk))]
    UnexpectedToken {
        got: TokenKind,
        expected: TokenKind,
        #[source_code]
        src: Arc<NamedSource<String>>,
        #[label("got unexpected token here...")]
        span: SourceSpan,
        #[label("while parsing that...")]
        prev: SourceSpan,
    },
    /// Unexpected expr token
    #[error("unexpected expression token `{got:?}`.")]
    #[diagnostic(
        code(parse::unexpected_expr_tk),
        help("token {got:?} can't be start of the expression.")
    )]
    UnexpectedExprToken {
        got: TokenKind,
        #[source_code]
        src: Arc<NamedSource<String>>,
        #[label("got unexpected token here...")]
        span: SourceSpan,
    },
    /// Unexpected end of file
    #[error("unexpected end of file.")]
    #[diagnostic(code(parse::unexpected_eof))]
    UnexpectedEof {
        #[source_code]
        src: Arc<NamedSource<String>>,
        #[label("while parsing that...")]
        span: SourceSpan,
    },
    /// Invalid usage of assignment operator
    #[error("invalid use of assignment operator.")]
    #[diagnostic(
        code(lex::invalid_use_of_assignment_op),
        help("assignment op-s can be used only with variable expressions.")
    )]
    InvalidUseOfAssignOp {
        #[source_code]
        src: Arc<NamedSource<String>>,
        #[label("variable expression was expected.")]
        first_span: SourceSpan,
    },
}
