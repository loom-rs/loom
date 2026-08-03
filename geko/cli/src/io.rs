/// Imports
use camino::Utf8PathBuf;
use common::{
    bail,
    io::{IOError, IO},
};
use std::{
    env, fs,
    io::{self, Write},
};

/// Cli IO implementation
pub struct CliIO;
impl IO for CliIO {
    /// Input implementation
    fn input(&self) -> String {
        let mut line = String::new();
        let _ = io::stdin().read_line(&mut line);
        line.trim_end().to_string()
    }

    /// Output implementation
    fn output(&self, text: &str) {
        print!("{text}");
    }

    /// Canonicalize implementation
    fn canonicalize(&self, path: &Utf8PathBuf) -> Option<Utf8PathBuf> {
        // If user wants to read from stdin (by providing "-"), keep it as-is.
        if path == "-" {
            return Some(path.clone());
        }

        // Reading file
        match path.canonicalize_utf8() {
            Ok(path) => Some(path),
            Err(_) => bail!(IOError::FileNotFound(path.clone())),
        }
    }

    /// Read implementation
    fn read(&self, path: &Utf8PathBuf) -> String {
        // Read from stdin if path is "-" (which means stdin), read a file otherwise.
        let result = if path == "-" {
            io::read_to_string(io::stdin())
        } else {
            fs::read_to_string(path)
        };

        match result {
            Ok(text) => text,
            Err(_) => bail!(IOError::FileNotFound(path.clone())),
        }
    }

    /// Write implementation
    fn write(&self, path: &Utf8PathBuf, text: String) {
        // Writing to file
        if fs::write(path, text).is_err() {
            bail!(IOError::FileNotFound(path.clone()))
        }
    }

    /// Flushes stream
    fn flush(&self) {
        let _ = io::stdout().flush();
    }

    /// Cwd implementation
    fn cwd(&self) -> Option<Utf8PathBuf> {
        // Matching current directory
        match env::current_dir() {
            // Note: `from_path_buf` is no implemented with reference
            Ok(path) => match Utf8PathBuf::from_path_buf(path.clone()) {
                Ok(path) => Some(path),
                Err(_) => bail!(IOError::NonUtf8Path(path)),
            },
            Err(err) => bail!(IOError::CwdNotAvailable(err)),
        }
    }
}
