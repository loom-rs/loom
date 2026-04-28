/// Imports
use crate::rt::value::Value;
use geko_ir::atom::{BinOp, UnaryOp};
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
        op: BinOp,
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
        op: UnaryOp,
        value: Value,
        #[source_code]
        src: Arc<NamedSource<String>>,
        #[label("here...")]
        span: SourceSpan,
    },
    /// Zero division error
    #[error("division by zero is invalid")]
    #[diagnostic(code(rt::zero_division))]
    ZeroDivision {
        #[source_code]
        src: Arc<NamedSource<String>>,
        #[label("here...")]
        span: SourceSpan,
    },
    /// Couldn't resolve fields
    #[error("couldn't resolve fields in `{value}`")]
    #[diagnostic(code(rt::could_not_resolve_fields))]
    CouldNotResolveFields {
        value: Value,
        #[source_code]
        src: Arc<NamedSource<String>>,
        #[label("here...")]
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
    /// Expected boolean value
    #[error("expected bool value. got `{value}`")]
    #[diagnostic(code(rt::expected_bool_value))]
    ExpectedBool {
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
    /// Failed to find module
    #[error("failed to find module `{name}`")]
    #[diagnostic(code(rt::failed_to_find_module))]
    FailedToFindModule {
        name: String,
        #[source_code]
        src: Arc<NamedSource<String>>,
        #[label("here...")]
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
    /// Invalid range
    #[error("couldn't create valid range with `{lhs}` and `{rhs}`")]
    #[diagnostic(code(rt::invalid_range))]
    InvalidRange {
        src: Arc<NamedSource<String>>,
        #[label("tried to create range here...")]
        span: SourceSpan,
        lhs: Value,
        rhs: Value,
    },
    /// Invalid iterator
    #[error("couldn't iterate over `{value}`")]
    #[diagnostic(code(rt::invalid_iterator))]
    InvalidIterator {
        src: Arc<NamedSource<String>>,
        #[label("tried to iterate here...")]
        span: SourceSpan,
        value: Value,
    },
    /// Corrupted iterator
    #[error("couldn't iterate over corrupted iterator")]
    #[diagnostic(code(rt::corrupted_iterator))]
    CorruptedIterator {
        src: Arc<NamedSource<String>>,
        #[label("tried to iterate here...")]
        span: SourceSpan,
    },
}
