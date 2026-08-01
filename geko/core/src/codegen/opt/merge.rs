/// Imports
use dune::ops::{Chunk, Opcode};

/// Tries to merge two instructions into one super instruction
pub fn try_merege(a: Opcode, b: Opcode) -> (Opcode, Opcode) {
    match (a, b) {
        (Opcode::Eq, Opcode::JumpIfTrue(label)) => (Opcode::JumpIfEq(label), Opcode::Nop),
        (Opcode::Ne, Opcode::JumpIfTrue(label)) => (Opcode::JumpIfNe(label), Opcode::Nop),
        (Opcode::Gt, Opcode::JumpIfTrue(label)) => (Opcode::JumpIfGt(label), Opcode::Nop),
        (Opcode::Ge, Opcode::JumpIfTrue(label)) => (Opcode::JumpIfGe(label), Opcode::Nop),
        (Opcode::Lt, Opcode::JumpIfTrue(label)) => (Opcode::JumpIfLt(label), Opcode::Nop),
        (Opcode::Le, Opcode::JumpIfTrue(label)) => (Opcode::JumpIfLe(label), Opcode::Nop),
        (Opcode::Eq, Opcode::JumpIfFalse(label)) => (Opcode::JumpIfNe(label), Opcode::Nop),
        (Opcode::Ne, Opcode::JumpIfFalse(label)) => (Opcode::JumpIfEq(label), Opcode::Nop),
        (Opcode::Gt, Opcode::JumpIfFalse(label)) => (Opcode::JumpIfLe(label), Opcode::Nop),
        (Opcode::Ge, Opcode::JumpIfFalse(label)) => (Opcode::JumpIfLt(label), Opcode::Nop),
        (Opcode::Lt, Opcode::JumpIfFalse(label)) => (Opcode::JumpIfGe(label), Opcode::Nop),
        (Opcode::Le, Opcode::JumpIfFalse(label)) => (Opcode::JumpIfGt(label), Opcode::Nop),
        (a, b) => (a, b),
    }
}

/// Performs merge optimization
pub fn merge_optimization(mut chunk: Chunk) -> Chunk {
    // If chunk has at least one instruction
    if chunk.code.len() > 0 {
        // Iterating chunk opcodes
        for i in 0..(chunk.code.len() - 1) {
            // Getting first and second opcodes
            let first = chunk.code[i].clone();
            let second = chunk.code[i + 1].clone();

            // Trying to merge
            (chunk.code[i], chunk.code[i + 1]) = try_merege(first, second)
        }
    }

    // Returning modified chunk
    chunk
}
