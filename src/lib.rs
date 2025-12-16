// git-sheets: Version control for spreadsheets
// A tool for Excel sufferers who deserve better

use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sha2::{Sha256, Digest};

// ============================================================================
// CORE PRIMITIVES
// ============================================================================

/// A snapshot represents the complete state of a table at a point in time
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Snapshot {
    /// Unique identifier for this snapshot
    pub id: String,
    /// When this snapshot was taken
    pub timestamp: DateTime<Utc>,
    /// User-provided message explaining the snapshot
    pub message: Option<String>,
    /// The table data
    pub table: Table,
    /// Hashes for integrity verification
    pub hashes: TableHashes,
    /// Dependencies on other tables/files
    pub dependencies: Vec<Dependency>,
}

/// A table is just headers + rows, nothing fancy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Table {
    /// Column names (the primary key lives here)
    pub headers: Vec<String>,
    /// Raw row data
    pub rows: Vec<Vec<String>>,
    /// Optional: which column(s) form the primary key
    pub primary_key: Option<Vec<usize>>,
}

/// Hashes for verifying table integrity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableHashes {
    /// Hash of the entire table (quick integrity check)
    pub table_hash: String,
    /// Per-header hashes (column-level verification)
    pub header_hashes: HashMap<String, String>,
    /// Optional: per-row hashes (fine-grained verification)
    pub row_hashes: Option<Vec<String>>,
}

/// A dependency represents a reference to another table or file
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dependency {
    /// Name or identifier of the dependency
    pub name: String,
    /// File path if it's external
    pub path: Option<PathBuf>,
    /// Hash of the dependency at snapshot time
    pub hash: String,
}

// ============================================================================
// SNAPSHOT OPERATIONS
// ============================================================================

impl Snapshot {
    /// Create a new snapshot from a table
    pub fn new(table: Table, message: Option<String>) -> Self {
        let hashes = TableHashes::compute(&table);
        let id = format!("{}-{}",
            Utc::now().timestamp(),
            &hashes.table_hash[..8]
        );

        Self {
            id,
            timestamp: Utc::now(),
            message,
            table,
            hashes,
            dependencies: Vec::new(),
        }
    }

    /// Add a dependency to this snapshot
    pub fn add_dependency(&mut self, name: String, path: Option<PathBuf>, hash: String) {
        self.dependencies.push(Dependency { name, path, hash });
    }

    /// Save snapshot to disk as TOML
    pub fn save(&self, output_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        let toml_string = toml::to_string_pretty(self)?;
        fs::write(output_path, toml_string)?;
        Ok(())
    }

    /// Load snapshot from disk
    pub fn load(path: &Path) -> Result<Self, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(path)?;
        let snapshot: Snapshot = toml::from_str(&content)?;
        Ok(snapshot)
    }

    /// Verify integrity of this snapshot
    pub fn verify(&self) -> bool {
        let computed = TableHashes::compute(&self.table);
        computed.table_hash == self.hashes.table_hash
    }
}

// ============================================================================
// HASH COMPUTATION
// ============================================================================

impl TableHashes {
    /// Compute all hashes for a table
    pub fn compute(table: &Table) -> Self {
        let mut header_hashes = HashMap::new();

        // Hash each column
        for (idx, header) in table.headers.iter().enumerate() {
            let column_data: Vec<&str> = table.rows
                .iter()
                .map(|row| row.get(idx).map(|s| s.as_str()).unwrap_or(""))
                .collect();

            let hash = Self::hash_column(header, &column_data);
            header_hashes.insert(header.clone(), hash);
        }

        // Hash entire table
        let table_hash = Self::hash_table(&table.headers, &table.rows);

        // Optional: per-row hashes
        let row_hashes = Some(
            table.rows
                .iter()
                .map(|row| Self::hash_row(row))
                .collect()
        );

        Self {
            table_hash,
            header_hashes,
            row_hashes,
        }
    }

    fn hash_column(header: &str, data: &[&str]) -> String {
        let mut hasher = Sha256::new();
        hasher.update(header.as_bytes());
        for value in data {
            hasher.update(value.as_bytes());
        }
        format!("{:x}", hasher.finalize())
    }

    fn hash_row(row: &[String]) -> String {
        let mut hasher = Sha256::new();
        for cell in row {
            hasher.update(cell.as_bytes());
        }
        format!("{:x}", hasher.finalize())
    }

    fn hash_table(headers: &[String], rows: &[Vec<String>]) -> String {
        let mut hasher = Sha256::new();

        // Hash headers
        for h in headers {
            hasher.update(h.as_bytes());
        }

        // Hash all row data
        for row in rows {
            for cell in row {
                hasher.update(cell.as_bytes());
            }
        }

        format!("{:x}", hasher.finalize())
    }
}

// ============================================================================
// TABLE OPERATIONS
// ============================================================================

impl Table {
    /// Create a table from CSV data
    pub fn from_csv(path: &Path) -> Result<Self, Box<dyn std::error::Error>> {
        let mut reader = csv::Reader::from_path(path)?;

        // Get headers
        let headers: Vec<String> = reader
            .headers()?
            .iter()
            .map(|h| h.trim().to_string())
            .collect();

        // Get rows
        let mut rows = Vec::new();
        for result in reader.records() {
            let record = result?;
            let row: Vec<String> = record
                .iter()
                .map(|cell| cell.trim().to_string())
                .collect();
            rows.push(row);
        }

        Ok(Self {
            headers,
            rows,
            primary_key: None,
        })
    }

    /// Set which columns form the primary key
    pub fn set_primary_key(&mut self, column_indices: Vec<usize>) {
        self.primary_key = Some(column_indices);
    }

