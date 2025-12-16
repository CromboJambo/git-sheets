// git-sheets CLI: For Excel sufferers who deserve version control
//
// Usage:
//   git-sheets snapshot <file.csv> -m "message"
//   git-sheets diff <snapshot1> <snapshot2>
//   git-sheets verify <snapshot>
//   git-sheets init
//   git-sheets status

use std::path::{Path, PathBuf};
use std::process::Command;
use std::fs;
use clap::{Parser, Subcommand};
use gitsheets::{Table, Snapshot, SnapshotDiff, Change};

// Re-use the core types from the previous module
// In real code, these would be: use gitsheets::{Snapshot, Table, SnapshotDiff};

#[derive(Parser)]
#[command(name = "git-sheets")]
#[command(about = "Version control for spreadsheets", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize a git-sheets repository
    Init {
        /// Directory to initialize (defaults to current)
        #[arg(short, long, default_value = ".")]
        path: PathBuf,
    },

    /// Create a snapshot of a CSV/Excel file
    Snapshot {
        /// Path to the CSV file
        file: PathBuf,

        /// Commit message
        #[arg(short, long)]
        message: Option<String>,

        /// Primary key column indices (comma-separated)
        #[arg(short = 'k', long)]
        primary_key: Option<String>,

        /// Auto-commit to git
        #[arg(short = 'c', long)]
        commit: bool,
    },

    /// Show differences between two snapshots
    Diff {
        /// First snapshot file
        from: PathBuf,

        /// Second snapshot file
        to: PathBuf,

        /// Output format: text, json, or git
        #[arg(short, long, default_value = "text")]
        format: String,
    },

    /// Verify snapshot integrity
    Verify {
        /// Snapshot file to verify
        snapshot: PathBuf,
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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init { path } => {
            init_repository(&path)?;
        }

        Commands::Snapshot { file, message, primary_key, commit } => {
            create_snapshot(&file, message, primary_key, commit)?;
        }

        Commands::Diff { from, to, format } => {
            show_diff(&from, &to, &format)?;
        }

        Commands::Verify { snapshot } => {
            verify_snapshot(&snapshot)?;
        }

        Commands::Status => {
            show_status()?;
        }

        Commands::Log { limit } => {
            show_log(limit)?;
        }
    }

    Ok(())
}

// ============================================================================
// COMMAND IMPLEMENTATIONS
// ============================================================================

fn init_repository(path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    println!("Initializing git-sheets repository at {}", path.display());

    // Create necessary directories
    let snapshots_dir = path.join("snapshots");
    let diffs_dir = path.join("diffs");

    fs::create_dir_all(&snapshots_dir)?;
    fs::create_dir_all(&diffs_dir)?;

    // Initialize git if not already present
    if !path.join(".git").exists() {
        println!("Initializing git repository...");
        Command::new("git")
            .arg("init")
            .current_dir(path)
            .status()?;
    }

    // Create .gitignore
    let gitignore_content = r#"
# Git-sheets specific
*.csv
*.xlsx
*.xls
*.tmp

# Keep snapshots and diffs
!snapshots/
!diffs/
"#;

    fs::write(path.join(".gitignore"), gitignore_content)?;

    // Create README
    let readme_content = r#"# Git-Sheets Repository

This directory is managed by git-sheets for version control of spreadsheets.

## Structure

- `snapshots/` - Snapshot files (.toml)
- `diffs/` - Diff files (.json)

## Usage

```bash
# Create a snapshot
git-sheets snapshot data.csv -m "Initial import"

# Compare snapshots
git-sheets diff snapshots/data_001.toml snapshots/data_002.toml

# Verify integrity
git-sheets verify snapshots/data_001.toml

# View history
git-sheets log
```

## Safety Principles

1. **User-triggered only** - No automatic snapshots
2. **Explicit commits** - You decide what gets saved
3. **Reversible** - Every snapshot can be rolled back
4. **Auditable** - Full history of changes
5. **Local-first** - Your data stays on your machine
"#;

    fs::write(path.join("README.md"), readme_content)?;

    println!("✓ Created snapshots/ directory");
    println!("✓ Created diffs/ directory");
    println!("✓ Created .gitignore");
    println!("✓ Created README.md");
    println!("\nRepository initialized. Try:");
    println!("  git-sheets snapshot <file.csv> -m \"First snapshot\"");

    Ok(())
}

