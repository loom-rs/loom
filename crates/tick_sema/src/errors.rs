/// Imports
use miette::{Diagnostic, NamedSource, SourceSpan};
use std::sync::Arc;
use thiserror::Error;

/// Semantic analysis error
#[derive(Error, Diagnostic, Debug)]
pub enum SemaError {
    /// Break outside loop
    #[error("couldn't use `break` statement outside of loop.")]
    #[diagnostic(code(sema::break_outside_loop))]
    BreakOutsideLoop {
        #[source_code]
        src: Arc<NamedSource<String>>,
        #[label("this `break` statement is invalid.")]
        span: SourceSpan,
    },
    /// Continue outside loop
    #[error("couldn't use `continue` statement outside of loop.")]
    #[diagnostic(code(sema::continue_outside_loop))]
    ContinueOutsideLoop {
        #[source_code]
        src: Arc<NamedSource<String>>,
        #[label("this `continue` statement is invalid.")]
        span: SourceSpan,
    },
    /// Return outside function
    #[error("couldn't use `return` statement outside of function.")]
    #[diagnostic(code(sema::return_outside_fn))]
    ReturnOutsideFn {
        #[source_code]
        src: Arc<NamedSource<String>>,
        #[label("this `return` statement is invalid.")]
        span: SourceSpan,
    },
}
