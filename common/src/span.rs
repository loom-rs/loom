/// Imports
use miette::NamedSource;
use std::{
    fmt::Debug,
    ops::{Add, Range},
    sync::Arc,
};

/// Represents source code span
#[derive(PartialEq, Clone, Eq)]
pub struct Span(pub Arc<NamedSource<String>>, pub Range<usize>);

/// Debug implementation
impl Debug for Span {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Span").field(&self.1).finish()
    }
}

/// Add implementation
impl Add for Span {
    type Output = Span;

    fn add(self, rhs: Self) -> Self::Output {
        // Checking that files are same
        if self.0 != rhs.0 {
            panic!("attemp to perform `+` operation on two spans from different files.")
        }
        // Calculating new span range
        let start = self.1.start.min(rhs.1.start);
        let end = self.1.end.max(rhs.1.end);
        Span(self.0, start..end)
    }
}
