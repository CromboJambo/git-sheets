// git-sheets: CLI module - command parsing and implementations
// A tool for Excel sufferers who deserve better

use crate::core::Table;
use crate::core::{GitSheetsError, Result, Snapshot};
use crate::diff::{Change, SnapshotDiff};
use clap::{Parser, Subcommand};
use std::io::Write;
use std::path::Path;

#[derive(Parser)]
#[command(name = "git-sheets")]
#[command(about = "Version control for spreadsheets", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize a new git-sheets repository
    Init {
        /// Path to initialize the repository
        #[arg(value_name = "PATH")]
        path: String,
    },

    /// Create a snapshot of a table
    Snapshot {
        /// Table file to snapshot
        #[arg(value_name = "FILE")]
        file: String,

        /// Commit message for the snapshot
        #[arg(short, long)]
        message: Option<String>,

        /// Set which column(s) form the primary key
        #[arg(long)]
        primary_key: Option<String>,

        /// Auto-commit to git after creating snapshot
        #[arg(long)]
        auto_commit: bool,
    },

    /// Show a diff between two snapshots
    Diff {
        /// First snapshot file
        #[arg(value_name = "FROM")]
        from: String,

        /// Second snapshot file
        #[arg(value_name = "TO")]
        to: String,

        /// Output format (json or git)
        #[arg(short, long)]
        format: Option<String>,
    },

    /// Verify integrity of a snapshot
    Verify {
        /// Snapshot file to verify
        #[arg(value_name = "FILE")]
        file: String,
    },

    /// Show current status
    Status,

    /// List all snapshots
    Log {
        /// Limit number of snapshots shown
        #[arg(short, long)]
        limit: Option<usize>,
    },
}

/// Diff output format
#[derive(Debug, Clone, Copy, PartialEq, Eq, clap::ValueEnum)]
pub enum DiffFormat {
    /// JSON format
    Json,
    /// Git-style unified diff
    Git,
}

// ============================================================================
// COMMAND IMPLEMENTATIONS
// ============================================================================

fn init_repository(path: &Path) -> Result<()> {
    println!("Initializing git-sheets repository at {}", path.display());

    // Create necessary directories
    std::fs::create_dir_all(path.join("snapshots"))?;
    std::fs::create_dir_all(path.join("diffs"))?;

    // Create .gitignore if needed
    let gitignore_path = path.join(".gitignore");
    if !gitignore_path.exists() {
        let mut gitignore = std::fs::File::create(gitignore_path)?;
        writeln!(gitignore, "snapshots/")?;
        writeln!(gitignore, "diffs/")?;
        writeln!(gitignore, "*.toml")?;
        writeln!(gitignore, "*.json")?;
    }

    Ok(())
}

fn create_snapshot(
    file: &Path,
    message: Option<String>,
    primary_key: Option<String>,
    auto_commit: bool,
) -> Result<()> {
    println!("Creating snapshot of {}", file.display());

    // Load the table
    let mut table = Table::from_csv(file)?;

    // Set primary key if specified
    if let Some(pk_str) = primary_key {
        let pk_indices: Vec<usize> = pk_str
            .split(',')
            .map(|s| s.trim().parse::<usize>().unwrap_or(0))
            .collect();
        table.set_primary_key(pk_indices);
    }

    // Create snapshot
    let snapshot = Snapshot::new(table, message);

    // Save snapshot
    let snapshot_path = Path::new("snapshots").join(format!("{}.toml", snapshot.id));
    snapshot.save(&snapshot_path)?;

    println!("Snapshot created: {}", snapshot.id);

    if auto_commit {
        // Placeholder for git commit
        println!("Auto-commit would be performed here");
    }

    Ok(())
}

fn show_diff(from: &Path, to: &Path, format: &str) -> Result<()> {
    println!("Computing diff...");

    let snapshot1 = Snapshot::load(from)?;
    let snapshot2 = Snapshot::load(to)?;
    let diff = SnapshotDiff::compute(&snapshot1, &snapshot2)?;

    match format {
        "json" => {
            diff.save(&Path::new("diffs").join(format!("{}.json", diff.from_id)))?;
            println!("Diff saved as JSON");
        }
        "git" => {
            // Print git-style diff
            println!("--- {}", snapshot1.id);
            println!("+++ {}", snapshot2.id);
            for change in &diff.changes {
                match change {
                    Change::RowAdded { index, data } => {
                        println!("@@ -0 +{} @@", index + 1);
                        println!("+{}", data.join("\t"));
                    }
                    Change::RowRemoved { index, data } => {
                        println!("@@ -{} +0 @@", index + 1);
                        println!("-{}", data.join("\t"));
                    }
                    Change::CellChanged { row, col, old, new } => {
                        println!("@@ -{} +{} @@", row + 1, row + 1);
                        println!("-{}", old);
                        println!("+{}", new);
                    }
                    Change::RowModified {
                        index,
                        old_data,
                        new_data,
                    } => {
                        println!("@@ -{} +{} @@", index + 1, index + 1);
                        println!("-{}", old_data.join("\t"));
                        println!("+{}", new_data.join("\t"));
                    }
                    Change::ColumnAdded { name, index } => {
                        println!("@@ -0 +{} @@", index + 1);
                        println!("+{}", name);
                    }
                    Change::ColumnRemoved { name, index } => {
                        println!("@@ -{} +0 @@", index + 1);
                        println!("-{}", name);
                    }
                }
            }
        }
        _ => {
            // Default to text format
            print_diff_text(&diff);
        }
    }

    Ok(())
}

