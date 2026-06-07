use std::{cell::RefCell, collections::HashMap, sync::Arc, time::Instant};

use camino::Utf8PathBuf;
use common::{io::IO, span::Span};
use miette::NamedSource;

use crate::{
    ModulesRegistry, VirtualMachine,
    frame::Scope,
    ops::{Chunk, Opcode},
    refs::{MutRef, Ref},
    value::{Callable, Native, Value},
};

struct TestIO;
impl IO for TestIO {
    fn input(&self) -> String {
        todo!()
    }

    fn output(&self, text: &str) {
        todo!()
    }

    fn canonicalize(&self, path: &Utf8PathBuf) -> Option<Utf8PathBuf> {
        todo!()
    }

    fn read(&self, path: &Utf8PathBuf) -> String {
        todo!()
    }

    fn write(&self, path: &Utf8PathBuf, text: String) {
        todo!()
    }

    fn cwd(&self) -> Option<Utf8PathBuf> {
        todo!()
    }

    fn flush(&self) {
        todo!()
    }
}

struct TestModules;
impl ModulesRegistry for TestModules {
    fn resolve(&self, id: &str) -> crate::refs::Ref<crate::ops::Chunk> {
        todo!()
    }

    fn insert(&mut self, id: &str, module: crate::refs::MutRef<crate::value::Module>) {
        todo!()
    }
}

#[test]
pub fn test() {
    let io = TestIO;
    let mut modules = TestModules;
    let mut vm = VirtualMachine::new(
        &io,
        &mut modules,
        MutRef::new(RefCell::new(Scope {
            enclosing: None,
            variables: HashMap::from([(
                "putln".to_string(),
                Value::Callable(Callable::Native(Ref::new(Native {
                    arity: 1,
                    function: Box::new(|_, _, args| {
                        println!("{:?}", args.last().unwrap());
                        Value::Null
                    }),
                }))),
            )]),
        })),
    );
    let source = Arc::new(NamedSource::new("a", "b".into()));
    let span = Span(source, 0..0);
    let time = Instant::now();
    vm.push(Ref::new(Chunk {
        code: vec![
            // a = 0
            Opcode::Push(Value::Int(0)),
            Opcode::Define("a".into()),
            // b = 1
            Opcode::Push(Value::Int(1)),
            Opcode::Define("b".into()),
            // i = 2
            Opcode::Push(Value::Int(2)),
            Opcode::Define("i".into()),
            // loop_start = 6
            Opcode::Load("i".into()),
            Opcode::Push(Value::Int(70)),
            Opcode::Le,
            Opcode::JumpIf(10), // body
            Opcode::Jump(23),   // end
            // body = 11

            // c = a + b
            Opcode::Load("a".into()),
            Opcode::Load("b".into()),
            Opcode::Add,
            Opcode::Define("c".into()),
            // a = b
            Opcode::Load("b".into()),
            Opcode::Store("a".into()),
            // b = c
            Opcode::Load("c".into()),
            Opcode::Store("b".into()),
            // i = i + 1
            Opcode::Load("i".into()),
            Opcode::Push(Value::Int(1)),
            Opcode::Add,
            Opcode::Store("i".into()),
            // jump to loop_start
            Opcode::Jump(5),
            // end = 27
            Opcode::Load("putln".into()),
            Opcode::Load("b".into()),
            Opcode::Call(1),
            Opcode::Halt,
        ],
        source_map: vec![
            span.clone(),
            span.clone(),
            span.clone(),
            span.clone(),
            span.clone(),
            span.clone(),
            span.clone(),
            span.clone(),
            span.clone(),
            span.clone(),
            span.clone(),
            span.clone(),
            span.clone(),
            span.clone(),
            span.clone(),
            span.clone(),
            span.clone(),
            span.clone(),
            span.clone(),
            span.clone(),
            span.clone(),
            span.clone(),
            span.clone(),
            span.clone(),
            span.clone(),
            span.clone(),
            span.clone(),
            span.clone(),
        ],
    }));
    vm.exec();
    let time = Instant::now() - time;
    println!("{time:?}")
}
