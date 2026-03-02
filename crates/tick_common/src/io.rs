/// Imports
use camino::Utf8PathBuf;
use miette::Diagnostic;
use std::{io, path::PathBuf};
use thiserror::Error;

/// IO error
#[derive(Error, Diagnostic, Debug)]
pub enum IOError {
    /// File not found
    #[error("file `{0}` not found")]
    #[diagnostic(code(io::file_not_found))]
    FileNotFound(Utf8PathBuf),
    /// Non-utf8 path
    #[error("invalid utf-8 path `{0}`")]
    #[diagnostic(code(io::non_utf8_path))]
    NonUtf8Path(PathBuf),
    /// Cwd not available
    #[error("failed to get current working directory due io error: {0}")]
    #[diagnostic(code(io::cwd_not_available))]
    CwdNotAvailable(io::Error),
    /// Something not supported
    #[error("`{0}` is not supported due platform limitations")]
    #[diagnostic(code(io::not_supported))]
    NotSupported(&'static str),
}

/// IO trait representation
pub trait IO {
    /// Reads input
    fn input(&self) -> String;

    /// Writes output
    fn output(&self, text: &str);

    /// Reads file
    fn read(&self, path: &Utf8PathBuf) -> String;

    /// Resolves path by inserting `cwd`.
    ///
    /// Returns `None` if path isn't exists or fs isn't available.
    /// Otherwise returns `Some(Utf8PathBuf)`
    ///
    fn resolve(&self, path: &str) -> Option<Utf8PathBuf>;

    /// Flushes stream
    fn flush(&self);
}
