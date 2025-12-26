# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Planned
- Python binding (PyPI)
- Java/Kotlin binding (Maven)
- .NET binding (NuGet)
- iOS/macOS binding (CocoaPods/SPM)

---

## [0.1.0] - 2025-01-XX

### Added
- Initial release
- Rust core with Bitap algorithm for fuzzy matching
- Bloom filter pre-filtering for O(1) rejection
- WebAssembly build (~3KB gzipped)
- TypeScript/JavaScript wrapper
- Zero-copy JS ↔ WASM communication
- Case-insensitive search
- Configurable threshold, maxErrors, maxResults
- Schema support for field weights
- Record add/remove/reset operations
- Statistics API (recordCount, stringPoolUsed, availableMemory)

### Performance
- Sub-millisecond search on 10K+ records
- 80-95% of records rejected by bloom filter before Bitap
- Zero-allocation memory architecture

### Technical
- `no_std` Rust implementation
- Static memory pools (4MB string pool, 100K records max)
- Adaptive maxErrors based on pattern length to prevent false positives

---

## Version History

| Version | Date | Highlights |
|---------|------|------------|
| 0.1.0 | 2025-01 | Initial release with WASM/JS binding |

[Unreleased]: https://github.com/RafaCalRob/FlashFuzzy/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/RafaCalRob/FlashFuzzy/releases/tag/v0.1.0
