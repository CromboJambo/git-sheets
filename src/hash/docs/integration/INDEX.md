# Trifecta Integration Documentation Index

Welcome to the integration documentation for `git-sheets`, `nustage`, and `zed-sheet-lsp`. This collection of documents explains how these three projects relate, compose, and remain independently useful.

## Quick Start

**New to the stack?** Start here:
1. [`EXECUTIVE_SUMMARY.md`](./EXECUTIVE_SUMMARY.md) — One-page overview of the architecture
2. [`QUICK_REFERENCE.md`](./QUICK_REFERENCE.md) — Cheatsheet for developers

**Ready to contribute?** Read these in order:
1. [`README.md`](./README.md) — Full integration guide with patterns and examples
2. [`AUDIT.md`](./AUDIT.md) — Current type overlaps and migration paths
3. [`dependency-config.md`](./dependency-config.md) — How to configure dependencies

**Want to see it in action?** Check:
- [`examples/`](./examples/) — Practical workflow scripts you can run

---

## Document Catalog

### Core Architecture Documents

| Document | Purpose | Audience | Length |
|----------|---------|----------|--------|
| [README.md](./README.md) | Complete integration guide with patterns, ownership matrix, and examples | Everyone | Long |
| [EXECUTIVE_SUMMARY.md](./EXECUTIVE_SUMMARY.md) | High-level overview for quick reference | Project leads, new contributors | Short |
| [QUICK_REFERENCE.md](./QUICK_REFERENCE.md) | Developer cheatsheet with decision trees and type imports | Daily developers | Reference |

### Technical Deep Dives

| Document | Purpose | Audience | Length |
|----------|---------|----------|--------|
| [AUDIT.md](./AUDIT.md) | Type overlap analysis, migration paths for eliminating duplication | Core maintainers | Medium-Long |
| [dependency-config.md](./dependency-config.md) | Cargo workspace strategy, dependency direction rules | Build engineers | Medium |

### Practical Resources

| Document | Purpose | Audience | Length |
|----------|---------|----------|--------|
| [examples/README.md](./examples/README.md) | Overview of available workflow scripts | All users | Short |
| [examples/full_stack_daily_development.sh](./examples/full_stack_daily_development.sh) | Complete end-to-end workflow example | Everyone | Script |

---

## How to Use This Documentation

### As a New Contributor

1. Read [`EXECUTIVE_SUMMARY.md`](./EXECUTIVE_SUMMARY.md) for the big picture
2. Skim [`README.md`](./README.md) for integration patterns
3. Bookmark [`QUICK_REFERENCE.md`](./QUICK_REFERENCE.md) for quick lookups
4. When adding features, consult the decision tree to find where code belongs

### As a Core Maintainer

1. Review [`AUDIT.md`](./AUDIT.md) before accepting new types or formats
2. Ensure any breaking changes follow the migration path in [`dependency-config.md`](./dependency-config.md)
3. Update these documents when architecture evolves
4. Run examples from [`examples/`](./examples/) to verify integration integrity

### As a User of One Project

1. Check if you need other projects for your workflow
2. Read relevant patterns in [`README.md`](./README.md) (e.g., "Version Control Only" section)
3. Use example scripts as templates for your own workflows

---

## External References

These documents complement the integration docs but live outside this directory:

| Document | Location | Purpose |
|----------|----------|---------|
| Stack Boundaries | `../docs/STACK_BOUNDARIES.md` | Feature placement rules and bucket tests |
| Nustage Integration | `../../nustage/docs/integration/README.md` | Boundary between nustage and zed-sheet-lsp |

---

## Document Versioning

| Version | Date | Changes |
|---------|------|---------|
| 1.0 | 2026-03-08 | Initial documentation set created |

**Status:** Active — These documents