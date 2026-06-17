# Trifecta Integration - Executive Summary

**Version:** 1.0  
**Status:** Active Documentation  
**Last Updated:** 2025-01-XX

## The Three-Pillar Stack at a Glance

| Project | Role | Standalone Value | Key Output |
|---------|------|------------------|------------|
| **nustage** | Intent Engine | CLI/TUI for tabular data transformation | `.nustage.json` sidecar, processed data |
| **zed-sheet-lsp** | Editor Integration | Language tooling for TSV/tabular files in Zed | LSP server, hover/completion/diagnostics |
| **git-sheets** | History Manager | Git-style version control for spreadsheets | Immutable snapshots, diff reports |

## Core Principle: Intent vs Interaction vs History

```
┌─────────────┐      ┌──────────────┐      ┌─────────────┐
│   intent    │      │  interaction │      │   history   │
│  nustage    │◄────►│ zed-sheet-lsp│─────►│ git-sheets  │
│             │      │              │      │             │
└─────────────┘      └──────────────┘      └─────────────┘
```

**Decision Tree for New Features:**
1. Defines transformation semantics? → **nustage**
2. Controls editor UX in Zed? → **zed-sheet-lsp**
3. Records or compares state over time? → **git-sheets**

## Dependency Direction (Critical Rule)

```
        ┌─────────────────┐
        │  git-sheets     │ (independent)
        └─────────────────┘
                  ▲
                  │ no dependency
        ┌──────────────────┐      ┌──────────────┐
        │   nustage        │◄────►│ zed-sheet-lsp│
        │  (no deps)       │      │ → uses nustage│
        └──────────────────┘      └──────────────┘
```

**Rules:**
- `nustage` has NO dependencies on other projects
- `git-sheets` has NO dependencies on other projects  
- `zed-sheet-lsp` MAY depend on `nustage` (editor consumes intent)
- Cross-project communication via file formats (sidecar, snapshots)

## Critical Issue: Type Duplication Alert 🔴

**Problem:** `zed-sheet-lsp` defines its own `Sidecar` type that duplicates `nustage::sidecar::SidecarFile`.

**Impact:** Creates second source of truth for pipeline intent—violates core architecture principle.

**Solution:** Remove local definition, import canonical types from nustage:
```rust
// In zed-sheet-lsp/zed-sheets-lsp/src/sidecar.rs
use nustage::sidecar::{SidecarFile, SidecarMetadata};
// Use SidecarFile directly instead of defining local type
```

**See:** [`AUDIT.md`](./AUDIT.md) for detailed migration path.

## Recommended Workflows

### Full Stack (All Three Projects)
```bash
# 1. Open in Zed with LSP diagnostics from .nustage.json
zed sales.csv

# 2. Snapshot before risky operation
git-sheets snapshot -m "Before macro" data.csv

# 3. Run transformation pipeline
nustage process data.csv

# 4. Audit trail via diffs
git-sheets diff snapshots/*.toml
```

### Version Control Only (Standalone)
```bash
git-sheets init
git-sheets snapshot important.csv -m "Before work"
# ... do Excel work ...
git-sheets snapshot important.csv -m "After work"
git-sheets diff snapshots/*.toml  # See what changed
```

### Pipeline-First (Standalone)
```bash
nustage init data.csv
nustage add-step filter Revenue > 1000
nustage process data.csv
```

## Integration Points by Feature Type

| Feature | Owner | Consumer/Integrator | Notes |
|---------|-------|---------------------|-------|
| Pipeline semantics | `nustage` | - | Canonical transformation model |
| Sidecar format | `nustage` | `zed-sheet-lsp` (read) | `.nustage.json` is single source of truth |
| LSP protocol | `zed-sheet-lsp` | - | Zed-specific integration only |
| Hover/completion | `zed-sheet-lsp` | `nustage` (data source) | Surface intent in editor |
| Snapshots/diffs | `git-sheets` | - | Immutable state records |
| Rename propagation | `nustage` + `zed-sheet-lsp` | - | Editor triggers, nustage executes |

## Migration Status

### Phase 1: Clarify Boundaries ✅ COMPLETE
- [x] Documentation created (`docs/integration/`)
- [x] Mental model established and shared
- [x] Type overlap audit completed
- [x] Example workflows documented

### Phase 2: Library Extraction ⏳ IN PROGRESS
- [ ] Remove `zed-sheet-lsp::Sidecar` duplication
- [ ] Add `nustage` dependency to `zed-sheet-lsp`
- [ ] Update all sidecar usage sites
- [ ] Verify no duplicate type definitions

### Phase 3: Workflow Integration ⏳ FUTURE
- [ ] Unified workspace configuration (opt-in)
- [ ] CLI wrapper for full-stack workflows
- [ ] Comprehensive integration test suite

## Quick Reference Links

| Document | Purpose | Location |
|----------|---------|----------|
| **Main Integration Guide** | Complete architecture overview | [`README.md`](./README.md) |
| **Type Audit** | Overlap analysis & migration paths | [`AUDIT.md`](./AUDIT.md) |
| **Dependency Config** | Cargo workspace strategy | [`dependency-config.md`](./dependency-config.md) |
| **Example Workflows** | Practical usage scripts | [`examples/`](./examples/) |
| **Stack Boundaries** | Feature placement rules | `../../docs/STACK_BOUNDARIES.md` |

## Key Takeaways

1. **Each project is valuable standalone** — none require the others to function.

2. **Clear ownership boundaries** prevent feature creep and duplication:
   - `nustage` = intent (what transformations mean)
   - `zed-sheet-lsp` = interaction (how users edit in Zed)
   - `git-sheets` = history (what changed over time)

3. **File formats are the contracts** — `.nustage.json` and snapshot format stability is critical for future extensibility.

4. **One source of truth per concept** — especially critical for sidecar format to avoid drift between projects.

5. **Integration happens at workflow level** — not all features need cross-project dependencies; CLI composition often suffices.

## Next Steps for Contributors

When adding new code:
1. Check the ownership matrix before implementing
2. Verify no type duplication exists (see [`AUDIT.md`](./AUDIT.md))
3. Update this documentation if you change any contracts
4. Add tests that verify independence where applicable

---

**Questions?** See detailed guides in `docs/integration/` or open an issue with the integration label.