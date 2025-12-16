# git-sheets

**Version control for spreadsheets - staging, commits, diffs for tables**

> For Excel sufferers who deserve better

---

## What Is This?

A command-line tool that brings Git-style version control to spreadsheets. Take snapshots of your CSV/Excel data, track changes, verify integrity, and never lose work to a bad macro again.

**Core principle**: User-triggered, explicit, reversible, local-first, boring by design.

---

## Installation

### From Source

```bash
# Clone the repository
git clone https://github.com/CromboJambo/git-sheets
cd git-sheets

# Build and install
cargo install --path .

# Or just build for local use
cargo build --release

# Binary will be in target/release/git-sheets
```

### Prerequisites

- [Rust toolchain](https://rustup.rs/) (stable)
- Git (optional, but recommended)

---

## Quick Start

### 1. Initialize a repository

```bash
# In your spreadsheet directory
git-sheets init
```

This creates:
- `snapshots/` - where table snapshots are stored
- `diffs/` - where diffs are saved
- `.gitignore` - configured for git-sheets
- `README.md` - usage guide

### 2. Create your first snapshot

```bash
# From a CSV export
git-sheets snapshot sales.csv -m "Initial Q4 data"

# With primary key (column indices 0 and 1)
git-sheets snapshot customers.csv -k "0,1" -m "Customer master list"

# Auto-commit to git
git-sheets snapshot sales.csv -m "Week 1 update" --commit
```

### 3. Compare snapshots

```bash
# Text format (human-readable)
git-sheets diff snapshots/sales_001.toml snapshots/sales_002.toml

# JSON format (machine-readable)
git-sheets diff snapshots/sales_001.toml snapshots/sales_002.toml -f json

# Git-style unified diff
git-sheets diff snapshots/sales_001.toml snapshots/sales_002.toml -f git
```

### 4. Verify integrity

```bash
# Check if a snapshot has been tampered with
git-sheets verify snapshots/sales_001.toml
```

### 5. View history

```bash
# Show all snapshots
git-sheets log

# Show last 5 snapshots
git-sheets log -l 5
```

### 6. Check status

```bash
git-sheets status
```

---

## Example Workflows

### The "Before Macro" Safety Net

```bash
# 1. Export current state to CSV
#    (From Excel: File → Save As → CSV)

# 2. Snapshot before running macro
git-sheets snapshot data.csv -m "Before cleanup macro"

# 3. Run your macro in Excel

# 4. Export again
#    (From Excel: File → Save As → CSV)

# 5. Snapshot after macro
git-sheets snapshot data.csv -m "After cleanup macro"

# 6. See what changed
git-sheets diff snapshots/data_001.toml snapshots/data_002.toml

# 7. If macro broke something, you have proof of what changed
```

### The "Inherited Spreadsheet" Audit

```bash
# 1. Snapshot the inherited mess
git-sheets snapshot inherited_horror.csv -m "Received from Bob, 2025-12-15"

# 2. Make your changes carefully

# 3. Snapshot after each logical change
git-sheets snapshot inherited_horror.csv -m "Fixed formula in column D"
git-sheets snapshot inherited_horror.csv -m "Removed duplicate rows"
git-sheets snapshot inherited_horror.csv -m "Added validation column"

# 4. You now have an audit trail showing:
#    - What you received
#    - Each change you made
#    - Why you made it
```

---

## Key Design Principles

### 1. User-Triggered Only
- No automatic snapshots
- No background processes
- No hidden telemetry

### 2. Explicit Actions
- You decide when to snapshot
- You decide what to commit
- You see the diff before accepting

### 3. Reversible
- Every snapshot is immutable
- Every change can be rolled back
- Full audit trail

### 4. Local-First
- Your data never leaves your machine
- Git is optional but recommended
- No cloud dependencies

### 5. Boring by Design
- Simple file formats (TOML, JSON)
- Standard tools (Git)
- No magic

---

## File Formats

### Snapshot Format (TOML)

```toml
id = "1734307200-abc12345"
timestamp = "2025-12-15T10:00:00Z"
message = "Initial snapshot"

[table]
headers = ["ID", "Name", "Amount"]
primary_key = [0]

[[table.rows]]
row = ["1", "Alice", "100"]

[[table.rows]]
row = ["2", "Bob", "200"]

[hashes]
table_hash = "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"

[hashes.header_hashes]
ID = "abc123..."
Name = "def456..."
Amount = "ghi789..."
```

### Diff Format (JSON)

```json
{
  "from_id": "1734307200-abc12345",
  "to_id": "1734307300-def67890",
  "summary": {
    "rows_added": 5,
    "rows_removed": 2,
    "rows_modified": 10,
    "columns_added": 1,
    "columns_removed": 0
  },
  "changes": [
    {
      "CellChanged": {
        "row": 3,
        "col": 2,
        "old": "100",
        "new": "150"
      }
    }
  ]
}
```

---

## Integration with Excel Workflow

### Recommended Setup

1. **Export to CSV before snapshots**
   - Excel: `File → Save As → CSV (UTF-8)`
   - LibreCalc: `File → Save As → Text CSV`

2. **Use consistent naming**
   - `sales_data.csv` → generates `sales_data_<timestamp>.toml`
   - Keeps snapshots organized

3. **Snapshot at key moments**
   - Before macros
   - After major edits
   - Before Find/Replace
   - End of day/week

4. **Commit to Git regularly**
   - Use `--commit` flag for auto-commit
   - Or commit manually with `git commit`

---

## Roadmap

### Phase 1 (Current)
- ✅ Core snapshot functionality
- ✅ CSV import
- ✅ Per-header hashing
- ✅ Diff calculation
- ✅ CLI interface

### Phase 2 (Next)
- [ ] Excel file support (.xlsx)
- [ ] Multi-sheet support
- [ ] Dependency tracking
- [ ] Visual diff tool

### Phase 3 (Future)
- [ ] LibreOffice extension
- [ ] Staging for Find/Replace
- [ ] Live formula tracking
- [ ] Collaborative features

---

## Why This Matters

### For Accountants
- Audit trails for regulatory compliance
- Proof of what changed and why
- Safety net for inherited workbooks

### For Analysts
- Experiment without fear
- Track methodology evolution
- Reproducible analysis

### For Teams
- Clear change history
- Reduced "who broke it" blame
- Knowledge preservation

### For Everyone
- Excel stops being a minefield
- Iteration becomes safe
- Human dignity preserved

---

## Contributing

This tool was built by Excel sufferers, for Excel sufferers. If you've ever:

- Lost work to a macro
- Been blamed for an inherited spreadsheet
- Feared the Find/Replace button
- Wished for Ctrl+Z after saving

...then you understand why this exists.

Contributions welcome. Keep it boring, safe, and user-first.

---

## License

MIT - Use it, modify it, share it. Just keep it honest.

---

**Remember**: This isn't about making spreadsheets fancy. It's about making them *safe*.
