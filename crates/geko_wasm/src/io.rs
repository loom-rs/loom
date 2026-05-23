/// Imports
use camino::Utf8PathBuf;
use geko_common::{
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
        js_input().trim_end().to_string()
    }

    // Output implementation
    fn output(&self, text: &str) {
        js_print(text);
    }

    // Canonicalize implementation
    fn canonicalize(&self, _: &Utf8PathBuf) -> Option<Utf8PathBuf> {
        None
    }

    // Read implementation
    fn read(&self, _: &Utf8PathBuf) -> String {
        bail!(IOError::NotSupported("read"));
    }

    // Write implementation
    fn write(&self, _: &Utf8PathBuf, _: String) {
        bail!(IOError::NotSupported("write"));
    }

    // Cwd implementation
    fn cwd(&self) -> Option<Utf8PathBuf> {
        None
    }

    // Flush implementation
    fn flush(&self) {
        // unnecessary
    }
}
