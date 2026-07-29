/// Imports
use miette::{Diagnostic, NamedSource, SourceSpan};
use std::sync::Arc;
use thiserror::Error;

/// Defines lexical analysis error
#[derive(Error, Diagnostic, Debug)]
pub enum LexError<'a> {
    #[error("unexpected character `{ch}`.")]
    #[diagnostic(code(lex::unexpected_char))]
    UnexpectedChar {
        ch: char,
        #[source_code]
        src: Arc<NamedSource<String>>,
        #[label("try to remove this character.")]
        span: SourceSpan,
    },
    #[error("found unclosed string quotes.")]
    #[diagnostic(code(lex::unclosed_string_quotes))]
    UnclosedStringQuotes {
        #[source_code]
        src: Arc<NamedSource<String>>,
        #[label("close string quotes by appending missed quote `\"`.")]
        span: SourceSpan,
    },
    #[error("found unterminated comment.")]
    #[diagnostic(code(lex::unterminated_comment))]
    UnterminatedComment {
        #[source_code]
        src: Arc<NamedSource<String>>,
        #[label("use `]#` to terminate comment")]
        span: SourceSpan,
    },
    #[error("invalid escape sequence.")]
    #[diagnostic(code(lex::invalid_escape_sequence), help("{cause}"))]
    InvalidEscapeSequence {
        #[source_code]
        src: Arc<NamedSource<String>>,
        #[label("this escape sequence isn't valid.")]
        span: SourceSpan,
        cause: &'a str,
    },
    #[error("unknown escape sequence.")]
    #[diagnostic(code(lex::unknown_escape_sequence))]
    UnknownEscapeSequence {
        #[source_code]
        src: Arc<NamedSource<String>>,
        #[label("this escape sequence isn't valid.")]
        span: SourceSpan,
    },
}
