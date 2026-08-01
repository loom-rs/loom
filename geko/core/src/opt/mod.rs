/// Different kinds of optimizations
pub mod merge;
pub mod peephole;

/// Imports
use crate::opt::{merge::merge_optimization, peephole::peephole_optimization};
use dune::ops::Chunk;

/// Performs all the optimizations
pub fn optimize(chunk: Chunk) -> Chunk {
    merge_optimization(peephole_optimization(chunk))
}
