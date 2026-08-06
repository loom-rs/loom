/// Imports
use dune::{
    ops::{Chunk, Opcode},
    value::Value,
};

/// Tries to fold three instructions into one super instruction
pub fn try_fold(a: Opcode, b: Opcode, c: Opcode) -> Option<Opcode> {
    match (a, b) {
        (Opcode::Push(Value::Int(a)), Opcode::Push(Value::Int(b))) => match c {
            Opcode::Add => Some(Opcode::Push(Value::Int(a + b))),
            Opcode::Sub => Some(Opcode::Push(Value::Int(a - b))),
            Opcode::Mul => Some(Opcode::Push(Value::Int(a * b))),
            Opcode::Div => Some(Opcode::Push(Value::Int(a / b))),
            Opcode::Rem => Some(Opcode::Push(Value::Int(a % b))),
            Opcode::Shl => Some(Opcode::Push(Value::Int(a << b))),
            Opcode::Shr => Some(Opcode::Push(Value::Int(a >> b))),
            Opcode::Band => Some(Opcode::Push(Value::Int(a & b))),
            Opcode::Bor => Some(Opcode::Push(Value::Int(a | b))),
            Opcode::Xor => Some(Opcode::Push(Value::Int(a ^ b))),
            Opcode::Gt => Some(Opcode::Push(Value::Bool(a > b))),
            Opcode::Ge => Some(Opcode::Push(Value::Bool(a >= b))),
            Opcode::Lt => Some(Opcode::Push(Value::Bool(a < b))),
            Opcode::Le => Some(Opcode::Push(Value::Bool(a <= b))),
            Opcode::Eq => Some(Opcode::Push(Value::Bool(a == b))),
            Opcode::Ne => Some(Opcode::Push(Value::Bool(a != b))),
            _ => None,
        },
        (Opcode::Push(Value::Float(a)), Opcode::Push(Value::Float(b))) => match c {
            Opcode::Add => Some(Opcode::Push(Value::Float(a + b))),
            Opcode::Sub => Some(Opcode::Push(Value::Float(a - b))),
            Opcode::Mul => Some(Opcode::Push(Value::Float(a * b))),
            Opcode::Div => Some(Opcode::Push(Value::Float(a / b))),
            Opcode::Rem => Some(Opcode::Push(Value::Float(a % b))),
            Opcode::Gt => Some(Opcode::Push(Value::Bool(a > b))),
            Opcode::Ge => Some(Opcode::Push(Value::Bool(a >= b))),
            Opcode::Lt => Some(Opcode::Push(Value::Bool(a < b))),
            Opcode::Le => Some(Opcode::Push(Value::Bool(a <= b))),
            Opcode::Eq => Some(Opcode::Push(Value::Bool(a == b))),
            Opcode::Ne => Some(Opcode::Push(Value::Bool(a != b))),
            _ => None,
        },
        (Opcode::Push(Value::Int(a)), Opcode::Push(Value::Float(b))) => match c {
            Opcode::Add => Some(Opcode::Push(Value::Float(a as f64 + b))),
            Opcode::Sub => Some(Opcode::Push(Value::Float(a as f64 - b))),
            Opcode::Mul => Some(Opcode::Push(Value::Float(a as f64 * b))),
            Opcode::Div => Some(Opcode::Push(Value::Float(a as f64 / b))),
            Opcode::Rem => Some(Opcode::Push(Value::Float(a as f64 % b))),
            Opcode::Gt => Some(Opcode::Push(Value::Bool(a as f64 > b))),
            Opcode::Ge => Some(Opcode::Push(Value::Bool(a as f64 >= b))),
            Opcode::Lt => Some(Opcode::Push(Value::Bool((a as f64) < b))),
            Opcode::Le => Some(Opcode::Push(Value::Bool(a as f64 <= b))),
            Opcode::Eq => Some(Opcode::Push(Value::Bool(a as f64 == b))),
            Opcode::Ne => Some(Opcode::Push(Value::Bool(a as f64 != b))),
            _ => None,
        },
        (Opcode::Push(Value::Float(a)), Opcode::Push(Value::Int(b))) => match c {
            Opcode::Add => Some(Opcode::Push(Value::Float(a + b as f64))),
            Opcode::Sub => Some(Opcode::Push(Value::Float(a - b as f64))),
            Opcode::Mul => Some(Opcode::Push(Value::Float(a * b as f64))),
            Opcode::Div => Some(Opcode::Push(Value::Float(a / b as f64))),
            Opcode::Rem => Some(Opcode::Push(Value::Float(a % b as f64))),
            Opcode::Gt => Some(Opcode::Push(Value::Bool(a > b as f64))),
            Opcode::Ge => Some(Opcode::Push(Value::Bool(a >= b as f64))),
            Opcode::Lt => Some(Opcode::Push(Value::Bool(a < b as f64))),
            Opcode::Le => Some(Opcode::Push(Value::Bool(a <= b as f64))),
            Opcode::Eq => Some(Opcode::Push(Value::Bool(a == b as f64))),
            Opcode::Ne => Some(Opcode::Push(Value::Bool(a != b as f64))),
            _ => None,
        },
        (_, _) => None,
    }
}

/// Performs fold optimization
pub fn fold_optimization(mut chunk: Chunk) -> Chunk {
    let mut current_len = chunk.code.len();
    let mut optimized_code_len = 0;

    // Run opimizations until it becomes impossible to optimize.
    while current_len != optimized_code_len {
        current_len = optimized_code_len;

        // If chunk has at least two instructions
        if chunk.code.len() > 1 {
            // Iterating chunk opcodes
            for i in 0..(chunk.code.len() - 2) {
                // Getting first and second opcodes
                let first = chunk.code[i].clone();
                let second = chunk.code[i + 1].clone();
                let third = chunk.code[i + 2].clone();

                // Trying to fold
                (chunk.code[i], chunk.code[i + 1], chunk.code[i + 2]) =
                    match try_fold(first.clone(), second.clone(), third.clone()) {
                        Some(op) => (op, Opcode::Nop, Opcode::Nop),
                        None => (first, second, third),
                    }
            }
        }

        // Remove all Nop opcodes.
        chunk.code.retain(|x| !matches!(x, Opcode::Nop));

        optimized_code_len = chunk.code.len();
    }

    // Returning modified chunk
    chunk
}