    /// Get the primary key for a specific row
    pub fn get_row_key(&self, row_idx: usize) -> Option<Vec<String>> {
        let pk_indices = self.primary_key.as_ref()?;
        let row = self.rows.get(row_idx)?;

        Some(
            pk_indices
                .iter()
                .filter_map(|&idx| row.get(idx).cloned())
                .collect()
        )
    }
}

// ============================================================================
// DIFF OPERATIONS
// ============================================================================

/// A diff between two snapshots
#[derive(Debug, Serialize, Deserialize)]
pub struct SnapshotDiff {
    pub from_id: String,
    pub to_id: String,
    pub summary: DiffSummary,
    pub changes: Vec<Change>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiffSummary {
    pub rows_added: usize,
    pub rows_removed: usize,
    pub rows_modified: usize,
    pub columns_added: usize,
    pub columns_removed: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Change {
    RowAdded { index: usize, data: Vec<String> },
    RowRemoved { index: usize, data: Vec<String> },
    CellChanged { row: usize, col: usize, old: String, new: String },
    ColumnAdded { name: String, index: usize },
    ColumnRemoved { name: String, index: usize },
}

impl SnapshotDiff {
    /// Compute diff between two snapshots
    pub fn compute(from: &Snapshot, to: &Snapshot) -> Self {
        let mut changes = Vec::new();
        let mut summary = DiffSummary {
            rows_added: 0,
            rows_removed: 0,
            rows_modified: 0,
            columns_added: 0,
            columns_removed: 0,
        };

        // Compare headers
        let from_headers: std::collections::HashSet<_> = from.table.headers.iter().collect();
        let to_headers: std::collections::HashSet<_> = to.table.headers.iter().collect();

        for (idx, header) in to.table.headers.iter().enumerate() {
            if !from_headers.contains(header) {
                changes.push(Change::ColumnAdded {
                    name: header.clone(),
                    index: idx
                });
                summary.columns_added += 1;
            }
        }

        for (idx, header) in from.table.headers.iter().enumerate() {
            if !to_headers.contains(header) {
                changes.push(Change::ColumnRemoved {
                    name: header.clone(),
                    index: idx
                });
                summary.columns_removed += 1;
            }
        }

        // Simple row comparison (could be smarter with primary keys)
        let max_rows = from.table.rows.len().max(to.table.rows.len());

        for i in 0..max_rows {
            match (from.table.rows.get(i), to.table.rows.get(i)) {
                (None, Some(row)) => {
                    changes.push(Change::RowAdded {
                        index: i,
                        data: row.clone()
                    });
                    summary.rows_added += 1;
                }
                (Some(row), None) => {
                    changes.push(Change::RowRemoved {
                        index: i,
                        data: row.clone()
                    });
                    summary.rows_removed += 1;
                }
                (Some(from_row), Some(to_row)) => {
                    if from_row != to_row {
                        summary.rows_modified += 1;
                        // Find specific cell changes
                        for (col, (old, new)) in from_row.iter().zip(to_row.iter()).enumerate() {
                            if old != new {
                                changes.push(Change::CellChanged {
                                    row: i,
                                    col,
                                    old: old.clone(),
                                    new: new.clone(),
                                });
                            }
                        }
                    }
                }
                (None, None) => unreachable!(),
            }
        }

        Self {
            from_id: from.id.clone(),
            to_id: to.id.clone(),
            summary,
            changes,
        }
    }

    /// Save diff to disk
    pub fn save(&self, path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        let json = serde_json::to_string_pretty(self)?;
        fs::write(path, json)?;
        Ok(())
    }
}

// ============================================================================
// CLI INTERFACE (example usage)
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_snapshot_creation() {
        let table = Table {
            headers: vec!["ID".to_string(), "Name".to_string(), "Amount".to_string()],
            rows: vec![
                vec!["1".to_string(), "Alice".to_string(), "100".to_string()],
                vec!["2".to_string(), "Bob".to_string(), "200".to_string()],
            ],
            primary_key: Some(vec![0]),
        };

        let snapshot = Snapshot::new(table, Some("Initial snapshot".to_string()));

        assert!(snapshot.verify());
        assert_eq!(snapshot.table.headers.len(), 3);
        assert_eq!(snapshot.table.rows.len(), 2);
    }

    #[test]
    fn test_hash_consistency() {
        let table = Table {
            headers: vec!["A".to_string(), "B".to_string()],
            rows: vec![
                vec!["1".to_string(), "2".to_string()],
            ],
            primary_key: None,
        };

        let hash1 = TableHashes::compute(&table);
        let hash2 = TableHashes::compute(&table);

        assert_eq!(hash1.table_hash, hash2.table_hash);
    }
}

// ============================================================================
// USAGE NOTES
// ============================================================================

/*
Example usage:

// 1. Load a CSV
let table = Table::from_csv(Path::new("sales.csv"))?;

// 2. Create a snapshot
let mut snapshot = Snapshot::new(table, Some("Initial import".to_string()));

// 3. Add dependencies if needed
snapshot.add_dependency(
    "customers.csv".to_string(),
    Some(PathBuf::from("../data/customers.csv")),
    "abc123...".to_string()
);

// 4. Save snapshot
snapshot.save(Path::new("snapshots/sales_001.toml"))?;

// 5. Later: verify integrity
let loaded = Snapshot::load(Path::new("snapshots/sales_001.toml"))?;
assert!(loaded.verify());

// 6. Compare two snapshots
let old_snapshot = Snapshot::load(Path::new("snapshots/sales_001.toml"))?;
let new_snapshot = Snapshot::load(Path::new("snapshots/sales_002.toml"))?;
let diff = SnapshotDiff::compute(&old_snapshot, &new_snapshot);
diff.save(Path::new("diffs/sales_001_to_002.json"))?;

*/
