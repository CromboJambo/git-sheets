# Changelog

All notable changes to git-sheets will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.2] - 2026-03-01
### Fixed
- Fixed type mismatch errors in diff module related to row comparison
- Fixed unused variable warning in CLI diff output
- Resolved compilation warnings related to unused imports

### Documentation
- Updated README.md with cleaner formatting
- Removed obsolete storage format considerations section
- Prepared project structure for GitHub and crates.io release

### Technical
- Updated version to 0.1.2 in Cargo.toml
- Improved row comparison logic using helper function
- Enhanced diff calculation accuracy

### Known Issues
- None

## [0.1.1] - 2025-12-15

### Added
- Initial release of git-sheets
- Core snapshot functionality with CSV import
- Per-header hashing for integrity verification
- Diff calculation between snapshots
- CLI interface with subcommands
- Support for primary key specification
- Auto-commit integration placeholder
- Git integration capabilities

### Features
- TOML-based snapshot storage
- JSON-based diff output
- Git-style unified diff format
- Snapshot verification system
- History logging functionality
- Status checking utilities

### Dependencies
- chrono 0.4
- serde 1.0
- serde_json 1.0
- sha2 0.10
- csv 1.4
- toml 0.7
- clap 4.5
- git2 0.20

[Unreleased]
