/// Modules
mod io;
mod modules;

/// Imports
use crate::{io::CliIO, modules::CliModuleRegistry};
use camino::Utf8PathBuf;
use clap::Parser;
use common::io::IO;
use geko_core::{Flags, run};
use geko_std::builtins;
use miette::NamedSource;
use std::sync::Arc;

/// Arguments parser
#[derive(Parser, Debug)]
#[command(version = concat!("🦎  ", env!("CARGO_PKG_VERSION")), about, long_about = None)]
struct Args {
    /// Path to the file
    path: Utf8PathBuf,

    /// Dump the AST flag
    #[arg(long)]
    dump_ast: bool,

    /// Dump bytecode flag
    #[arg(long)]
    dump_bytecode: bool,
}

/// Prepares miette
fn prepare_miette() {
    let _ = miette::set_hook(Box::new(|_| {
        Box::new(
            miette::MietteHandlerOpts::new()
                .terminal_links(true)
                .unicode(false)
                .rgb_colors(miette::RgbColors::Preferred)
                .show_related_errors_as_nested()
                .context_lines(3)
                .build(),
        )
    }));
}

/// Main
fn main() {
    // Preparing miette
    prepare_miette();

    // Parsing arguments
    let args = Args::parse();

    // Getting file path
    let path = args.path;

    // Preparing IO, Modules Registry and Builtins
    let io = CliIO;
    let mut modules = CliModuleRegistry::new(&io);
    let builtins = builtins::provide();

    // Preparing module name
    let name = io.canonicalize(&path).unwrap().to_string();

    // Reading code
    let code = io.read(&path);

    // Preparing flags
    let flags = Flags {
        dump_ast: args.dump_ast,
        dump_bytecode: args.dump_bytecode,
    };

    // Generating and running vm instructions
    let source = Arc::new(NamedSource::new(name, code.clone()));
    run(source, &code, &io, &mut modules, builtins, flags);
}
