/// Imports
use miette::{Diagnostic, NamedSource, SourceSpan};
use std::sync::Arc;
use thiserror::Error;

/// Defines semantic analysis error
#[derive(Error, Diagnostic, Debug)]
pub enum SemaError {
    #[error("couldn't use `break` statement outside of loop.")]
    #[diagnostic(code(sema::break_outside_loop))]
    BreakOutsideLoop {
        #[source_code]
        src: Arc<NamedSource<String>>,
        #[label("this `break` statement is invalid.")]
        span: SourceSpan,
    },
    #[error("couldn't use `continue` statement outside of loop.")]
    #[diagnostic(code(sema::continue_outside_loop))]
    ContinueOutsideLoop {
        #[source_code]
        src: Arc<NamedSource<String>>,
        #[label("this `continue` statement is invalid.")]
        span: SourceSpan,
    },
    #[error("couldn't use `return` statement outside of function.")]
    #[diagnostic(code(sema::return_outside_fn))]
    ReturnOutsideFn {
        #[source_code]
        src: Arc<NamedSource<String>>,
        #[label("this `return` statement is invalid.")]
        span: SourceSpan,
    },
    #[error("invalid left-hand side of assign.")]
    #[diagnostic(code(lex::invalid_assign_lhs))]
    InvalidAssignLhs {
        #[source_code]
        src: Arc<NamedSource<String>>,
        #[label("variable or field expression was expected.")]
        span: SourceSpan,
    },
}
