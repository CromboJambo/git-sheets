// git-sheets: Diff module - computing differences between snapshots
// A tool for Excel sufferers who deserve better

use crate::core::GitSheetsError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

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

        // Compare rows using primary key-based identification
        let from_rows = &from.table.rows;
        let to_rows = &to.table.rows;

        // Create lookup maps for rows by primary key
        let mut from_row_lookup: HashMap<Vec<String>, usize> = HashMap::new();
        let mut to_row_lookup: HashMap<Vec<String>, usize> = HashMap::new();

        if let Some(pk_indices) = &from.table.primary_key {
            for (idx, row) in from_rows.iter().enumerate() {
                let pk_values: Vec<String> = pk_indices
                    .iter()
                    .filter_map(|&i| row.get(i).cloned())
                    .collect();
                if !pk_values.is_empty() {
                    from_row_lookup.insert(pk_values, idx);
                }
            }
        }

        if let Some(pk_indices) = &to.table.primary_key {
            for (idx, row) in to_rows.iter().enumerate() {
                let pk_values: Vec<String> = pk_indices
                    .iter()
                    .filter_map(|&i| row.get(i).cloned())
                    .collect();
                if !pk_values.is_empty() {
                    to_row_lookup.insert(pk_values, idx);
                }
            }
        }

        // Check for added rows (rows not in from but in to)
        let mut added_rows = Vec::new();
        for (pk_values, to_idx) in &to_row_lookup {
            if !from_row_lookup.contains_key(pk_values) {
                added_rows.push((to_idx, to_rows[*to_idx].clone()));
            }
        }

        // Check for removed rows (rows not in to but in from)
        let mut removed_rows = Vec::new();
        for (pk_values, from_idx) in &from_row_lookup {
            if !to_row_lookup.contains_key(pk_values) {
                removed_rows.push((from_idx, from_rows[*from_idx].clone()));
            }
        }

        // Check for modified rows (rows with same primary key but different content)
        let mut modified_rows = Vec::new();
        for (pk_values, from_idx) in &from_row_lookup {
            if let Some(to_idx) = to_row_lookup.get(pk_values) {
                if from_rows[*from_idx] != to_rows[*to_idx] {
                    modified_rows.push((from_idx, to_idx));
                }
            }
        }

        // Add added rows
        for (index, data) in added_rows {
            changes.push(Change::RowAdded {
                index: *index,
                data,
            });
            summary.rows_added += 1;
        }

        // Add removed rows
        for (index, data) in removed_rows {
            changes.push(Change::RowRemoved {
                index: *index,
                data,
            });
            summary.rows_removed += 1;
        }

        // Add modified rows - but avoid double-counting by only adding row modification
        // if there are no other changes for this row (cell changes would be handled separately)
        for (from_idx, to_idx) in modified_rows {
            // Check if this row has cell-level changes
            let mut has_cell_changes = false;
            for (col_idx, (from_cell, to_cell)) in from_rows[*from_idx]
                .iter()
                .zip(to_rows[*to_idx].iter())
                .enumerate()
            {
                if from_cell != to_cell {
                    changes.push(Change::CellChanged {
                        row: *from_idx,
                        col: col_idx,
                        old: from_cell.clone(),
                        new: to_cell.clone(),
                    });
                    has_cell_changes = true;
                }
            }

            // Only add RowModified if there are no cell changes (avoid double counting)
            if !has_cell_changes {
                changes.push(Change::RowModified {
                    index: *from_idx,
                    old_data: from_rows[*from_idx].clone(),
                    new_data: to_rows[*to_idx].clone(),
                });
                summary.rows_modified += 1;
            }
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
