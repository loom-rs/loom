/// Modules
pub mod codegen;
pub mod lex;
pub mod parse;
pub mod sema;

/// Imports
use crate::{codegen::CodeGenerator, lex::Lexer, parse::Parser, sema::Analyzer};
use common::io::IO;
use dune::{
    ModuleRegistry, VirtualMachine,
    frame::Scope,
    ops::Chunk,
    refs::{MutRef, Ref},
};
use miette::NamedSource;
use std::sync::Arc;

/// Compiles module and returns reference to chunk
pub fn emit(source: Arc<NamedSource<String>>, code: &str) -> Ref<Chunk> {
    // Parsing program
    let lexer = Lexer::new(source.clone(), code);
    let mut parser = Parser::new(source, lexer);
    let program = parser.program();

    // Analyzing program
    let mut sema = Analyzer::default();
    sema.analyze_module(&program);

    // Generating vm instructions
    let mut generator = CodeGenerator::default();
    generator.gen_program(program)
}

/// Compiles and runs module
pub fn run(
    source: Arc<NamedSource<String>>,
    code: &str,
    io: &dyn IO,
    modules: &mut dyn ModuleRegistry,
    builtins: MutRef<Scope>,
) {
    // Parsing program
    let lexer = Lexer::new(source.clone(), code);
    let mut parser = Parser::new(source, lexer);
    let program = parser.program();

    // Analyzing program
    let mut sema = Analyzer::default();
    sema.analyze_module(&program);

    // Generating vm instructions
    let mut generator = CodeGenerator::default();
    let chunk = generator.gen_program(program);

    println!("{:#?}", chunk.code);

    // Running vm instructions
    let mut vm = VirtualMachine::new(io, modules, builtins);
    vm.push(chunk);
    vm.exec();
}
