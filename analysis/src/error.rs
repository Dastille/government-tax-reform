//! Error type for rendering tables and documents (no external dependencies).

use std::fmt;

/// Everything that can go wrong while rendering or writing generated files.
#[derive(Debug)]
pub enum Error {
    /// Writing to a `String` failed (only possible if a `Display` impl errors).
    Fmt(fmt::Error),
    /// Reading or writing a generated file failed.
    Io(std::io::Error),
    /// A template used `{{name}}` but the model provides no value called `name`.
    UnknownPlaceholder(String),
    /// A template opened `{{` at this byte offset without a closing `}}`.
    UnclosedPlaceholder(usize),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Fmt(e) => write!(f, "formatting failed: {e}"),
            Error::Io(e) => write!(f, "file I/O failed: {e}"),
            Error::UnknownPlaceholder(k) => write!(f, "unknown template placeholder {{{{{k}}}}}"),
            Error::UnclosedPlaceholder(at) => write!(f, "unclosed '{{{{' at byte {at}"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Fmt(e) => Some(e),
            Error::Io(e) => Some(e),
            Error::UnknownPlaceholder(_) | Error::UnclosedPlaceholder(_) => None,
        }
    }
}

impl From<fmt::Error> for Error {
    fn from(e: fmt::Error) -> Self {
        Error::Fmt(e)
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::Io(e)
    }
}

/// Result alias used across the crate.
pub type Result<T> = std::result::Result<T, Error>;
