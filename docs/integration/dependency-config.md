# Dependency Configuration Guide

This document specifies how the three projects should declare their dependencies on each other at the Cargo workspace level.

## Current State Analysis

### Project Dependencies (Existing)

| Project | External Dependencies | Internal Workspace Members | Cross-Project Dependencies |
|---------|----------------------|----------------------------|---------------------------|
| `git-sheets` | chrono, serde, sha2, csv, toml, clap, git2, similar, walkdir | - | None |
| `nustage` | ratatui, crossterm, duckdb, polars, ironcalc, calamine, clap, thiserror, anyhow, serde, serde_json, chrono, uuid | - | None |
| `zed-sheet-lsp` (workspace) | tower-lsp, tokio, serde, serde_json, petgraph, chrono | zed-sheets-lsp | None currently |

### Recommended Dependency Direction

```
┌─────────────────┐         ┌──────────────────┐
│  git-sheets     │         │   nustage        │
│  (history)      │         │   (intent)       │
└─────────────────┘         └────────┬─────────┘
                                     │
                                     ▼
                          ┌────────────────────┐
                          │ zed-sheet-lsp      │
                          │ (editor integration)│
                          │ depends on:        │
                          │ → nustage (library)│
                          └────────────────────┘

git-sheets ↔ nustage: NO DIRECT DEPENDENCY
```

## Workspace Configuration Strategy

### Option A: Separate Workspaces (Current - Recommended for Now)

Keep each project as its own Cargo workspace with independent `Cargo.toml` files.

**Pros:**
- Maximum modularity
- Each can be built/tested independently
- No accidental cross-dependencies
- Clear ownership boundaries

**Cons:**
- Cross-project testing requires manual setup
- Version coordination between projects

**Recommended for:** Current state where each project should remain standalone-first.

### Option B: Unified Workspace (Future Integration)

Create a top-level workspace that includes all three as members.

```toml
# /workspace_root/Cargo.toml
[workspace]
resolver = "2"
members = [
    "git-sheets",
    "nustage", 
    "zed-sheet-lsp/zed-sheets-lsp",
]

[workspace.dependencies]
# Shared dependencies that all projects use
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
chrono = { version = "0.4", features = ["serde"] }
clap = { version = "4.5", features = ["derive"] }

# Cross-project dependencies (only zed-sheet-lsp uses these)
nustage-types = { path = "../nustage/src/lib.rs" }  # hypothetical extraction
```

**Pros:**
- Easier cross-project testing
- Shared dependency versions
- Unified `cargo test` command

**Cons:**
- Tighter coupling
- Breaking changes in one affect all builds
- More complex CI/CD requirements

## Recommended Approach: Gradual Integration

### Phase 1: Maintain Separate Workspaces (Current)

Keep projects independent. Document integration at the workflow level.

**Action items:**
- [x] Documentation complete (`docs/integration/`)
- [ ] Each project has clear standalone tests
- [ ] CLI workflows documented for composition

### Phase 2: Library Extraction + Optional Workspace

Extract shared types from `nustage` into a dedicated library crate that `zed-sheet-lsp` can depend on.

```toml
# nustage/Cargo.toml (modified)
[lib]
name = "nustage-core"           # New core library
path = "src/lib.rs"

[[bin]]
name = "nustage"                 # CLI binary remains
path = "src/main.rs"

# Add workspace member for zed-sheet-lsp to use
[dependencies]
# ... existing dependencies ...

# Optional: expose types as separate crate
[dependencies.nustage-types]
path = "../nustage-core"         # hypothetical new location
```

**Action items:**
- Identify which `nustage` types are stable contracts
- Create clear API boundary for external consumers
- Add `zed-sheet-lsp` dependency on extracted types

### Phase 3: Optional Unified Workspace (Opt-in)

Create optional workspace root that all three can participate in.

```
/workspace_root/
├── Cargo.toml                   # Workspace manifest
├── git-sheets/                  # Remains standalone-capable
│   └── Cargo.toml               # Can work without workspace
├── nustage/                     # Remains standalone-capable  
│   └── Cargo.toml               # Can work without workspace
└── zed-sheet-lsp/               # Optionally depends on others
    ├── Cargo.toml               # Can declare nustage dependency
    └── zed-sheets-lsp/
        └── Cargo.toml           # workspace.dependencies usage
```

## Practical Recommendations

### For `zed-sheet-lsp` → `nustage` Integration

