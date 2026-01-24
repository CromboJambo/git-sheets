// git-sheets: Error types for the library
use csv::Error as CsvError;
use serde_json::Error as JsonError;
use std::io;
use std::path::PathBuf;
use toml::de::Error as TomlError;

/// Custom error types for git-sheets operations
#[derive(Debug)]
pub enum GitSheetsError {
    /// I/O errors (file operations)
    IoError(io::Error),

    /// CSV parsing errors
    CsvError(CsvError),

    /// TOML parsing errors
    TomlError(TomlError),

    /// JSON parsing errors
    JsonError(JsonError),

    /// Invalid primary key specification
    InvalidPrimaryKey(String),

    /// Snapshot file not found
    SnapshotNotFound(PathBuf),

    /// Dependency file not found
    DependencyNotFound(PathBuf),

    /// Integrity check failed
    IntegrityCheckFailed(String),

    /// Hash mismatch for dependency
    DependencyHashMismatch(String),

    /// Invalid file format
    InvalidFileFormat(String),

    /// Empty table
    EmptyTable,

    /// No primary key defined
    NoPrimaryKey,

    /// Invalid column index
    InvalidColumnIndex(String),

    /// Invalid row index
    InvalidRowIndex(String),

    /// Snapshot already exists
    SnapshotAlreadyExists(PathBuf),

    /// Diff computation failed
    DiffComputationFailed(String),
}

impl std::fmt::Display for GitSheetsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GitSheetsError::IoError(e) => write!(f, "I/O error: {}", e),
            GitSheetsError::CsvError(e) => write!(f, "CSV parsing error: {}", e),
            GitSheetsError::TomlError(e) => write!(f, "TOML parsing error: {}", e),
            GitSheetsError::JsonError(e) => write!(f, "JSON parsing error: {}", e),
            GitSheetsError::InvalidPrimaryKey(s) => write!(f, "Invalid primary key: {}", s),
            GitSheetsError::SnapshotNotFound(p) => write!(f, "Snapshot not found: {}", p.display()),
            GitSheetsError::DependencyNotFound(p) => {
                write!(f, "Dependency not found: {}", p.display())
            }
            GitSheetsError::IntegrityCheckFailed(s) => write!(f, "Integrity check failed: {}", s),
            GitSheetsError::DependencyHashMismatch(s) => {
                write!(f, "Dependency hash mismatch: {}", s)
            }
            GitSheetsError::InvalidFileFormat(s) => write!(f, "Invalid file format: {}", s),
            GitSheetsError::EmptyTable => write!(f, "Table is empty"),
            GitSheetsError::NoPrimaryKey => write!(f, "No primary key defined"),
            GitSheetsError::InvalidColumnIndex(s) => write!(f, "Invalid column index: {}", s),
            GitSheetsError::InvalidRowIndex(s) => write!(f, "Invalid row index: {}", s),
            GitSheetsError::SnapshotAlreadyExists(p) => {
                write!(f, "Snapshot already exists: {}", p.display())
            }
            GitSheetsError::DiffComputationFailed(s) => write!(f, "Diff computation failed: {}", s),
        }
    }
}

impl std::error::Error for GitSheetsError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            GitSheetsError::IoError(e) => Some(e),
            GitSheetsError::CsvError(e) => Some(e),
            GitSheetsError::TomlError(e) => Some(e),
            GitSheetsError::JsonError(e) => Some(e),
            _ => None,
        }
    }
}

// Implement From traits for automatic error conversion
impl From<io::Error> for GitSheetsError {
    fn from(err: io::Error) -> Self {
        GitSheetsError::IoError(err)
    }
}

impl From<CsvError> for GitSheetsError {
    fn from(err: CsvError) -> Self {
        GitSheetsError::CsvError(err)
    }
}

impl From<TomlError> for GitSheetsError {
    fn from(err: TomlError) -> Self {
        GitSheetsError::TomlError(err)
    }
}

impl From<JsonError> for GitSheetsError {
    fn from(err: JsonError) -> Self {
        GitSheetsError::JsonError(err)
    }
}

/// Result type alias for git-sheets operations
pub type Result<T> = std::result::Result<T, GitSheetsError>;
