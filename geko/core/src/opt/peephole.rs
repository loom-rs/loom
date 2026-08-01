/// Imports
use dune::{
    ops::{Chunk, Opcode},
    value::Value,
};

/// Tries to perform peephole optimization
pub fn try_peephole(a: Opcode, b: Opcode) -> (Opcode, Opcode) {
    match (a, b) {
        (Opcode::Store(name), Opcode::Load(name2)) if name == name2 => {
            (Opcode::Dup, Opcode::Store(name))
        }
        (Opcode::Push(Value::Int(0)), Opcode::Add) => (Opcode::Nop, Opcode::Nop),
        (Opcode::Push(Value::Int(0)), Opcode::Sub) => (Opcode::Nop, Opcode::Nop),
        (Opcode::Push(Value::Int(0)), Opcode::Mul) => (Opcode::Push(Value::Int(0)), Opcode::Nop),
        (Opcode::Push(Value::Int(1)), Opcode::Mul) => (Opcode::Nop, Opcode::Nop),
        (Opcode::Push(Value::Int(1)), Opcode::Div) => (Opcode::Nop, Opcode::Nop),
        (Opcode::Not, Opcode::JumpIfFalse(label)) => (Opcode::JumpIfTrue(label), Opcode::Nop),
        /* (Opcode::Call(n), Opcode::Return) => (Opcode::TailCall(n), Opcode::Nop), */
        (a, b) => (a, b),
    }
}

/// Performs peephole optimization
pub fn peephole_optimization(mut chunk: Chunk) -> Chunk {
    // If chunk has at least one instruction
    if chunk.code.len() > 0 {
        // Iterating chunk opcodes
        for i in 0..(chunk.code.len() - 1) {
            // Getting first and second opcodes
            let first = chunk.code[i].clone();
            let second = chunk.code[i + 1].clone();

            // Trying to merge
            (chunk.code[i], chunk.code[i + 1]) = try_peephole(first, second)
        }
    }

    // Returning modified chunk
    chunk
}