When ready to integrate, add this to `zed-sheet-lsp/Cargo.toml`:

```toml
[dependencies]
# Existing dependencies...

# Optional: depend on nustage library types
nustage = { path = "../nustage", package = "nustage" }
```

Then in code:

```rust
// zed-sheet-lsp/zed-sheets-lsp/src/sidecar.rs
use nustage::sidecar::{SidecarFile, SidecarMetadata};
use nustage::transformations::{TransformationStep, ColumnSchema};

pub fn load_sidecar(path: &Path) -> Result<SidecarFile, LspError> {
    // Use nustage types directly, no duplication
}
```

### For `git-sheets` Independence

Keep `git-sheets` completely independent. It should never import from the other projects.

**Verification:** Add to CI:

```yaml
# .github/workflows/ci.yml
- name: Verify git-sheets independence
  run: |
    cd git-sheets
    cargo check --message-format=short | grep -v "git-sheets" || true
    # Should not have dependencies on nustage or zed-sheet-lsp

- name: Verify nustage independence  
  run: |
    cd nustage
    cargo check --message-format=short | grep -v "nustage" || true
    # Should not have dependencies on git-sheets or zed-sheet-lsp
```

## Migration Checklist

### When Adding Cross-Project Dependencies

1. **Identify the direction:** Which project should depend on which?
   - `zed-sheet-lsp` → `nustage`: OK (editor consumes intent)
   - `git-sheets` → others: NO (history is independent)
   - `nustage` → others: NO (intent doesn't know about editor/history)

2. **Check for duplicate types:** Are you redefining types that already exist?
   - If yes, use the canonical type from its owner project

3. **Update documentation:** Add new dependency to this document

4. **Add integration tests:** Verify cross-project behavior works correctly

5. **Verify independence:** Ensure core projects still build without optional dependencies

## Example: Adding Editor Integration with Sidecar Support

### Step 1: Define Contract (nustage)

Ensure `nustage::sidecar` types are stable and well-documented:

```rust
// nustage/src/sidecar/mod.rs
/// Canonical pipeline sidecar format.
/// 
/// This type is a public contract and will maintain backward compatibility.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SidecarFile {
    pub version: u32,
    pub source: String,
    pub pipeline: Vec<TransformationStep>,
    // ... other fields
}

impl SidecarFile {
    /// Load sidecar from file path.
    pub fn load(path: &Path) -> Result<Self, SidecarError> {
        // Implementation...
    }
    
    /// Save sidecar to file path.
    pub fn save(&self, path: &Path) -> Result<(), SidecarError> {
        // Implementation...
    }
}
```

### Step 2: Add Dependency (zed-sheet-lsp)

Update `zed-sheet-lsp/zed-sheets-lsp/Cargo.toml`:

```toml
[dependencies]
tower-lsp = "0.20"
tokio = { version = "1", features = ["full"] }
serde = { version = "1", features = ["derive"] }
# ... other dependencies ...

# NEW: Depend on nustage for sidecar handling
nustage = { path = "../nustage", package = "nustage" }
```

### Step 3: Use Canonical Types (zed-sheet-lsp)

Update `zed-sheet-lsp/zed-sheets-lsp/src/sidecar.rs`:

```rust
// OLD: Duplicate type definition (DON'T DO THIS)
#[derive(Debug, Clone)]
pub struct LocalSidecar {
    pub source: String,
    // ...
}

// NEW: Use canonical nustage types
use nustage::sidecar::{SidecarFile, SidecarMetadata};

pub fn load_nustage_sidecar(path: &Path) -> Result<SidecarFile, LspError> {
    let sidecar = SidecarFile::load(path.as_str())?;
    Ok(sidecar)  // Direct use of canonical type
}
```

### Step 4: Verify (CI/Testing)

Add verification that `zed-sheet-lsp` can build with optional `nustage`:

```toml
# zed-sheet-lsp/zed-sheets-lsp/Cargo.toml
[features]
default = []
full-integration = ["nustage"]  # Opt-in feature for full integration
```

## Summary

**Current recommendation:** Keep projects separate with documented workflow integration. This maximizes modularity and allows each project to evolve independently.

**Future path:** When editor-sidecar integration becomes necessary, add `zed-sheet-lsp` → `nustage` dependency via workspace or direct path reference. Never create circular dependencies.

**Key principle:** Each project should be buildable and testable without the others unless explicitly opted into integration.