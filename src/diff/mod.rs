// git-sheets: Diff module - computing differences between snapshots
// A tool for Excel sufferers who deserve better

use crate::core::GitSheetsError;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

// Re-export from core module
pub use crate::core::{Snapshot, TableHashes};

/// Summary of changes between snapshots
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiffSummary {
    /// Number of rows added
    pub rows_added: usize,
    /// Number of rows removed
    pub rows_removed: usize,
    /// Number of rows modified
    pub rows_modified: usize,
    /// Number of columns added
    pub columns_added: usize,
    /// Number of columns removed
    pub columns_removed: usize,
}

/// Individual change types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Change {
    RowAdded {
        index: usize,
        data: Vec<String>,
    },
    RowRemoved {
        index: usize,
        data: Vec<String>,
    },
    RowModified {
        index: usize,
        old_data: Vec<String>,
        new_data: Vec<String>,
    },
    CellChanged {
        row: usize,
        col: usize,
        old: String,
        new: String,
    },
    ColumnAdded {
        name: String,
        index: usize,
    },
    ColumnRemoved {
        name: String,
        index: usize,
    },
}

/// A diff between two snapshots
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnapshotDiff {
    /// ID of the from snapshot
    pub from_id: String,
    /// ID of the to snapshot
    pub to_id: String,
    /// Summary of changes
    pub summary: DiffSummary,
    /// Detailed changes
    pub changes: Vec<Change>,
}

impl SnapshotDiff {
    /// Create a diff between two snapshots
    pub fn compute(from: &Snapshot, to: &Snapshot) -> Result<Self, GitSheetsError> {
        let mut changes = Vec::new();
        let mut summary = DiffSummary {
            rows_added: 0,
            rows_removed: 0,
            rows_modified: 0,
            columns_added: 0,
            columns_removed: 0,
        };

        // Compare headers (columns)
        let from_headers = &from.table.headers;
        let to_headers = &to.table.headers;

        // Check for added columns
        for (idx, header) in to_headers.iter().enumerate() {
            if !from_headers.contains(header) {
                changes.push(Change::ColumnAdded {
                    name: header.clone(),
                    index: idx,
                });
                summary.columns_added += 1;
            }
        }

        // Check for removed columns
        for (idx, header) in from_headers.iter().enumerate() {
            if !to_headers.contains(header) {
                changes.push(Change::ColumnRemoved {
                    name: header.clone(),
                    index: idx,
                });
                summary.columns_removed += 1;
            }
        }

        // Compare rows
        let from_rows = &from.table.rows;
        let to_rows = &to.table.rows;

        // Check for added rows
        for (idx, row) in to_rows.iter().enumerate() {
            if !from_rows.contains(row) {
                changes.push(Change::RowAdded {
                    index: idx,
                    data: row.clone(),
                });
                summary.rows_added += 1;
            }
        }

        // Check for removed rows
        for (idx, row) in from_rows.iter().enumerate() {
            if !to_rows.contains(row) {
                changes.push(Change::RowRemoved {
                    index: idx,
                    data: row.clone(),
                });
                summary.rows_removed += 1;
            }
        }

        // Check for modified rows
        let mut row_idx = 0;
        while row_idx < from_rows.len() && row_idx < to_rows.len() {
            let from_row = &from_rows[row_idx];
            let to_row = &to_rows[row_idx];

            if from_row != to_row {
                changes.push(Change::RowModified {
                    index: row_idx,
                    old_data: from_row.clone(),
                    new_data: to_row.clone(),
                });
                summary.rows_modified += 1;
            }

            row_idx += 1;
        }

        // Check for cell changes
        let mut row_idx = 0;
        while row_idx < from_rows.len() && row_idx < to_rows.len() {
            let from_row = &from_rows[row_idx];
            let to_row = &to_rows[row_idx];

            if from_row != to_row {
                for (col_idx, (from_cell, to_cell)) in
                    from_row.iter().zip(to_row.iter()).enumerate()
                {
                    if from_cell != to_cell {
                        changes.push(Change::CellChanged {
                            row: row_idx,
                            col: col_idx,
                            old: from_cell.clone(),
                            new: to_cell.clone(),
                        });
                    }
                }
            }

            row_idx += 1;
        }

        Ok(Self {
            from_id: from.id.clone(),
            to_id: to.id.clone(),
            summary,
            changes,
        })
    }

    /// Save diff to disk as TOML
    pub fn save(&self, path: &Path) -> Result<(), GitSheetsError> {
        let toml_string = toml::to_string_pretty(self)?;
        fs::write(path, toml_string)?;
        Ok(())
    }
}
