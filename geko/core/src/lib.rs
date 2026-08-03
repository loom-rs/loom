/// Modules
pub mod codegen;
pub mod lex;
pub mod parse;
pub mod sema;

/// Imports
use crate::{codegen::CodeGenerator, lex::Lexer, parse::Parser, sema::Analyzer};
use common::{io::IO, warn};
use dune::{
    ModuleRegistry, VirtualMachine,
    frame::Scope,
    ops::Chunk,
    refs::{MutRef, Ref},
};
use miette::NamedSource;
use std::sync::Arc;

/// Geko flags used during emit or run
#[derive(Default, Clone, Copy)]
pub struct Flags {
    /// Dump the AST
    pub dump_ast: bool,

    /// Dump bytecode
    pub dump_bytecode: bool,

    /// Measure execution time?
    pub measure_exec_time: bool,

    /// Drop optimizations?
    pub drop_optimizations: bool,
}

/// Compiles module and returns reference to chunk
pub fn emit(source: Arc<NamedSource<String>>, code: &str, flags: Flags) -> Ref<Chunk> {
    // Parsing program
    let lexer = Lexer::new(source.clone(), code);
    let mut parser = Parser::new(source, lexer);
    let program = parser.program();

    // Dumping ast if needed
    if flags.dump_ast {
        println!("{program:#?}");
    }

    // Analyzing program
    let mut sema = Analyzer::default();
    sema.analyze_module(&program);

    // Generating vm instructions
    let mut generator = CodeGenerator::new(!flags.drop_optimizations);
    let chunk = generator.gen_program(program);

    // Dumping bytecode if needed
    if flags.dump_bytecode {
        println!("{chunk:#?}");
    }

    // If measure exec time is true throw a warning
    if flags.measure_exec_time {
        warn!("could not measure executiong time in `emit()`")
    }

    chunk
}

/// Compiles and runs module
pub fn run(
    source: Arc<NamedSource<String>>,
    code: &str,
    io: &dyn IO,
    modules: &mut dyn ModuleRegistry,
    builtins: MutRef<Scope>,
    flags: Flags,
) {
    // Parsing program
    let lexer = Lexer::new(source.clone(), code);
    let mut parser = Parser::new(source, lexer);
    let program = parser.program();

    // Dumping ast if needed
    if flags.dump_ast {
        println!("{program:#?}");
    }

    // Analyzing program
    let mut sema = Analyzer::default();
    sema.analyze_module(&program);

    // Generating vm instructions
    let mut generator = CodeGenerator::new(!flags.drop_optimizations);
    let chunk = generator.gen_program(program);

    // Dumping bytecode if needed
    if flags.dump_bytecode {
        println!("{chunk:#?}");
    }

    // Preparing VM
    let mut vm = VirtualMachine::new(io, modules, builtins);
    vm.push(chunk);

    // Executing instructions
    if flags.measure_exec_time {
        let start = std::time::Instant::now();
        vm.exec();
        let duration = start.elapsed();
        println!("execution time: {duration:?}")
    } else {
        vm.exec();
    }
}
