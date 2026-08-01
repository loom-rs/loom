/// Different kinds of optimizations
pub mod fold;
pub mod merge;
pub mod peephole;

use crate::codegen::opt::fold::fold_optimization;

/// Imports
use super::opt::{merge::merge_optimization, peephole::peephole_optimization};
use dune::ops::Chunk;

/// Performs all the optimizations
pub fn optimize(chunk: Chunk) -> Chunk {
    merge_optimization(peephole_optimization(fold_optimization(chunk)))
}
