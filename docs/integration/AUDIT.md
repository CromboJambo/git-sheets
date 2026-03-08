# Integration Audit - Type Overlaps & Migration Paths

This document identifies current type overlaps across the three projects and provides concrete migration paths to eliminate duplication while preserving modularity.

## Executive Summary

**Critical Issue:** `zed-sheet-lsp` defines its own `Sidecar` type that duplicates functionality from `nustage::sidecar::SidecarFile`. This violates Rule 1 (one canonical sidecar format) and creates a second source of truth.

**Status:**
| Project | Standalone OK? | Has Overlaps? | Integration Ready? |
|---------|---------------|---------------|-------------------|
| `git-sheets` | ✅ Yes | ⚠️ Minor (hash types) | ✅ Yes |
| `nustage` | ✅ Yes | ❌ No | ✅ Yes |
| `zed-sheet-lsp` | ✅ Yes | 🔴 **Yes** - Sidecar duplication | ⚠️ Needs work |

## Type Overlap Inventory

### 🔴 Critical: Duplicate Sidecar Definitions

#### Location 1: nustage (Canonical)
```rust
// nustage/src/sidecar/mod.rs:14-24
pub struct SidecarFile {
    pub version: u32,
    pub source: String,
    #[serde(default)]
    pub pipeline: Vec<TransformationStep>,
    #[serde(default)]
    pub schema_history: HashMap<String, Vec<ColumnSchema>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<SidecarMetadata>,
}

pub struct SidecarMetadata {
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub modified_at: chrono::DateTime<chrono::Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}
```

#### Location 2: zed-sheet-lsp (Duplicate - Should Remove)
```rust
// zed-sheet-lsp/zed-sheets-lsp/src/sidecar.rs:19-23
pub struct Sidecar {
    pub version: u32,
    pub columns: HashMap<String, ColumnMetadata>,
    pub named_ranges: HashMap<String, NamedRange>,
}

// Also in zed-sheet-lsp/zed-sheets-lsp/src/document.rs:55-60 (similar)
pub struct Sidecar {
    pub version: u32,
    pub columns: HashMap<String, ColumnMetadata>,
    #[serde(default)]
    pub named_ranges: HashMap<String, NamedRange>,
}
```

**Impact:** Two different sidecar schemas exist. One is canonical (`nustage`), one is editor-local but duplicates structure. This creates drift risk and confusion about which format to use.

**Migration Action Required:** Remove `zed-sheet-lsp::Sidecar`, import `nustage::sidecar::SidecarFile` instead.

### ⚠️ Minor: Hash Type Differences

#### Location 1: git-sheets (Table integrity hashes)
```rust
// git-sheets/src/core/mod.rs:49-52
pub struct TableHashes {
    pub table_hash: String,
    pub header_hashes: HashMap<String, String>,
    pub row_hashes: Option<Vec<String>>,
}

impl TableHashes {
    pub fn compute(table: &Table) -> Self {
        // SHA-256 computation...
    }
}
```

#### Location 2: git-sheets (Snapshot hashes - similar structure)
```rust
// Same file, separate but related types
pub struct Snapshot {
    pub id: String,
    pub timestamp: DateTime<Utc>,
    pub message: Option<String>,
    pub table: Table,
    pub hashes: TableHashes,  // Uses the above type
}
```

**Status:** This is internal to `git-sheets`. No overlap with other projects. **No action needed.**

### ℹ️ Informational: Column Schema Types

#### Location 1: nustage (Canonical pipeline schema)
```rust
// nustage/src/transformations/mod.rs
pub struct ColumnSchema {
    pub name: String,
    pub type_name: String,
}
```

#### Location 2: zed-sheet-lsp (Editor-local metadata - different purpose)
```rust
// zed-sheet-lsp/zed-sheets-lsp/src/document.rs (referenced in Sidecar)
pub struct ColumnMetadata {
    // Likely editor-specific fields like position, validation rules, etc.
}
```

**Status:** These serve different purposes:
- `nustage::ColumnSchema`: Schema for pipeline steps (what columns exist and their types)
- `zed-sheet-lsp::ColumnMetadata`: Editor-local metadata (likely UI state, validation hints)

**Recommendation:** Keep separate but document the relationship. The editor can use `nustage` schema for completion/diagnostics, then layer editor-specific metadata on top.

## Migration Plan

### Phase 1: Remove Sidecar Duplication (High Priority)

**Goal:** Eliminate `zed-sheet-lsp::Sidecar`, use `nustage::sidecar::SidecarFile` instead.

#### Step 1.1: Verify nustage API Stability
Ensure `nustage::sidecar::SidecarFile` and related types are stable public APIs:

```rust
// Check that these are properly exported from nustage lib
pub use sidecar::{SidecarError, SidecarFile, SidecarMetadata};  // In nustage/src/lib.rs
```

**Verification:** If not exported, add to `nustage/src/lib.rs` with `#[doc(hidden)]` or stable documentation.

#### Step 1.2: Update zed-sheet-lsp Dependencies
Add dependency on `nustage`:

```toml
# zed-sheet-lsp/zed-sheets-lsp/Cargo.toml
[dependencies]
tower-lsp = "0.20"
tokio = { version = "1", features = ["full"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
petgraph = "0.6"

# NEW: Add nustage as optional or required dependency
nustage = { path = "../nustage", package = "nustage", features = ["sidecar"] }
```

