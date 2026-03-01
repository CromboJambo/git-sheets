# git-sheets v0.1.2 Release Notes

## 🎉 Release Overview

**Version:** 0.1.2  
**Release Date:** December 15, 2025  
**Status:** Stable

git-sheets v0.1.2 brings important bug fixes and improvements to make the tool more reliable and ready for production use. This release focuses on fixing type comparison issues in the diff module and cleaning up compilation warnings.

## ✨ What's New

### Bug Fixes
- **Fixed Row Comparison Type Errors**: Resolved type mismatch errors when comparing rows in the diff module. The implementation now uses a proper helper function for element-by-element comparison.
- **Fixed Unused Variable Warning**: Removed unused `col` variable in the git-style diff output format.
- **Cleaned Up Compilation Warnings**: Eliminated unused import warnings and other compiler warnings.

### Improvements
- **Enhanced Diff Calculation**: Improved accuracy of diff calculations by using proper row comparison logic.
- **Better Code Organization**: Added helper function for row existence checking to improve code maintainability.
- **Documentation Updates**: Updated README.md with cleaner formatting and removed obsolete content.

## 📋 Installation

### Quick Install

```bash
# Install from source
cargo install --path .

# Or build for local use
cargo build --release
```

### Prerequisites
- Rust toolchain (stable) - [rustup.rs](https://rustup.rs/)
- Git (optional, but recommended)

## 🚀 Quick Start

```bash
# Initialize a repository
git-sheets init

# Create your first snapshot
git-sheets snapshot data.csv -m "Initial data"

# Compare snapshots
git-sheets diff snapshots/data_001.toml snapshots/data_002.toml

# Verify integrity
git-sheets verify snapshots/data_001.toml
```

## 🔧 Migration Guide

### From v0.1.1
No migration needed! The changes in v0.1.2 are backward compatible with existing snapshots and configurations.

### Breaking Changes
None

## 📚 Documentation

- **[README.md](README.md)** - Comprehensive user guide
- **[CHANGELOG.md](CHANGELOG.md)** - Detailed change history
- **[LICENSE](LICENSE)** - License information (AGPL-3.0-or-later)

## 🛠️ Technical Details

### Key Features
- CSV import and snapshot creation
- Per-header hashing for integrity verification
- Diff calculation with multiple formats (text, JSON, git-style)
- Primary key support for row identification
- Snapshot verification system
- History logging and status checking

### Dependencies
- chrono 0.4
- serde 1.0
- serde_json 1.0
- sha2 0.10
- csv 1.4
- toml 0.7
- clap 4.5
- git2 0.20

## 🐛 Known Issues

None reported in v0.1.2.

## 🙏 Contributors

Special thanks to:
- **Dustin Grover** - Original creator and maintainer
- **All contributors** - Thanks for your feedback and bug reports

## 📝 Release Notes Format

This release follows the [Keep a Changelog](https://keepachangelog.com/en/1.0.0/) format and adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 🔗 Links

- **Project Home**: https://github.com/CromboJambo/git-sheets
- **Issues**: https://github.com/CromboJambo/git-sheets/issues
- **Discussion**: https://github.com/CromboJambo/git-sheets/discussions

## 📄 License

AGPL-3.0-or-later - See [LICENSE](LICENSE) for details.

---

**Remember**: This isn't about making spreadsheets fancy. It's about making them *safe*.