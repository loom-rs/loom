/// Modules
mod io;

/// Imports
use crate::io::CliIO;
use camino::Utf8PathBuf;
use clap::Parser;
use geko_common::io::IO;
use geko_rt::interpreter::Interpreter;

/// Arguments parser
#[derive(Parser, Debug)]
#[command(version = concat!("🦎  ", env!("CARGO_PKG_VERSION")), about, long_about = None)]
struct Args {
    /// Path to the file
    path: Utf8PathBuf,
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
    let path = Args::parse().path;

    // Preparing IO
    let io = CliIO;

    // Preparing module name
    let name = io.canonicalize(&path).unwrap().to_string();

    // Interpreting
    let code = io.read(&path);
    let mut interpreter = Interpreter::new(&io);
    let _ = interpreter.interpret_module(&name, &code);
}