fn create_snapshot(
    file: &Path,
    message: Option<String>,
    primary_key: Option<String>,
    auto_commit: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("Creating snapshot of {}", file.display());

    // Load the table
    let mut table = Table::from_csv(file)?;

    // Set primary key if specified
    if let Some(pk_str) = primary_key {
        let indices: Vec<usize> = pk_str
            .split(',')
            .filter_map(|s| s.trim().parse().ok())
            .collect();

        if !indices.is_empty() {
            table.set_primary_key(indices.clone());
            println!("✓ Set primary key: columns {}",
                indices.iter()
                    .map(|i| table.headers.get(*i).map(|s| s.as_str()).unwrap_or("?"))
                    .collect::<Vec<_>>()
                    .join(", ")
            );
        }
    }

    // Create snapshot
    let snapshot = Snapshot::new(table, message.clone());

    // Save to snapshots directory
    let snapshot_dir = Path::new("snapshots");
    if !snapshot_dir.exists() {
        fs::create_dir_all(snapshot_dir)?;
    }

    let filename = format!("{}_{}.toml",
        file.file_stem().unwrap().to_string_lossy(),
        snapshot.id
    );
    let snapshot_path = snapshot_dir.join(&filename);

    snapshot.save(&snapshot_path)?;

    println!("✓ Snapshot saved: {}", snapshot_path.display());
    println!("  ID: {}", snapshot.id);
    println!("  Rows: {}", snapshot.table.rows.len());
    println!("  Columns: {}", snapshot.table.headers.len());
    println!("  Table hash: {}...", &snapshot.hashes.table_hash[..16]);

    // Auto-commit to git if requested
    if auto_commit {
        println!("\nCommitting to git...");

        Command::new("git")
            .args(&["add", snapshot_path.to_str().unwrap()])
            .status()?;

        let commit_msg = message
            .unwrap_or_else(|| format!("Snapshot: {}", filename));

        Command::new("git")
            .args(&["commit", "-m", &commit_msg])
            .status()?;

        println!("✓ Committed to git");
    } else {
        println!("\nTo commit to git:");
        println!("  git add {}", snapshot_path.display());
        println!("  git commit -m \"{}\"",
            message.unwrap_or_else(|| "Snapshot".to_string()));
    }

    Ok(())
}

fn show_diff(
    from: &Path,
    to: &Path,
    format: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("Computing diff...");

    let snapshot1 = Snapshot::load(from)?;
    let snapshot2 = Snapshot::load(to)?;

    let diff = SnapshotDiff::compute(&snapshot1, &snapshot2);

    match format {
        "json" => {
            println!("{}", serde_json::to_string_pretty(&diff)?);
        }
        "text" => {
            print_diff_text(&diff);
        }
        "git" => {
            print_diff_git_style(&diff, &snapshot1, &snapshot2);
        }
        _ => {
            eprintln!("Unknown format: {}", format);
        }
    }

    Ok(())
}

fn print_diff_text(diff: &SnapshotDiff) {
    println!("\n═══════════════════════════════════════");
    println!("Diff: {} → {}", diff.from_id, diff.to_id);
    println!("═══════════════════════════════════════\n");

    let s = &diff.summary;
    println!("Summary:");
    println!("  Rows:    +{} -{} ~{}", s.rows_added, s.rows_removed, s.rows_modified);
    println!("  Columns: +{} -{}", s.columns_added, s.columns_removed);

    if !diff.changes.is_empty() {
        println!("\nChanges:");

        for change in &diff.changes {
            match change {
                Change::RowAdded { index, data } => {
                    println!("  + Row {}: {:?}", index, data);
                }
                Change::RowRemoved { index, data } => {
                    println!("  - Row {}: {:?}", index, data);
                }
                Change::CellChanged { row, col, old, new } => {
                    println!("  ~ Cell[{},{}]: \"{}\" → \"{}\"", row, col, old, new);
                }
                Change::ColumnAdded { name, index } => {
                    println!("  + Column {}: \"{}\"", index, name);
                }
                Change::ColumnRemoved { name, index } => {
                    println!("  - Column {}: \"{}\"", index, name);
                }
            }
        }
    }

    println!();
}

