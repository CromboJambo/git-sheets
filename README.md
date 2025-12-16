# git-sheets

**Version control for spreadsheets - staging, commits, diffs for tables**

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
