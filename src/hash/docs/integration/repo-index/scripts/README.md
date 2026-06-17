# Cargo Metadata Index Scripts

This directory contains Nushell scripts for extracting and querying cargo metadata from your Rust repositories.

## Scripts

### 1. `cargo-metadata-extractor.nu`
Extracts cargo metadata from all repositories and creates an index.

**Usage:**
```bash
./cargo-metadata-extractor.nu
```

**What it does:**
- Recursively finds all `Cargo.toml` files
- Extracts metadata using `cargo metadata --format-version 1`
- Saves individual metadata files for each repo
- Creates a consolidated index file

**Output:**
- Individual metadata files in `data/` directory
- Consolidated index at `data/repo-index.json`

### 2. `repo-list.nu`
Lists all indexed repositories.

**Usage:**
```bash
./repo-list.nu
```

**Output:**
- List of all indexed repositories with names, versions, and descriptions

### 3. `repo-lookup.nu`
Look up detailed information about a specific repository.

**Usage:**
```bash
./repo-lookup.nu <repo-name>
```

**Example:**
```bash
./repo-lookup.nu git-sheets
```

**Output:**
- Repository name and version
- Description
- Path
- Number of dependencies
- List of direct dependencies

### 4. `repo-diff.nu`
Compare dependencies between two repositories.

**Usage:**
```bash
./repo-diff.nu <repo1> <repo2>
```

**Example:**
```bash
./repo-diff.nu git-sheets nustage
```

**Output:**
- Summary of differences
- Added dependencies
- Removed dependencies
- Changed versions

## Workflow

1. **First time setup:**
   ```bash
   ./cargo-metadata-extractor.nu
   ```

2. **View all repos:**
   ```bash
   ./repo-list.nu
   ```

3. **Get details about a repo:**
   ```bash
   ./repo-lookup.nu <repo-name>
   ```

4. **Compare two repos:**
   ```bash
   ./repo-diff.nu <repo1> <repo2>
   ```

5. **Update index when adding new repos:**
   ```bash
   ./cargo-metadata-extractor.nu
   ```

## Data Files

- `data/repo-index.json` - Consolidated index of all repositories
- `data/<repo-name>-metadata.json` - Individual metadata for each repo

## Agent Integration

Your agent can read the index files directly:

1. **Read all repos:**
   ```bash
   open data/repo-index.json | from json
   ```

2. **Read specific repo:**
   ```bash
   open data/<repo-name>-metadata.json | from json
   ```

3. **Query for dependencies:**
   ```bash
   open data/repo-index.json | from json | where name == "git-sheets" | get dependencies
   ```

## Requirements

- Nushell (nu) installed
- Cargo (Rust package manager) installed
- Each repository must have a `Cargo.toml` file
