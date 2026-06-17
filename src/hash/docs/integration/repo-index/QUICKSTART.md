# Quick Start: Cargo Metadata Index

## Setup

1. **Navigate to the scripts directory:**
   ```bash
   cd /home/crombo/git-sheets/repo-index/scripts
   ```

2. **Make the scripts executable:**
   ```bash
   chmod +x *.nu
   ```

3. **Run the extractor:**
   ```bash
   ./cargo-metadata-extractor.nu
   ```

## Usage Examples

### List all repositories
```bash
./repo-list.nu
```

### Look up a specific repository
```bash
./repo-lookup.nu git-sheets
```

### Compare two repositories
```bash
./repo-diff.nu git-sheets nustage
```

## Agent Integration

Your agent can access the metadata directly:

```bash
# List all repos
open data/repo-index.json | from json

# Get specific repo
open data/git-sheets-metadata.json | from json

# Find repos with specific dependency
open data/repo-index.json | from json | where dependencies.name == "serde"
```

## Location

- **Scripts:** `/home/crombo/git-sheets/repo-index/scripts/`
- **Index:** `/home/crombo/git-sheets/repo-index/data/repo-index.json`
- **Individual metadata:** `/home/crombo/git-sheets/repo-index/data/<repo-name>-metadata.json`
