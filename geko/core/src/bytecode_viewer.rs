use dune::ops::Chunk;

pub fn dump(chunk: &Chunk, level: Option<usize>) {
    for (op, span) in chunk.code.iter().zip(chunk.source_map.iter()) {
        let opcode_name = match op {
            dune::ops::Opcode::Add => "ADD",
            dune::ops::Opcode::Sub => "SUB",
            dune::ops::Opcode::Mul => "MUL",
            dune::ops::Opcode::Div => "DIV",
            dune::ops::Opcode::Neg => "NEG",
            dune::ops::Opcode::Not => "NOT",
            dune::ops::Opcode::And => "AND",
            dune::ops::Opcode::Or => "OR",
            dune::ops::Opcode::Eq => "EQ",
            dune::ops::Opcode::Ne => "NE",
            dune::ops::Opcode::Lt => "LT",
            dune::ops::Opcode::Le => "LE",
            dune::ops::Opcode::Gt => "GT",
            dune::ops::Opcode::Ge => "GE",
            dune::ops::Opcode::Rem => "REM",
            dune::ops::Opcode::Shl => "SHL",
            dune::ops::Opcode::Shr => "SHR",
            dune::ops::Opcode::Jump(x) => &format!("JUMP {x:?}"),
            dune::ops::Opcode::JumpIfTrue(x) => &format!("JUMP_IF_TRUE {x:?}"),
            dune::ops::Opcode::JumpIfFalse(x) => &format!("JUMP_IF_FALSE {x:?}"),
            dune::ops::Opcode::Return => "RETURN",
            dune::ops::Opcode::LoadBuiltin(x) => &format!("LOAD_BUILTIN {x}"),
            dune::ops::Opcode::Load(x) => &format!("LOAD {x}"),
            dune::ops::Opcode::Store(x) => &format!("STORE {x}"),
            dune::ops::Opcode::Define(x) => &format!("DEFINE {x}"),
            dune::ops::Opcode::Call(x) => &format!("CALL {x}"),
            dune::ops::Opcode::LoadField(x) => &format!("LOAD_FIELD {x}"),
            dune::ops::Opcode::StoreField(x) => &format!("STORE_FIELD {x}"),
            dune::ops::Opcode::DefineField(x) => &format!("DEFINE_FIELD {x}"),
            dune::ops::Opcode::MakeClosure(_) => "MAKE_CLOSURE",
            dune::ops::Opcode::MakeClass(x, _) => &format!("MAKE_CLASS {x}"),
            dune::ops::Opcode::MakeTrait(x, _) => &format!("MAKE_TRAIT {x}"),
            dune::ops::Opcode::Import(x) => &format!("IMPORT {x}"),
            dune::ops::Opcode::Nop => "NOP",
            dune::ops::Opcode::Push(value) => &format!("PUSH {value:?}"),
            dune::ops::Opcode::Pop => "POP",
            dune::ops::Opcode::Dup => "DUP",
            dune::ops::Opcode::Swap => "SWAP",
            dune::ops::Opcode::Xor => "XOR",
            dune::ops::Opcode::Band => "BITAND",
            dune::ops::Opcode::Bor => "BITOR",
            dune::ops::Opcode::Impls => "IMPLS",
            dune::ops::Opcode::NotImpls => "NOT_IMPLS",
            dune::ops::Opcode::JumpIfGt(label) => &format!("JUMP_IF_GT {label:?}"),
            dune::ops::Opcode::JumpIfGe(label) => &format!("JUMP_IF_GE {label:?}"),
            dune::ops::Opcode::JumpIfLt(label) => &format!("JUMP_IF_LT {label:?}"),
            dune::ops::Opcode::JumpIfLe(label) => &format!("JUMP_IF_LE {label:?}"),
            dune::ops::Opcode::JumpIfEq(label) => &format!("JUMP_IF_EQ {label:?}"),
            dune::ops::Opcode::JumpIfNe(label) => &format!("JUMP_IF_NE {label:?}"),
            dune::ops::Opcode::Load2(x, y) => &format!("LOAD2 {x}, {y}"),
            dune::ops::Opcode::Push2(value, value1) => &format!("PUSH2 {value}, {value1}"),
        };

        println!(
            "{}{:>10} | {opcode_name}",
            if let Some(level) = level {
                " ".repeat(level * 11) + "> "
            } else {
                String::new()
            },
            &format!("{}..{}", span.1.start, span.1.end)
        );

        if let dune::ops::Opcode::MakeClosure(x) = op {
            dump(&x.chunk, Some(level.unwrap_or(0) + 1));
        }
    }

    println!(
        "{}Labels: {:#?}",
        if let Some(level) = level {
            " ".repeat(level * 11) + "> "
        } else {
            String::new()
        },
        chunk.labels
    );
}