#### Step 1.3: Replace Local Sidecar Type
Update `zed-sheet-lsp/zed-sheets-lsp/src/sidecar.rs`:

**Before:**
```rust
pub struct Sidecar {
    pub version: u32,
    pub columns: HashMap<String, ColumnMetadata>,
    pub named_ranges: HashMap<String, NamedRange>,
}

impl serde::Serialize for Sidecar { ... }
impl serde::Deserialize<'_> for Sidecar { ... }
```

**After:**
```rust
// Use canonical nustage types instead
use nustage::sidecar::{SidecarFile, SidecarMetadata};
use nustage::transformations::ColumnSchema;

pub type ColumnMetadata = ColumnSchema;  // Re-export if needed for compatibility
pub type NamedRange = String;  // Or define appropriate type

// Remove local Sidecar definition entirely
// All code should use SidecarFile from nustage
```

#### Step 1.4: Update Document Module
Update `zed-sheet-lsp/zed-sheets-lsp/src/document.rs`:

**Before:**
```rust
pub struct Sidecar {
    pub version: u32,
    pub columns: HashMap<String, ColumnMetadata>,
    #[serde(default)]
    pub named_ranges: HashMap<String, NamedRange>,
}
```

**After:**
```rust
// Simply re-export or use nustage types directly
pub type DocumentSidecar = SidecarFile;  // Alias for clarity in this context

// Or import and use directly:
use nustage::sidecar::{SidecarFile as DocumentSidecar, SidecarMetadata};
```

#### Step 1.5: Update All Usage Sites
Search and replace all uses of local `Sidecar` type with `SidecarFile`:

```bash
# Find all usages
cd zed-sheet-lsp/zed-sheets-lsp && grep -n "Sidecar" src/*.rs

# Replace imports
sed -i 's/use.*LocalSidecar/use nustage::sidecar::SidecarFile as Sidecar/' src/*.rs

# Update type annotations and function signatures
```

### Phase 2: Document Type Contracts (Medium Priority)

Create a dedicated document for each project's public API surface:

#### Create: `nustage/docs/CONTRACTS.md`

Document which types are stable contracts vs internal implementation:

```markdown
# Nustage Public Contracts

These types and their serialization formats are stable and may be used by other projects.

## Stable Types (Do Not Break)

### SidecarFile
- Format: `.nustage.json` schema must maintain backward compatibility
- Breaking changes require version bump and migration path
- Exported from: `nustage::sidecar::SidecarFile`

### TransformationStep / StepType
- Pipeline step model for transformation semantics
- Used by editor integration (zed-sheet-lsp) for diagnostics/completion
- Breaking changes affect all consumers

## Internal Types (May Change)

Types not listed here are considered internal and may change without notice.
```

#### Create: `git-sheets/docs/CONTRACTS.md`

Document snapshot format stability guarantees.

### Phase 3: Add Integration Tests (Low Priority)

Create cross-project verification tests:

```rust
// zed-sheet-lsp/tests/integration/sidecar_integration.rs

#[test]
fn test_editor_uses_nustage_sidecar_types() {
    // Verify that zed-sheet-lsp uses nustage::sidecar::SidecarFile, not a duplicate
    use nustage::sidecar::{SidecarFile, SidecarMetadata};
    
    let sidecar = SidecarFile::new("test.csv");
    
    // Can serialize/deserialize correctly
    let json = serde_json::to_string(&sidecar).unwrap();
    let parsed: SidecarFile = serde_json::from_str(&json).unwrap();
    
    assert_eq!(parsed.source, "test.csv");
}

#[test] 
fn test_sidecar_format_stability() {
    // Verify that sidecar format can be read by other tools (e.g., git-sheets)
    use std::fs;
    use tempfile::tempdir;
    
    let dir = tempdir().unwrap();
    let path = dir.path().join("test.nustage.json");
    
    let sidecar = SidecarFile::new("data.csv");
    fs::write(&path, serde_json::to_string_pretty(&sidecar).unwrap()).unwrap();
    
    // Can be read as plain JSON by any tool
    let content = fs::read_to_string(&path).unwrap();
    assert!(content.contains("\"source\""));
}
```

## Verification Checklist

After completing migration:

- [ ] `zed-sheet-lsp` no longer defines its own `Sidecar` type
- [ ] All sidecar loading uses `nustage::sidecar::SidecarFile`
- [ ] Integration tests pass (run from workspace or manually)
- [ ] CI verifies no duplicate types exist in cross-project builds
- [ ] Documentation updated to reflect new dependency direction

## Rollback Plan

If migration causes issues:

1. Keep local `Sidecar` type with deprecation warning
2. Add conversion methods between local and canonical types
3. Migrate incrementally, one module at a time
4. Maintain both formats until all consumers update

**Warning:** This rollback pattern should be temporary. Long-term goal is single source of truth.

## Related Documents

- [`docs/integration/README.md`](./README.md) - Integration overview
- [`docs/integration/dependency-config.md`](./dependency-config.md) - Dependency management guide  
- [`docs/STACK_BOUNDARIES.md`](../docs/STACK_BOUNDARIES.md) - Feature placement rules
- [`nustage/docs/integration/AUDIT.md`](../../nustage/docs/integration/AUDIT.md) - Mirror document

---

**Last updated:** 2025-01-XX  
**Status:** Active - Phase 1 in progress  
**Owner:** Project maintainers (collaborative effort)