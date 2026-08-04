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
        (Opcode::Load(name), Opcode::Store(name2)) if name == name2 => (Opcode::Nop, Opcode::Nop),
        (Opcode::Push(Value::Int(0)), Opcode::Add) => (Opcode::Nop, Opcode::Nop),
        (Opcode::Push(Value::Int(0)), Opcode::Sub) => (Opcode::Nop, Opcode::Nop),
        (Opcode::Push(Value::Int(0)), Opcode::Mul) => (Opcode::Push(Value::Int(0)), Opcode::Nop),
        (Opcode::Push(Value::Int(1)), Opcode::Mul) => (Opcode::Nop, Opcode::Nop),
        (Opcode::Push(Value::Int(1)), Opcode::Div) => (Opcode::Nop, Opcode::Nop),
        (Opcode::Not, Opcode::JumpIfFalse(label)) => (Opcode::JumpIfTrue(label), Opcode::Nop),
        (Opcode::Neg, Opcode::Neg) | (Opcode::Not, Opcode::Not) => (Opcode::Nop, Opcode::Nop),
        
        // Multiplying and dividing by `-1` is negation.
        (Opcode::Push(Value::Int(-1)), Opcode::Mul) => (Opcode::Neg, Opcode::Nop),
        (Opcode::Push(Value::Int(-1)), Opcode::Div) => (Opcode::Neg, Opcode::Nop),

        // Multiplying a number `x` by number `2^n` can be replaced by `x << n`.
        (Opcode::Push(Value::Int(x)), Opcode::Mul) if x.is_positive() && (x as u64).is_power_of_two() => {
            let power = (x as u64).ilog2();

            (Opcode::Push(Value::Int(power as _)), Opcode::Shl)
        },

        // Dividing a number `x` by number `2^n` can be replaced by `x >> n`.
        (Opcode::Push(Value::Int(x)), Opcode::Div) if x.is_positive() && (x as u64).is_power_of_two() => {
            let power = (x as u64).ilog2();
        
            (Opcode::Push(Value::Int(power as _)), Opcode::Shr)
        },

        (Opcode::Push(Value::Bool(true)), Opcode::And) => (Opcode::Nop, Opcode::Nop),
        (Opcode::Push(Value::Bool(false)), Opcode::And) => {
            (Opcode::Push(Value::Bool(false)), Opcode::Nop)
        }
        (Opcode::Push(Value::Bool(true)), Opcode::Or) => {
            (Opcode::Push(Value::Bool(true)), Opcode::Nop)
        }
        (Opcode::Push(Value::Bool(false)), Opcode::Or) => (Opcode::Nop, Opcode::Nop),
        (Opcode::Dup, Opcode::Pop) => (Opcode::Nop, Opcode::Nop),
        (Opcode::Swap, Opcode::Swap) => (Opcode::Nop, Opcode::Nop),
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
