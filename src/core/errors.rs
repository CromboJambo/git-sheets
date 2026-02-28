// git-sheets: Core error types
// A tool for Excel sufferers who deserve better

use csv::Error as CsvError;
use serde_json::Error as JsonError;
use std::io;
use toml::de::Error as TomlDeError;
use toml::ser::Error as TomlSerError;

use std::fmt;

/// Result type for git-sheets operations
pub type Result<T> = std::result::Result<T, GitSheetsError>;

/// Error types for git-sheets operations
#[derive(Debug, Clone)]
pub enum GitSheetsError {
    /// CSV parsing error
    CsvParseError(String),
    /// Empty table error
    EmptyTable,
    /// No primary key error
    NoPrimaryKey,
    /// Invalid row index error
    InvalidRowIndex(String),
    /// Dependency hash mismatch
    DependencyHashMismatch(String),
    /// File system error
    FileSystemError(String),
    /// TOML serialization error
    TomlError(String),
}

impl fmt::Display for GitSheetsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GitSheetsError::CsvParseError(msg) => write!(f, "CSV Parse Error: {}", msg),
            GitSheetsError::EmptyTable => write!(f, "Empty table"),
            GitSheetsError::NoPrimaryKey => write!(f, "No primary key defined"),
            GitSheetsError::InvalidRowIndex(msg) => write!(f, "Invalid row index: {}", msg),
            GitSheetsError::DependencyHashMismatch(msg) => {
                write!(f, "Dependency hash mismatch: {}", msg)
            }
            GitSheetsError::FileSystemError(msg) => write!(f, "File system error: {}", msg),
            GitSheetsError::TomlError(msg) => write!(f, "TOML serialization error: {}", msg),
        }
    }

    impl std::error::Error for GitSheetsError {}
}

impl From<io::Error> for GitSheetsError {
    fn from(error: io::Error) -> Self {
        GitSheetsError::FileSystemError(error.to_string())
    }
}

impl From<TomlSerError> for GitSheetsError {
    fn from(error: TomlSerError) -> Self {
        GitSheetsError::TomlError(error.to_string())
    }
}

impl From<TomlDeError> for GitSheetsError {
    fn from(error: TomlDeError) -> Self {
        GitSheetsError::TomlError(error.to_string())
    }
}

impl From<CsvError> for GitSheetsError {
    fn from(error: CsvError) -> Self {
        GitSheetsError::CsvParseError(error.to_string())
    }
}

impl From<JsonError> for GitSheetsError {
    fn from(error: JsonError) -> Self {
        GitSheetsError::FileSystemError(error.to_string())
    }
}

// Removed duplicate implementation
