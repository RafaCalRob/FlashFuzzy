# flash-fuzzy-core

High-performance fuzzy search engine core using Bitap algorithm with bloom filter pre-filtering.

[![Crates.io](https://img.shields.io/crates/v/flash-fuzzy-core.svg)](https://crates.io/crates/flash-fuzzy-core)
[![Documentation](https://docs.rs/flash-fuzzy-core/badge.svg)](https://docs.rs/flash-fuzzy-core)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)

## Features

- **Zero dependencies** - Pure Rust implementation
- **`no_std` compatible** - Works in embedded and WASM environments
- **Blazing fast** - Bitap algorithm with bit-parallel operations
- **Smart pre-filtering** - 64-bit bloom filter rejects non-matches in O(1)
- **Typo tolerant** - Configurable edit distance (0-3 errors)

## Quick Start

```rust
use flash_fuzzy_core::{BitapSearcher, BloomFilter, bitap};

// Create a searcher for a pattern
let searcher = BitapSearcher::new(b"keyboard");

// Check bloom filter first (O(1) rejection)
let text = b"Mechanical Keyboard Pro";
let text_bloom = BloomFilter::from_text(text);

if text_bloom.might_contain(searcher.bloom()) {
    // Run fuzzy search with max 2 errors
    if let Some(match_result) = searcher.search(text, 2) {
        let score = bitap::compute_score(
            match_result.errors,
            searcher.pattern_len() as u32,
            match_result.end_pos
        );
        println!("Found match with {} errors, score: {}", match_result.errors, score);
    }
}
```

## How It Works

### 1. Bloom Filter Pre-filtering

Before running expensive fuzzy matching, we check if the search pattern could possibly exist using a 64-bit bloom filter:

```
Text:    "Wireless Laptop Pro" → bloom bits: 01001010...
Pattern: "laptop"              → bloom bits: 00001010...

Check: (text_bloom & pattern_bloom) == pattern_bloom
       If false → definitely no match, skip (80-95% of records rejected)
       If true  → might match, run Bitap
```

### 2. Bitap Algorithm

The Bitap (Shift-Or) algorithm uses bit-parallel operations for approximate string matching:

```rust
// Each error level tracks match state as a bitmask
R[0] = exact matches
R[1] = matches with 1 error (substitution/insertion/deletion)
R[2] = matches with 2 errors
...
```

## API

### `BitapSearcher`

```rust
impl BitapSearcher {
    /// Create a new searcher from a pattern (max 32 bytes)
    pub fn new(pattern: &[u8]) -> Self;

    /// Get the pattern's bloom filter
    pub fn bloom(&self) -> BloomFilter;

    /// Get the pattern length
    pub fn pattern_len(&self) -> usize;

    /// Search for pattern in text with up to max_errors
    pub fn search(&self, text: &[u8], max_errors: u32) -> Option<SearchMatch>;
}
```

### `BloomFilter`

```rust
impl BloomFilter {
    /// Create bloom filter from text
    pub fn from_text(text: &[u8]) -> Self;

    /// Check if pattern might be contained
    pub fn might_contain(&self, pattern_bloom: BloomFilter) -> bool;
}
```

### `SearchMatch`

```rust
pub struct SearchMatch {
    pub errors: u32,    // Number of errors (edit distance)
    pub end_pos: usize, // End position of match in text
}
```

## Scoring

```rust
use flash_fuzzy_core::bitap::compute_score;

// Score formula: base (1000 - errors*250) + position bonus (0-50)
let score = compute_score(errors, pattern_len, end_pos);
// Returns: 0-1000 (higher = better match)
```

| Errors | Base Score | + Start Bonus | Final |
|--------|------------|---------------|-------|
| 0 | 1000 | +50 | 1000 (capped) |
| 1 | 750 | +25 (near start) | 775 |
| 2 | 500 | +0 | 500 |
| 3 | 250 | +0 | 250 |

## `no_std` Usage

This crate is `no_std` by default:

```toml
[dependencies]
flash-fuzzy-core = "0.1"
```

Enable `std` feature for standard library support:

```toml
[dependencies]
flash-fuzzy-core = { version = "0.1", features = ["std"] }
```

## Performance

- **Pattern compilation**: O(m) where m = pattern length
- **Bloom check**: O(1)
- **Search**: O(n * k) where n = text length, k = max errors
- **Memory**: ~1KB per searcher (256 u32 masks + state)

Typical benchmark: **< 1ms** to search 10,000 records.

## License

MIT - see [LICENSE](https://github.com/RafaCalRob/FlashFuzzy/blob/main/LICENSE)

## Part of Flash-Fuzzy

This is the core algorithm crate. For complete bindings see:
- [npm: flash-fuzzy](https://www.npmjs.com/package/flash-fuzzy) (JavaScript/TypeScript)
- [PyPI: flash-fuzzy](https://pypi.org/project/flash-fuzzy/) (Python)
- [Maven: flash-fuzzy](https://github.com/RafaCalRob/FlashFuzzy) (Java/Kotlin/Android)
- [Go module](https://github.com/RafaCalRob/flashfuzzy-go) (Go)
