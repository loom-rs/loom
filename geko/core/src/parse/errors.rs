/// Imports
use crate::lex::token::TokenKind;
use miette::{Diagnostic, NamedSource, SourceSpan};
use std::sync::Arc;
use thiserror::Error;

/// Defines parsing error
#[derive(Error, Diagnostic, Debug)]
pub enum ParseError {
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
    #[error("unexpected end of file.")]
    #[diagnostic(code(parse::unexpected_eof))]
    UnexpectedEof {
        #[source_code]
        src: Arc<NamedSource<String>>,
        #[label("while parsing that...")]
        span: SourceSpan,
    },
}
