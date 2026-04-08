# Changelog - dustdoc (DPL Documentation Generator)

All notable changes to dustdoc are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased] - 2026-03-09

### Changed

- Migrated `dustdoc` from Rust crate layout to Dust-native workspace layout.
- Replaced `Cargo.toml` + `Cargo.lock` + `crates/dustdoc` with `State.toml` + `src/main.ds`.
- Collapsed runtime build profile to compiler-compatible single-file top-level grammar (`K main`).
- Updated CI and repository documentation to reflect the Dust-native structure.

## [0.2.0] - 2026-02-12 (DPL v0.2)

### Added

- **DPL v0.2 Compliance**: Full support for v0.2 specification
- Documentation generation for K Regime v0.2 code
- Full support for doc comments on all new constructs
- Support for v0.2 keywords: `alloc`, `free`, `spawn`, `join`, `mutex_new`, `mutex_lock`, `mutex_unlock`, `open`, `read`, `write`, `close`, `io_read`, `io_write`, `mmio_read`, `mmio_write`, `unsafe`
- Support for v0.2 resource types: `Mem`, `Thread<T>`, `Mutex`, `File`, `Port`, `Device`, `Ptr`
- Unsafe block documentation with safety badge display
- Resource type annotations in generated documentation

### Changed

- Enhanced HTML output styling
- Improved navigation for larger projects
- Better search functionality

### Fixed

- Documentation generation for complex types
- Cross-reference linking issues

## [0.1.0] - 2026-02-12

### Added

- Initial documentation generator
- Basic HTML output
- Simple doc comment parsing

### Known Issues

- Limited documentation for v0.1 features only

---

Copyright (c) 2026 Dust LLC
