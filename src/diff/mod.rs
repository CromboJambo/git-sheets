// git-sheets: Diff module - snapshot comparison operations
// A tool for Excel sufferers who deserve better

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

// Re-export from core module
pub use crate::core::Result;
pub use crate::core::Snapshot;

// ============================================================================
// DIFF TYPES
// ============================================================================

/// A diff between two snapshots
#[derive(Debug, Serialize, Deserialize)]
pub struct SnapshotDiff {
    pub from_id: String,
    pub to_id: String,
    pub summary: DiffSummary,
    pub changes: Vec<Change>,
}

/// Summary of changes between snapshots
#[derive(Debug, Serialize, Deserialize)]
pub struct DiffSummary {
    pub rows_added: usize,
    pub rows_removed: usize,
    pub rows_modified: usize,
    pub columns_added: usize,
    pub columns_removed: usize,
}

/// Individual change types
#[derive(Debug, Serialize, Deserialize)]
pub enum Change {
    RowAdded {
        index: usize,
        data: Vec<String>,
    },
    RowRemoved {
        index: usize,
        data: Vec<String>,
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

// ============================================================================
// DIFF OPERATIONS
// ============================================================================

impl SnapshotDiff {
    /// Create a map from primary key values to row indices
    fn create_pk_map(snapshot: &Snapshot) -> HashMap<Vec<String>, Vec<usize>> {
        let mut map = HashMap::new();
        let pk_indices = match &snapshot.table.primary_key {
            Some(indices) => indices,
            None => return map,
        };

        for (row_idx, row) in snapshot.table.rows.iter().enumerate() {
            let pk_values: Vec<String> = pk_indices
                .iter()
                .filter_map(|&idx| row.get(idx).cloned())
                .collect();

            if !pk_values.is_empty() {
                map.insert(pk_values, vec![row_idx]);
            }
        }

        map
    }

    /// Create a diff between two snapshots
    pub fn compute(from: &Snapshot, to: &Snapshot) -> Result<Self, crate::core::GitSheetsError> {
        let mut changes = Vec::new();
        let mut summary = DiffSummary {
            rows_added: 0,
            rows_removed: 0,
            rows_modified: 0,
            columns_added: 0,
            columns_removed: 0,
        };

        // Create primary key maps
        let from_map = Self::create_pk_map(from);
        let to_map = Self::create_pk_map(to);

        // Compare rows using primary keys
        for (pk, from_indices) in &from_map {
            match to_map.get(pk) {
                Some(to_indices) => {
                    if from_indices.len() != to_indices.len() {
                        summary.rows_modified += 1;
                        // Find specific cell changes
                        for (from_idx, to_idx) in from_indices.iter().zip(to_indices.iter()) {
                            if from.table.rows[*from_idx] != to.table.rows[to_idx] {
                                // Find specific cell changes
                                for (col, (old, new)) in from.table.rows[*from_idx]
                                    .iter()
                                    .zip(to.table.rows[to_idx].iter())
                                    .enumerate()
                                {
                                    if old != new {
                                        changes.push(Change::CellChanged {
                                            row: *to_idx,
                                            col,
                                            old: old.clone(),
                                            new: new.clone(),
                                        });
                                    }
                                }
                            }
                        }
                    }
                }
                None => {
                    // Rows removed
                    for from_idx in from_indices {
                        summary.rows_removed += 1;
                        changes.push(Change::RowRemoved {
                            index: *from_idx,
                            data: from.table.rows[*from_idx].clone(),
                        });
                    }
                }
            }
        }

        // Check for new rows
        for (pk, to_indices) in &to_map {
            if !from_map.contains_key(pk) {
                // New rows
                for to_idx in to_indices {
                    changes.push(Change::RowAdded {
                        index: *to_idx,
                        data: to.table.rows[*to_idx].clone(),
                    });
                    summary.rows_added += 1;
                }
            }
        }

        Ok(Self {
            from_id: from.id.clone(),
            to_id: to.id.clone(),
            summary,
            changes,
        })
    }

    /// Save diff to disk as JSON
    pub fn save(&self, path: &Path) -> Result<(), crate::core::GitSheetsError> {
        let json = serde_json::to_string_pretty(self)?;
        std::fs::write(path, json)?;
        Ok(())
    }
}
