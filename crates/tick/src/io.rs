/// Imports
use camino::Utf8PathBuf;
use tick_common::{
    bail,
    io::{IO, IOError},
};
use std::{
    fs,
    io::{self, Write},
};

/// Cli IO implementation
pub struct CliIO;
impl IO for CliIO {
    // Input implementation
    fn input(&self) -> String {
        let mut line = String::new();
        let _ = io::stdin().read_line(&mut line);
        line
    }

    // Output implementation
    fn output(&self, text: &str) {
        print!("{text}");
    }

    // Read implementation
    fn read(&self, path: &Utf8PathBuf) -> String {
        // Reading module
        match fs::read_to_string(&path) {
            Ok(text) => text,
            Err(_) => bail!(IOError::FileNotFound(path.clone())),
        }
    }

    // Resolve implementation
    fn resolve(&self, path: &str) -> Option<Utf8PathBuf> {
        // Retrieving current directory
        match std::env::current_dir() {
            // Note: from_path_buf with reference is not implemented.
            Ok(cwd) => match Utf8PathBuf::from_path_buf(cwd.clone()) {
                Ok(mut dir) => {
                    // Appending path to cwd
                    dir.push(Utf8PathBuf::from(format!("{path}.tick")));
                    // If path exists
                    if dir.exists() {
                        Some(dir)
                    } // If not
                    else {
                        None
                    }
                }
                Err(_) => bail!(IOError::NonUtf8Path(cwd)),
            },
            Err(err) => bail!(IOError::CwdNotAvailable(err)),
        }
    }

    // Flushes stream
    fn flush(&self) {
        let _ = io::stdout().flush();
    }
}
