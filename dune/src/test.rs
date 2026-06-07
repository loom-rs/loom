use common::io::IO;

use crate::{
    ModulesRegistry, VirtualMachine,
    ops::{Chunk, Opcode},
    refs::Ref, value::{Callable, Value},
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
    let mut vm = VirtualMachine::new(&io, &mut modules);
    vm.push(Ref::new(Chunk {
        code: vec![Opcode::Push(Value::Callable(Callable))],
        source_map: vec![],
    }));
    vm.exec();
}
