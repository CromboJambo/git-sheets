// git-sheets: Core module - fundamental data structures and operations
// A tool for Excel sufferers who deserve better

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

// Git integration
use git2;

pub mod errors;
pub use errors::{GitSheetsError, Result};

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

impl TableHashes {
    /// Compute hashes for a table
    pub fn compute(table: &Table) -> Self {
        let mut hasher = Sha256::new();

        // Hash the entire table by concatenating all data
        for header in &table.headers {
            hasher.update(header.as_bytes());
        }
        for row in &table.rows {
            for cell in row {
                hasher.update(cell.as_bytes());
            }
        }

        let table_hash = format!("{:x}", hasher.finalize());

        // Compute per-header hashes
        let mut header_hashes = HashMap::new();
        for (idx, header) in table.headers.iter().enumerate() {
            let mut hasher = Sha256::new();
            hasher.update(header.as_bytes());

            // Hash all values in this column
            for row in &table.rows {
                if idx < row.len() {
                    hasher.update(row[idx].as_bytes());
                }
            }

            header_hashes.insert(header.clone(), format!("{:x}", hasher.finalize()));
        }

        Self {
            table_hash,
            header_hashes,
            row_hashes: None,
        }
    }
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
        let timestamp = Utc::now();
        let id = format!("{}-{}", timestamp.timestamp(), &hashes.table_hash[..8]);

        Self {
            id,
            timestamp,
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
    pub fn save(&self, path: &Path) -> Result<()> {
        let toml_string = toml::to_string_pretty(self)?;
        fs::write(path, toml_string)?;
        Ok(())
    }

    /// Load snapshot from disk
    pub fn load(path: &Path) -> Result<Snapshot> {
        let content = fs::read_to_string(path)?;
        let snapshot: Snapshot = toml::from_str(&content)?;
        Ok(snapshot)
    }

    /// Verify integrity of this snapshot
    pub fn verify(&self) -> bool {
        let computed = TableHashes::compute(&self.table);
        computed.table_hash == self.hashes.table_hash
    }

    /// Verify dependencies of this snapshot
    pub fn verify_dependencies(&self) -> Result<()> {
        for dep in &self.dependencies {
            if let Some(dep_path) = &dep.path {
                let content = fs::read_to_string(dep_path)?;
                let computed_hash = Self::compute_hash(&content);
                if computed_hash != dep.hash {
                    return Err(GitSheetsError::DependencyHashMismatch(format!(
                        "Dependency '{}' hash mismatch",
                        dep.name
                    )));
                }
            }
        }
        Ok(())
    }

    /// Compute hash for string content
    fn compute_hash(content: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(content.as_bytes());
        format!("{:x}", hasher.finalize())
    }
}

// ============================================================================
// TABLE OPERATIONS
// ============================================================================

impl Table {
    /// Create a table from CSV data
    pub fn from_csv(path: &Path) -> Result<Self> {
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
            let row: Vec<String> = record.iter().map(|cell| cell.trim().to_string()).collect();
            rows.push(row);
        }

        // Allow empty tables (headers but no data rows) - this is a valid state
        // that should be tracked as a snapshot

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
    pub fn get_row_key(&self, row_idx: usize) -> Result<Vec<String>> {
        let pk_indices = self
            .primary_key
            .as_ref()
            .ok_or_else(|| GitSheetsError::NoPrimaryKey)?;

        let row = self.rows.get(row_idx).ok_or_else(|| {
            GitSheetsError::InvalidRowIndex(format!(
                "Row index {} exceeds row count {}",
                row_idx,
                self.rows.len()
            ))
        })?;

        let pk_values: Vec<String> = pk_indices
            .iter()
            .filter_map(|&idx| row.get(idx).cloned())
            .collect();

        if pk_values.is_empty() {
            return Err(GitSheetsError::NoPrimaryKey);
        }

        Ok(pk_values)
    }
}

// ============================================================================
// REPO OPERATIONS
// ============================================================================

/// A git-sheets repository
pub struct GitSheetsRepo {
    /// Path to the repository
    pub path: PathBuf,
    /// Git repository handle (optional)
    pub git_repo: Option<git2::Repository>,
}

impl GitSheetsRepo {
    /// Initialize a new git-sheets repository
    pub fn init(path: PathBuf) -> Result<GitSheetsRepo> {
        let repo_path = path.canonicalize()?;

        // Create directory structure
        std::fs::create_dir_all(repo_path.join("snapshots"))?;
        std::fs::create_dir_all(repo_path.join("diffs"))?;

        // Create .gitignore if needed
        let gitignore_path = repo_path.join(".gitignore");
        if !gitignore_path.exists() {
            let mut gitignore = std::fs::File::create(gitignore_path)?;
            writeln!(gitignore, "snapshots/")?;
            writeln!(gitignore, "diffs/")?;
            writeln!(gitignore, "*.toml")?;
            writeln!(gitignore, "*.json")?;
        }

        Ok(GitSheetsRepo {
            path: repo_path,
            git_repo: None,
        })
    }

    /// Open an existing git-sheets repository
    pub fn open(path: &str) -> Result<GitSheetsRepo> {
        let repo_path = PathBuf::from(path).canonicalize()?;

        if !repo_path.join("snapshots").exists() {
            return Err(GitSheetsError::FileSystemError(
                "Not a git-sheets repository".to_string(),
            ));
        }

        Ok(GitSheetsRepo {
            path: repo_path,
            git_repo: None,
        })
    }

    /// Commit a snapshot to git
    pub fn commit_snapshot(&self) -> Result<()> {
        // This is a placeholder implementation
        // In a real implementation, this would integrate with git
        Ok(())
    }

    /// Check if there are uncommitted changes
    pub fn has_changes(&self) -> bool {
        // Placeholder implementation
        false
    }
}
