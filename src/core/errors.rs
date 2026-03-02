// git-sheets: Core module - fundamental data structures and operations
// A tool for Excel sufferers who deserve better

use csv::Error as CsvError;
use std::fmt;
use std::io;
use toml::de::Error as TomlError;
use toml::ser::Error as TomlSerError;

/// Error type for git-sheets operations
#[derive(Debug)]
pub enum GitSheetsError {
    /// IO error during file operations
    IoError(io::Error),
    /// TOML parsing error
    TomlError(TomlError),
    /// TOML serialization error
    TomlSerError(TomlSerError),
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

impl PartialEq for GitSheetsError {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (GitSheetsError::TomlError(e1), GitSheetsError::TomlError(e2)) => e1 == e2,
            (GitSheetsError::TomlSerError(e1), GitSheetsError::TomlSerError(e2)) => e1 == e2,
            (
                GitSheetsError::DependencyHashMismatch(s1),
                GitSheetsError::DependencyHashMismatch(s2),
            ) => s1 == s2,
            (GitSheetsError::EmptyTable, GitSheetsError::EmptyTable) => true,
            (GitSheetsError::NoPrimaryKey, GitSheetsError::NoPrimaryKey) => true,
            (GitSheetsError::InvalidRowIndex(s1), GitSheetsError::InvalidRowIndex(s2)) => s1 == s2,
            (GitSheetsError::FileSystemError(s1), GitSheetsError::FileSystemError(s2)) => s1 == s2,
            (GitSheetsError::GitError(e1), GitSheetsError::GitError(e2)) => e1 == e2,
            _ => false,
        }
    }
}

impl fmt::Display for GitSheetsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GitSheetsError::IoError(e) => write!(f, "IO Error: {}", e),
            GitSheetsError::TomlError(e) => write!(f, "TOML Error: {}", e),
            GitSheetsError::TomlSerError(e) => write!(f, "TOML Serialization Error: {}", e),
            GitSheetsError::CsvError(e) => write!(f, "CSV Error: {}", e),
            GitSheetsError::GitError(e) => write!(f, "Git Error: {}", e),
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
            GitSheetsError::TomlError(e) => Some(e),
            GitSheetsError::TomlSerError(e) => Some(e),
            GitSheetsError::CsvError(e) => Some(e),
            GitSheetsError::GitError(e) => Some(e),
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

/// Result type for git-sheets operations
pub type Result<T> = std::result::Result<T, GitSheetsError>;
