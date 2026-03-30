/// Modules
mod io;

/// Imports
use crate::io::WasmIO;
use squirrel_rt::interpreter::Interpreter;
use wasm_bindgen::prelude::*;

/// Runs script
#[wasm_bindgen]
pub fn run(code: &str) {
    let io = WasmIO;
    let mut interpreter = Interpreter::new(&io);
    interpreter.interpret_module("wasm", code);
}
