use std::fmt;

use csv::Error as CsvError;
use serde_json::Error as JsonError;
use toml::de::Error as TomlError;
use toml::ser::Error as TomlSerError;

/// Error type for git-sheets operations
#[derive(Debug)]
pub enum GitSheetsError {
    /// IO error during file operations
    IoError(std::io::Error),
    /// TOML parsing error
    TomlError(TomlError),
    /// TOML serialization error
    TomlSerError(TomlSerError),
    /// JSON parsing error
    JsonError(JsonError),
    /// CSV error
    CsvError(CsvError),
    /// Git error
    GitError(git2::Error),
    /// Dependency hash mismatch
    DependencyHashMismatch(String),
    /// Empty table encountered
    EmptyTable,
    /// No primary key defined
    NoPrimaryKey,
    /// Invalid row index provided
    InvalidRowIndex(String),
    /// File system error
    FileSystemError(String),
}

impl fmt::Display for GitSheetsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GitSheetsError::IoError(e) => write!(f, "IO Error: {e}"),
            GitSheetsError::TomlError(e) => write!(f, "TOML Error: {e}"),
            GitSheetsError::TomlSerError(e) => write!(f, "TOML Serialization Error: {e}"),
            GitSheetsError::JsonError(e) => write!(f, "JSON Error: {e}"),
            GitSheetsError::CsvError(e) => write!(f, "CSV Error: {e}"),
            GitSheetsError::GitError(e) => write!(f, "Git Error: {e}"),
            GitSheetsError::DependencyHashMismatch(msg) => {
                write!(f, "Dependency Hash Mismatch: {msg}")
            }
            GitSheetsError::EmptyTable => write!(f, "Empty Table"),
            GitSheetsError::NoPrimaryKey => write!(f, "No Primary Key"),
            GitSheetsError::InvalidRowIndex(msg) => write!(f, "Invalid Row Index: {msg}"),
            GitSheetsError::FileSystemError(msg) => write!(f, "File System Error: {msg}"),
        }
    }
}

impl std::error::Error for GitSheetsError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            GitSheetsError::IoError(e) => Some(e),
            GitSheetsError::TomlError(e) => Some(e),
            GitSheetsError::TomlSerError(e) => Some(e),
            GitSheetsError::CsvError(e) => Some(e),
            GitSheetsError::GitError(e) => Some(e),
            GitSheetsError::JsonError(e) => Some(e),
            GitSheetsError::DependencyHashMismatch(_)
            | GitSheetsError::EmptyTable
            | GitSheetsError::NoPrimaryKey
            | GitSheetsError::InvalidRowIndex(_)
            | GitSheetsError::FileSystemError(_) => None,
        }
    }
}

impl From<std::io::Error> for GitSheetsError {
    fn from(error: std::io::Error) -> Self {
        GitSheetsError::IoError(error)
    }
}

impl From<TomlError> for GitSheetsError {
    fn from(error: TomlError) -> Self {
        GitSheetsError::TomlError(error)
    }
}

impl From<TomlSerError> for GitSheetsError {
    fn from(error: TomlSerError) -> Self {
        GitSheetsError::TomlSerError(error)
    }
}

impl From<CsvError> for GitSheetsError {
    fn from(error: CsvError) -> Self {
        GitSheetsError::CsvError(error)
    }
}

impl From<git2::Error> for GitSheetsError {
    fn from(error: git2::Error) -> Self {
        GitSheetsError::GitError(error)
    }
}

impl From<JsonError> for GitSheetsError {
    fn from(error: JsonError) -> Self {
        GitSheetsError::JsonError(error)
    }
}

/// Result type for git-sheets operations
pub type Result<T> = std::result::Result<T, GitSheetsError>;
