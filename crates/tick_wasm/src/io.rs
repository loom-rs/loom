/// Imports
use camino::Utf8PathBuf;
use tick_common::{
    bail,
    io::{IO, IOError},
};
use wasm_bindgen::prelude::wasm_bindgen;

/// Wasm binds
#[wasm_bindgen]
extern "C" {
    fn js_print(s: &str);
    fn js_input() -> String;
}

/// Wasm IO implementation
pub struct WasmIO;
impl IO for WasmIO {
    // Input implementation
    fn input(&self) -> String {
        js_input()
    }

    // Output implementation
    fn output(&self, text: &str) {
        js_print(text);
    }

    // Read implementation
    fn read(&self, _: &Utf8PathBuf) -> String {
        bail!(IOError::NotSupported("read"));
    }

    // Resolve implementation
    fn resolve(&self, _: &str) -> Option<Utf8PathBuf> {
        None
    }

    // Flush implementation
    fn flush(&self) {
        // needless
    }
}
