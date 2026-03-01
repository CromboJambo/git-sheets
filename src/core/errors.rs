// git-sheets: Core module - fundamental data structures and operations
// A tool for Excel sufferers who deserve better

use std::fmt;
use std::io;

/// Error type for git-sheets operations
#[derive(Debug, Clone, PartialEq)]
pub enum GitSheetsError {
    /// IO error during file operations
    IoError(io::Error),
    /// Parsing error when reading files
    ParseError(serde_json::Error),
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
            GitSheetsError::IoError(e) => write!(f, "IO Error: {}", e),
            GitSheetsError::ParseError(e) => write!(f, "Parse Error: {}", e),
            GitSheetsError::DependencyHashMismatch(msg) => {
                write!(f, "Dependency Hash Mismatch: {}", msg)
            }
            GitSheetsError::EmptyTable => write!(f, "Empty Table"),
            GitSheetsError::NoPrimaryKey => write!(f, "No Primary Key"),
            GitSheetsError::InvalidRowIndex(msg) => write!(f, "Invalid Row Index: {}", msg),
            GitSheetsError::FileSystemError(msg) => write!(f, "File System Error: {}", msg),
        }
    }
}

impl std::error::Error for GitSheetsError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            GitSheetsError::IoError(e) => Some(e),
            GitSheetsError::ParseError(e) => Some(e),
            GitSheetsError::DependencyHashMismatch(_) => None,
            GitSheetsError::EmptyTable => None,
            GitSheetsError::NoPrimaryKey => None,
            GitSheetsError::InvalidRowIndex(_) => None,
            GitSheetsError::FileSystemError(_) => None,
        }
    }
}

impl From<io::Error> for GitSheetsError {
    fn from(error: io::Error) -> Self {
        GitSheetsError::IoError(error)
    }
}

impl From<serde_json::Error> for GitSheetsError {
    fn from(error: serde_json::Error) -> Self {
        GitSheetsError::ParseError(error)
    }
}

/// Result type for git-sheets operations
pub type Result<T> = std::result::Result<T, GitSheetsError>;
