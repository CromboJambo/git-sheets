# Trifecta Integration Guide

This directory documents how `git-sheets`, `nustage`, and `zed-sheet-lsp` work together as an integrated stack while maintaining their individual identities.

## The Stack Overview

```
┌─────────────────────────────────────────────────────────────┐
│                    Zed Editor                              │
│  ┌───────────────────────────────────────────────────────┐  │
│  │           zed-sheet-lsp (Editor Integration)         │  │
│  │  - Hover, completion, diagnostics for tabular data   │  │
│  │  - Pipeline-aware code actions                       │  │
│  └───────────────────────┬───────────────────────────────┘  │
│                          │ LSP Protocol                     │
├──────────────────────────┼──────────────────────────────────┤
│                          ▼                                   │
│  ┌───────────────────────────────────────────────────────┐  │
│  │           nustage (Intent Engine)                    │  │
│  │  - Canonical pipeline model                          │  │
│  │  - Sidecar management (.nustage.json)                │  │
│  │  - Transformation execution                          │  │
│  └───────────────────────┬───────────────────────────────┘  │
│                          │ File System                      │
├──────────────────────────┼──────────────────────────────────┤
│                          ▼                                   │
│  ┌───────────────────────────────────────────────────────┐  │
│  │          git-sheets (History Manager)                │  │
│  │  - Snapshots of table state                          │  │
│  │  - Diff computation                                  │  │
│  │  - Integrity verification                            │  │
│  └───────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────┘
```

## Responsibility Matrix

| Concern | Primary Owner | Secondary Consumers | Notes |
|---------|--------------|---------------------|-------|
| Pipeline definition semantics | `nustage` | - | Canonical transformation model |
| Sidecar format & persistence | `nustage` | `zed-sheet-lsp` (read-only) | `.nustage.json` is single source of truth |
| LSP protocol handling | `zed-sheet-lsp` | - | Zed-specific integration |
| Hover/completion/diagnostics | `zed-sheet-lsp` | `nustage` (provides data) | Surface `nustage` intent in editor |
| Snapshots & versioning | `git-sheets` | - | Immutable table state records |
| Diff computation | `git-sheets` | - | Before/after comparison |
| Integrity hashing | `git-sheets` | - | SHA-256 verification |

## Integration Patterns

### Pattern 1: Editor Opens Tabular File with Sidecar

```
User Action → zed-sheet-lsp detects .nustage.json → 
zed-sheet-lsp loads sidecar via nustage library → 
Diagnostics/completion based on pipeline model
```

**Where it happens:** `zed-sheet-lsp`  
**Dependencies:** `nustage` (library)  
**No changes to:** `git-sheets`

### Pattern 2: User Makes Changes, Wants Version Control

```
User Action → git-sheets snapshot → 
git-sheets hashes table state → 
Stores in snapshots/ with integrity verification
```

**Where it happens:** `git-sheets` CLI  
**Dependencies:** None (standalone)  
**Can work without:** `nustage`, `zed-sheet-lsp`

### Pattern 3: Audit Trail for Pipeline Changes

```
User Action → git-sheets snapshot (before change) → 
User modifies .nustage.json via editor → 
git-sheets snapshot (after change) → 
git-sheets diff shows pipeline evolution
```

**Where it happens:** `git-sheets` + user workflow  
**Dependencies:** None between repos  
**Composed at:** CLI/user level

### Pattern 4: Pipeline-Aware Rename in Editor

```
User Action: rename column "Revenue" to "Gross Revenue" → 
zed-sheet-lsp triggers rename → 
Calls nustage library for safe rename across pipeline steps → 
Updates .nustage.json → 
git-sheets can snapshot before/after as audit trail
```

**Where it happens:** `zed-sheet-lsp` (trigger) + `nustage` (execution)  
**Optional integration:** `git-sheets` snapshots the result  

## Shared Data Formats

### `.nustage.json` (Canonical Pipeline Sidecar)

Owned by: `nustage`  
Consumed by: `zed-sheet-lsp` (read), `git-sheets` (audit snapshot)

```json
{
  "version": 1,
  "source": "data.csv",
  "pipeline": [
    { "step_id": "filter_revenue", "type": "filter", "column": "Revenue", "condition": "> 1000" },
    { "step_id": "add_margin", "type": "add_column", "name": "Margin", "expr": "@Revenue - @Cost" }
  ],
  "schema_history": {
    "filter_revenue": [{"name": "Revenue", "type": "f64"}, {"name": "Cost", "type": "f64"}]
  },
  "metadata": {
    "created_at": "2025-01-15T10:00:00Z",
    "modified_at": "2025-01-15T12:00:00Z"
  }
}
```

### Snapshot Format (git-sheets)

Owned by: `git-sheets`  
Can snapshot: Any table state, including pipeline outputs

