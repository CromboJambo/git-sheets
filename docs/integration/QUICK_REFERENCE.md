# Trifecta Quick Reference Card

**One-page cheatsheet for integrating `git-sheets`, `nustage`, and `zed-sheet-lsp`**

---

## Ownership at a Glance

| Concern | Owner | Can Others Use? |
|---------|-------|-----------------|
| Pipeline semantics | **nustage** | Yes (read-only) |
| Sidecar format | **nustage** | Yes (read-only for `zed-sheet-lsp`) |
| Editor UX in Zed | **zed-sheet-lsp** | N/A |
| Snapshots/diffs | **git-sheets** | No integration needed |
| History/audit trail | **git-sheets** | All projects can snapshot outputs |

---

## Decision Tree: Where Does This Feature Go?

```
Is this about transformation semantics?
  └─ YES → nustage (canonical types, sidecar format)

Is this about Zed editor UX?
  └─ YES → zed-sheet-lsp (LSP, hover, completion, diagnostics)

Is this about recording state over time?
  └─ YES → git-sheets (snapshots, diffs, hashes)

Creates duplicate definition of existing type?
  └─ STOP! → Use canonical type from owner project
```

---

## File Format Contracts

### `.nustage.json` - Pipeline Sidecar

**Owner:** `nustage`  
**Consumers:** `zed-sheet-lsp` (read)

```json
{
  "version": 1,
  "source": "data.csv",
  "pipeline": [
    {"step_id": "filter_001", "type": "filter", "column": "Revenue", "condition": "> 1000"}
  ],
  "schema_history": {},
  "metadata": {
    "created_at": "2025-01-15T10:00:00Z"
  }
}
```

### Snapshot Format (git-sheets)

**Owner:** `git-sheets`  
**Location:** `snapshots/<name>_<timestamp>.toml`

```toml
id = "1736942400-abc123"
timestamp = "2025-01-15T12:00:00Z"
message = "Post-transformation state"

[table]
headers = ["Revenue", "Cost"]
primary_key = [0]

[[table.rows]]
row = ["1000", "800"]

[hashes]
table_hash = "e3b0c44..."
```

---

## Dependency Rules

| From | To | Allowed? | Notes |
|------|-----|----------|-------|
| `nustage` → any other | ❌ NO | Never depend on others |
| `git-sheets` → any other | ❌ NO | Never depend on others |
| `zed-sheet-lsp` → `nustage` | ✅ YES | Editor consumes intent |

**File system communication:** All projects communicate via shared file formats (sidecar, snapshots).

---

## Common Patterns

### Pattern 1: Open File with Sidecar Diagnostics

```bash
# User opens in Zed
zed sales.csv

# zed-sheet-lsp detects .nustage.json → loads sidecar via nustage types
# Shows diagnostics/completion based on pipeline model
```

**Flow:** `zed-sheet-lsp` (file watcher) → `nustage::sidecar::load()` → surface in editor

### Pattern 2: Pipeline-Aware Rename

```bash
# User triggers rename "Revenue" → "Gross Revenue" in Zed
# zed-sheet-lsp calls nustage for safe rename across all steps
# .nustage.json updated atomically
# Optional: git-sheets snapshots before/after
```

**Flow:** `zed-sheet-lsp` (trigger) → `nustage::transformations::rename_column()` → update sidecar

### Pattern 3: Snapshot Pipeline Output

```bash
# Execute pipeline via nustage
nustage process sales.csv

# Capture result with git-sheets
git-sheets snapshot processed_sales.csv -m "After transformation"
```

**Flow:** `nustage` (execution) → output file → `git-sheets` (snapshot)

---

## Type Imports Reference

### From nustage (for zed-sheet-lsp consumers)

```rust
use nustage::sidecar::{SidecarFile, SidecarMetadata};
use nustage::transformations::{TransformationStep, ColumnSchema, StepType};
```

### From git-sheets (standalone use)

```rust
use gitsheets::{Snapshot, Table, TableHashes, DiffSummary};
use gitsheets::{GitSheetsError, Result};
```

---

## CLI Commands Quick Reference

### git-sheets

```bash
git-sheets init                           # Initialize repository
git-sheets snapshot data.csv -m "msg"    # Create snapshot
git-sheets diff snap1.toml snap2.toml    # Compare snapshots
git-sheets log                           # Show history
git-sheets verify snap.toml              # Verify integrity
```

### nustage

```bash
nustage init data.csv                    # Initialize sidecar
nustage add-step filter Revenue > 1000   # Add transformation step
nustage process data.csv                 # Execute pipeline
nustage --tui                            # Interactive TUI mode
```

---

## Anti-Patterns to Avoid

| ❌ Don't | ✅ Do Instead |
|----------|---------------|
| Define local `Sidecar` type | Use `nustage::sidecar::SidecarFile` |
| Implement formula parsing in LSP | Delegate to `nustage` for semantics |
| Create duplicate sidecar format | Use `.nustage.json` as single source of truth |
| Let editor own transformation rules | Editor surfaces `nustage` types only |
| Mix snapshot logic with pipeline logic | Keep history separate from intent |

---

## Testing Independence

### Verify Each Project Works Standalone

```bash
# git-sheets independence
cd git-sheets && cargo test

# nustage independence  
cd nustage && cargo test

# zed-sheet-lsp without full stack (if possible)
cd zed-sheet-lsp && cargo test --package zed-sheets-lsp
```

### Integration Tests (Future)

```bash
# From workspace root or CI
cargo test --workspace sidecar_integration
cargo test --workspace snapshot_pipeline_output
```

---

## When Breaking Changes Are Needed

1. **Identify affected consumers** - Check who uses this type/format
2. **Update documentation** - Note breaking change in `docs/integration/`
3. **Add migration path** - Provide version bump + old format support if needed
4. **Coordinate releases** - Notify other project maintainers

---

## Related Documents

| Document | Purpose | Location |
|----------|---------|----------|
| Full Integration Guide | Complete architecture overview | `README.md` |
| Type Audit & Migration | Duplicate type analysis | `AUDIT.md` |
| Dependency Config | Cargo workspace strategy | `dependency-config.md` |
| Example Workflows | Practical scripts | `examples/` |
| Stack Boundaries | Feature placement rules | `../../docs/STACK_BOUNDARIES.md` |

---

## Quick Troubleshooting

**Issue:** Editor shows wrong diagnostics  
**Fix:** Verify `zed-sheet-lsp` uses `nustage::sidecar::SidecarFile`, not local type

**Issue:** Sidecar format drift between projects  
**Fix:** Check ownership - only `nustage` should modify `.nustage.json`

**Issue:** Circular dependency error in Cargo  
**Fix:** Review dependency direction - `zed-sheet-lsp → nustage` is OK, reverse is not

---

**Keep this card handy. For detailed patterns, see the full integration documentation.**