fn print_diff_git_style(diff: &SnapshotDiff, from: &Snapshot, to: &Snapshot) {
    println!("diff --git a/{} b/{}", diff.from_id, diff.to_id);
    println!("--- a/{}", diff.from_id);
    println!("+++ b/{}", diff.to_id);
    println!("@@ Summary @@");
    println!(" Rows: {} → {}", from.table.rows.len(), to.table.rows.len());
    println!(" Columns: {} → {}", from.table.headers.len(), to.table.headers.len());

    for change in &diff.changes {
        match change {
            Change::CellChanged { row, col, old, new } => {
                println!("-{},{}: {}", row, col, old);
                println!("+{},{}: {}", row, col, new);
            }
            _ => {}
        }
    }
}

fn verify_snapshot(path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    println!("Verifying snapshot: {}", path.display());

    let snapshot = Snapshot::load(path)?;

    if snapshot.verify() {
        println!("✓ Integrity check passed");
        println!("  Snapshot ID: {}", snapshot.id);
        println!("  Timestamp: {}", snapshot.timestamp);
        if let Some(msg) = &snapshot.message {
            println!("  Message: {}", msg);
        }
        println!("  Table hash: {}", snapshot.hashes.table_hash);
    } else {
        eprintln!("✗ Integrity check FAILED");
        eprintln!("  This snapshot may be corrupted!");
        std::process::exit(1);
    }

    Ok(())
}

fn show_status() -> Result<(), Box<dyn std::error::Error>> {
    println!("Git-sheets status\n");

    // Check if git repo exists
    let git_status = Command::new("git")
        .args(&["status", "--short"])
        .output()?;

    if git_status.status.success() {
        println!("Git repository: ✓");
        let output = String::from_utf8_lossy(&git_status.stdout);
        if !output.trim().is_empty() {
            println!("\nUncommitted changes:");
            println!("{}", output);
        }
    } else {
        println!("Git repository: ✗ (run 'git-sheets init')");
    }

    // List snapshots
    let snapshots_dir = Path::new("snapshots");
    if snapshots_dir.exists() {
        let count = fs::read_dir(snapshots_dir)?
            .filter_map(|e| e.ok())
            .filter(|e| e.path().extension().map(|s| s == "toml").unwrap_or(false))
            .count();

        println!("\nSnapshots: {}", count);
    }

    Ok(())
}

fn show_log(limit: Option<usize>) -> Result<(), Box<dyn std::error::Error>> {
    let snapshots_dir = Path::new("snapshots");

    if !snapshots_dir.exists() {
        println!("No snapshots found. Create one with:");
        println!("  git-sheets snapshot <file.csv> -m \"message\"");
        return Ok(());
    }

    let mut snapshots: Vec<_> = fs::read_dir(snapshots_dir)?
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().map(|s| s == "toml").unwrap_or(false))
        .collect();

    // Sort by modification time (newest first)
    snapshots.sort_by(|a, b| {
        let time_a = a.metadata().ok().and_then(|m| m.modified().ok());
        let time_b = b.metadata().ok().and_then(|m| m.modified().ok());
        time_b.cmp(&time_a)
    });

    let display_count = limit.unwrap_or(snapshots.len()).min(snapshots.len());

    println!("Showing {} most recent snapshots:\n", display_count);

    for entry in snapshots.iter().take(display_count) {
        let path = entry.path();
        if let Ok(snapshot) = Snapshot::load(&path) {
            println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
            println!("Snapshot: {}", snapshot.id);
            println!("Time:     {}", snapshot.timestamp.format("%Y-%m-%d %H:%M:%S"));
            if let Some(msg) = &snapshot.message {
                println!("Message:  {}", msg);
            }
            println!("Table:    {} rows × {} cols",
                snapshot.table.rows.len(),
                snapshot.table.headers.len()
            );
            println!("Hash:     {}...", &snapshot.hashes.table_hash[..16]);
            println!();
        }
    }

    Ok(())
}
