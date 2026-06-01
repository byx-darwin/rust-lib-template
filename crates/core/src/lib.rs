//! {{ project-name }} core library.
//!
//! Provides domain types, error handling, and core business logic.

#![forbid(unsafe_code)]
#![warn(missing_docs, missing_debug_implementations)]

use std::path::{Path, PathBuf};
use thiserror::Error;

/// Application error type.
///
/// Library functions return this error type. Use `thiserror` for
/// derive macros and proper error context.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum CoreError {
    /// An I/O operation failed.
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),

    /// A serialization or deserialization failure.
    #[error("serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    /// An application-level error with a custom message.
    #[error("application error: {0}")]
    App(String),

    /// A path traversal or path validation failure.
    #[error("path error: {0}")]
    Path(String),
}

/// Core result type alias.
pub type Result<T> = std::result::Result<T, CoreError>;

/// Example domain struct demonstrating CLAUDE.md conventions.
///
/// Implements `Debug`, uses `typed-builder` style for structs
/// with many fields, and validates input at construction.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct Config {
    /// Configuration name, non-empty.
    pub name: String,
    /// Optional description.
    pub description: Option<String>,
}

impl Config {
    /// Creates a new `Config` with the given name.
    ///
    /// # Errors
    ///
    /// Returns [`CoreError::App`] if `name` is empty.
    pub fn new(name: impl Into<String>) -> Result<Self> {
        let name = name.into();
        if name.is_empty() {
            return Err(CoreError::App("config name must not be empty".into()));
        }
        Ok(Self {
            name,
            description: None,
        })
    }

    /// Sets the description and returns the updated config.
    #[must_use]
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }
}

/// A validated, safe filesystem path.
///
/// Rejects `..` components, absolute paths, and null bytes.
/// Use this for all externally-supplied file path arguments.
#[derive(Debug, Clone)]
pub struct SafePath(PathBuf);

impl SafePath {
    /// Creates a new `SafePath` from a relative path string.
    ///
    /// # Errors
    ///
    /// Returns [`CoreError::Path`] if:
    /// - The path contains `..` components
    /// - The path is absolute
    /// - The path contains null bytes
    pub fn new(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref();
        if path.is_absolute() {
            return Err(CoreError::Path("absolute paths are not allowed".into()));
        }
        if path.components().any(|c| c == std::path::Component::ParentDir) {
            return Err(CoreError::Path("'..' components are not allowed".into()));
        }
        if path.as_os_str().as_encoded_bytes().contains(&b'\0') {
            return Err(CoreError::Path("null bytes are not allowed".into()));
        }
        Ok(Self(path.to_path_buf()))
    }

    /// Returns the inner path.
    #[must_use]
    pub fn as_path(&self) -> &Path {
        &self.0
    }
}

impl AsRef<Path> for SafePath {
    fn as_ref(&self) -> &Path {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_new_with_valid_name() -> Result<()> {
        let config = Config::new("test")?;
        assert_eq!(config.name, "test");
        assert!(config.description.is_none());
        Ok(())
    }

    #[test]
    fn test_config_new_with_empty_name_errors() {
        let err = Config::new("");
        assert!(matches!(err, Err(CoreError::App(_))));
    }

    #[test]
    fn test_config_with_description() -> Result<()> {
        let config = Config::new("test")?.with_description("a test config");
        assert_eq!(config.description.as_deref(), Some("a test config"));
        Ok(())
    }

    #[test]
    fn test_io_error_conversion() {
        let io_err = std::io::Error::other("test");
        let core_err = CoreError::from(io_err);
        assert!(matches!(core_err, CoreError::Io(_)));
    }

    #[test]
    fn test_serialization_error_conversion() {
        let io_err = std::io::Error::other("invalid json");
        let serde_err = serde_json::Error::io(io_err);
        let core_err = CoreError::from(serde_err);
        assert!(matches!(core_err, CoreError::Serialization(_)));
    }

    #[test]
    fn test_safe_path_accepts_relative() -> Result<()> {
        let sp = SafePath::new("foo/bar.txt")?;
        assert_eq!(sp.as_path(), Path::new("foo/bar.txt"));
        Ok(())
    }

    #[test]
    fn test_safe_path_rejects_absolute() {
        assert!(matches!(SafePath::new("/etc/passwd"), Err(CoreError::Path(_))));
    }

    #[test]
    fn test_safe_path_rejects_parent_dir() {
        assert!(matches!(SafePath::new("../secret.txt"), Err(CoreError::Path(_))));
    }

    #[test]
    fn test_safe_path_rejects_null_byte() {
        assert!(matches!(SafePath::new("foo\0bar.txt"), Err(CoreError::Path(_))));
    }
}
