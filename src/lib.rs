// git-sheets: Version control for spreadsheets
// A tool for Excel sufferers who deserve better

pub mod cli;
pub mod core;
pub mod diff;

// Re-export core types for convenience
pub use core::{Dependency, GitSheetsError, Result, Snapshot, Table, TableHashes};

// Re-export diff types
pub use diff::{Change, DiffSummary, SnapshotDiff};

// Re-export CLI module
pub use cli::Cli;

// Export the main run function
pub use cli::run;
