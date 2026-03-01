// git-sheets: Hash module - pure hash computation logic
// A tool for Excel sufferers who deserve better

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashMap;

// Re-export from core module
pub use crate::core::Table;

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

impl TableHashes {
    /// Compute all hashes for a table
    pub fn compute(table: &Table) -> Self {
        let mut header_hashes = HashMap::new();

        // Hash each column
        for (idx, header) in table.headers.iter().enumerate() {
            let column_data: Vec<&str> = table
                .rows
                .iter()
                .map(|row| row.get(idx).map(|s| s.as_str()).unwrap_or(""))
                .collect();

            let hash = Self::hash_column(header, &column_data);
            header_hashes.insert(header.clone(), hash);
        }

        // Hash entire table
        let table_hash = Self::hash_table(&table.headers, &table.rows);

        // Optional: per-row hashes
        let row_hashes = Some(table.rows.iter().map(|row| Self::hash_row(row)).collect());

        Self {
            table_hash,
            header_hashes,
            row_hashes,
        }
    }

    /// Hash a single column
    fn hash_column(header: &str, data: &[&str]) -> String {
        let mut hasher = Sha256::new();
        hasher.update(header.as_bytes());
        for value in data {
            hasher.update(value.as_bytes());
        }
        format!("{:x}", hasher.finalize())
    }

    /// Hash a single row
    fn hash_row(row: &[String]) -> String {
        let mut hasher = Sha256::new();
        for cell in row {
            hasher.update(cell.as_bytes());
        }
        format!("{:x}", hasher.finalize())
    }

    /// Hash the entire table (headers + all rows)
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
