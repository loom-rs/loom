/// Imports
use crate::value::Value;
use miette::{Diagnostic, NamedSource, SourceSpan};
use std::sync::Arc;
use thiserror::Error;

/// Unsafe `Send` + `Sync` implementations.
unsafe impl Send for Value {}
unsafe impl Sync for Value {}

/// Runtime error
#[derive(Error, Diagnostic, Debug)]
pub enum RuntimeError {
    /// Undefined variable
    #[error("variable `{name}` is not defined")]
    #[diagnostic(code(rt::undefined_variable))]
    UndefinedVariable {
        name: String,
        #[source_code]
        src: Arc<NamedSource<String>>,
        #[label("variable access here...")]
        span: SourceSpan,
    },
    /// Undefined field
    #[error("field `{name}` is not defined")]
    #[diagnostic(code(rt::undefined_field))]
    UndefinedField {
        name: String,
        #[source_code]
        src: Arc<NamedSource<String>>,
        #[label("field access here...")]
        span: SourceSpan,
    },
    /// Invalid binary op
    #[error("couldn't use `{op}` with `{a}` and `{b}`")]
    #[diagnostic(code(rt::invalid_bin_op))]
    InvalidBinOp {
        op: String,
        a: Value,
        b: Value,
        #[source_code]
        src: Arc<NamedSource<String>>,
        #[label("here...")]
        span: SourceSpan,
    },
    /// Invalid unary op
    #[error("couldn't use `{op}` with `{value}`")]
    #[diagnostic(code(rt::invalid_unary_op))]
    InvalidUnaryOp {
        op: String,
        value: Value,
        #[source_code]
        src: Arc<NamedSource<String>>,
        #[label("here...")]
        span: SourceSpan,
    },
    /// Couldn't resolve fields
    #[error("`{value}` has no fields")]
    #[diagnostic(code(rt::could_not_lookup_field))]
    CouldNotLookupField {
        value: Value,
        #[source_code]
        src: Arc<NamedSource<String>>,
        #[label("lookup here...")]
        span: SourceSpan,
    },
    /// Couldn't assign field
    #[error("couldn't assign field in `{value}`")]
    #[diagnostic(code(rt::could_not_assign_field))]
    CouldNotAssignField {
        value: Value,
        #[source_code]
        src: Arc<NamedSource<String>>,
        #[label("assign here...")]
        span: SourceSpan,
    },
    /// Couldn't define field
    #[error("couldn't define field in `{value}`")]
    #[diagnostic(code(rt::could_not_define_field))]
    CouldNotDefineField {
        value: Value,
        #[source_code]
        src: Arc<NamedSource<String>>,
        #[label("define here...")]
        span: SourceSpan,
    },
    /// Couldn't call a value
    #[error("couldn't call `{value}`")]
    #[diagnostic(code(rt::could_not_call))]
    CouldNotCall {
        value: Value,
        #[source_code]
        src: Arc<NamedSource<String>>,
        #[label("here...")]
        span: SourceSpan,
    },
    /// Incorrect arity
    #[error("incorrect arity. expected {params} params got {args} args")]
    #[diagnostic(code(rt::incorrect_arity))]
    IncorrectArity {
        params: usize,
        args: usize,
        #[source_code]
        src: Arc<NamedSource<String>>,
        #[label("here...")]
        span: SourceSpan,
    },
    /// Unhandled error
    #[error("unhandled error: `{error}`")]
    #[diagnostic(code(rt::unhandled_error))]
    UnhandledError {
        error: Value,
        #[source_code]
        src: Arc<NamedSource<String>>,
        #[label("error raised here...")]
        span: SourceSpan,
    },
    /// Bail
    #[error("bail: {text}")]
    #[diagnostic(code(rt::bail))]
    Bail {
        text: String,
        #[source_code]
        src: Arc<NamedSource<String>>,
        #[label("bail occurred here...")]
        span: SourceSpan,
    },
}