```toml
id = "1736942400-abc123"
timestamp = "2025-01-15T12:00:00Z"
message = "After adding margin column"

[table]
headers = ["Revenue", "Cost", "Margin"]
primary_key = [0]

[[table.rows]]
row = ["1000", "800", "200"]

[hashes]
table_hash = "e3b0c44..."
```

## Dependency Directions

```
zed-sheet-lsp ────────┐
                      ├──→ nustage (library dependency)
git-sheets ───────────┘

nustage ────────────── (no dependencies on other two)
git-sheets ─────────── (no dependencies on other two)
```

**Key rule:** `nustage` and `git-sheets` have no mutual dependencies. They compose at the workflow level.

## Feature Placement Checklist

When adding a new feature, ask:

1. **Is this defining transformation semantics?** → `nustage`
2. **Is this about editor UX (hover, completion, rename)?** → `zed-sheet-lsp`
3. **Is this about recording history or comparing states?** → `git-sheets`
4. **Does this create duplicate truth?** → Redesign needed

## Workflow Examples

### Daily Development with Full Stack

```bash
# 1. Open data file in Zed (with zed-sheet-lsp active)
zed sales.csv

# Editor shows pipeline diagnostics from .nustage.json

# 2. Make changes via editor code actions or direct edit

# 3. Snapshot before risky operation
git-sheets snapshot -m "Before bulk update" data.csv

# 4. Run transformation (via nustage CLI or Zed preview)
nustage process sales.csv

# 5. Snapshot result for audit trail
git-sheets snapshot processed_data.csv -m "Post-transformation state"

# 6. Review what changed
git-sheets diff snapshots/data_*.toml
```

### Minimal Setup (Just Version Control)

```bash
# No Zed, no pipeline engine needed
git-sheets init
git-sheets snapshot important_data.csv -m "Before macro run"
# ... do work in Excel ...
git-sheets snapshot important_data.csv -m "After macro"
git-sheets diff snapshots/*.toml  # See what changed
```

### Pipeline-First Workflow (No Version Control)

```bash
# Focus on transformations, not history
nustage init sales.csv
nustage add-step filter Revenue > 1000
nustage add-step add_column Margin = @Revenue - @Cost
nustage process sales.csv  # Execute pipeline
```

## Anti-Patterns to Avoid

### ❌ Don't: Duplicate Sidecar Format

**Bad:** Create `.zed-sheets.json` alongside `.nustage.json` with different schema  
**Good:** Use `.nustage.json` as single source of truth, let `zed-sheet-lsp` read it

### ❌ Don't: Move Core Logic to Editor

**Bad:** Implement formula parsing in `zed-sheet-lsp`  
**Good:** Let `nustage` define expression semantics, have editor surface them

### ❌ Don't: Mix History with Intent

**Bad:** Use git-sheets snapshots as the canonical pipeline model  
**Good:** Snapshots record state; sidecar defines transformations

## Testing Integration Points

### Unit Tests (Per-Repo)

Each repo should have tests that work independently:
- `nustage`: Pipeline validation, sidecar serialization
- `zed-sheet-lsp`: LSP protocol handling, diagnostic generation
- `git-sheets`: Snapshot creation, diff computation

### Integration Tests (Cross-Repo)

Run from a workspace root or CI:

```bash
# Test: Editor can read nustage sidecar
cargo test --workspace sidecar_integration

# Test: Snapshot captures pipeline output correctly  
cargo test --workspace snapshot_pipeline_output

# Test: Rename propagates through pipeline
cargo test --workspace rename_propagation
```

## Migration Path for Existing Code

### Phase 1: Clarify Boundaries (Current)
- ✅ Documentation exists (`STACK_BOUNDARIES.md`)
- ✅ Mental model established
- ⚠️ Some code still overlaps

### Phase 2: Library Extraction
- [ ] Extract `nustage` pipeline types into dedicated lib crate
- [ ] Have `zed-sheet-lsp` depend on extracted types (not copy)
- [ ] Ensure `git-sheets` has no accidental dependencies

### Phase 3: Workflow Integration
- [ ] Add CLI commands that compose all three
- [ ] Document recommended workflows in user guides
- [ ] Create example scripts showing full stack usage

## Questions This Architecture Answers

**Q: Can I use just git-sheets for Excel version control?**  
A: Yes. No other projects required.

**Q: Can I use nustage without Zed editor?**  
A: Yes. CLI and TUI work standalone.

**Q: What if I want pipeline-aware editing but no history tracking?**  
A: Use `nustage` + `zed-sheet-lsp`. git-sheets is optional.

**Q: How do changes in one repo affect others?**  
A: Carefully defined via library dependencies and file formats. See "Dependency Directions" above.

**Q: Can I swap out components later?**  
A: Yes, as long as the contracts (sidecar format, snapshot format) remain stable.