fn print_diff_text(diff: &SnapshotDiff) {
    println!("Diff from {} to {}", diff.from_id, diff.to_id);
    println!("Summary:");
    println!("  Rows added: {}", diff.summary.rows_added);
    println!("  Rows removed: {}", diff.summary.rows_removed);
    println!("  Rows modified: {}", diff.summary.rows_modified);

    if !diff.changes.is_empty() {
        println!("Changes:");
        for change in &diff.changes {
            match change {
                Change::RowAdded { index, data } => {
                    println!("Row added at {}: {:?}", index, data);
                }
                Change::RowRemoved { index, data } => {
                    println!("Row removed at {}: {:?}", index, data);
                }
                Change::CellChanged { row, col, old, new } => {
                    println!("Cell changed at ({}, {}): {} -> {}", row, col, old, new);
                }
                Change::RowModified {
                    index,
                    old_data,
                    new_data,
                } => {
                    println!("Row modified at {}: {:?} -> {:?}", index, old_data, new_data);
                }
                Change::ColumnAdded { name, index } => {
                    println!("Column added at {}: {}", index, name);
                }
                Change::ColumnRemoved { name, index } => {
                    println!("Column removed at {}: {}", index, name);
                }
            }
        }
    }
}

fn verify_snapshot(path: &Path) -> Result<()> {
    println!("Verifying snapshot: {}", path.display());

    let snapshot = Snapshot::load(path)?;

    if snapshot.verify() {
        println!("Snapshot integrity verified");
    } else {
        println!("Snapshot integrity check failed");
        return Err(GitSheetsError::FileSystemError(
            "Snapshot verification failed".to_string(),
        ));
    }

    Ok(())
}

fn show_status() -> Result<()> {
    println!("Git-sheets status\n");

    // Check if git repo exists
    let repo_path = Path::new(".");
    if !repo_path.join("snapshots").exists() {
        println!("Not a git-sheets repository");
        return Err(GitSheetsError::FileSystemError(
            "Not a git-sheets repository".to_string(),
        ));
    }

    println!("Repository: {}", repo_path.display());
    println!("Snapshots directory: snapshots/");
    println!("Diffs directory: diffs/");

    Ok(())
}

fn show_log(limit: Option<usize>) -> Result<()> {
    let snapshots_dir = Path::new("snapshots");

    if !snapshots_dir.exists() {
        println!("No snapshots directory found");
        return Err(GitSheetsError::FileSystemError(
            "No snapshots directory".to_string(),
        ));
    }

    // List snapshots
    let mut snapshot_files: Vec<_> = std::fs::read_dir(snapshots_dir)?
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .filter(|path| path.extension().map_or(false, |ext| ext == "toml"))
        .collect();

    // Sort by name (which should be timestamp-based)
    snapshot_files.sort_by(|a, b| a.file_name().cmp(&b.file_name()));

    let limit = limit.unwrap_or(snapshot_files.len());
    let snapshots_to_show = snapshot_files.iter().take(limit);

    println!("Recent snapshots:");
    for path in snapshots_to_show {
        let filename = path.file_name().unwrap().to_string_lossy();
        println!("  {}", filename);
    }

    Ok(())
}

pub fn run() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Init { path } => {
            init_repository(&Path::new(path))?;
        }
        Commands::Snapshot {
            file,
            message,
            primary_key,
            auto_commit,
        } => {
            create_snapshot(
                &Path::new(file),
                message.clone(),
                primary_key.clone(),
                *auto_commit,
            )?;
        }
        Commands::Diff { from, to, format } => {
            let format_str = format.as_ref().map(|s| s.as_str()).unwrap_or("text");
            show_diff(&Path::new(from), &Path::new(to), format_str)?;
        }
        Commands::Verify { file } => {
            verify_snapshot(&Path::new(file))?;
        }
        Commands::Status => {
            show_status()?;
        }
        Commands::Log { limit } => {
            show_log(*limit)?;
        }
    }

    Ok(())
}
