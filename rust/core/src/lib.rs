//! Flash-Fuzzy Core
//! High-performance fuzzy search using Bitap algorithm with bloom filter pre-filtering
//!
//! This crate is `no_std` compatible and provides the core search algorithms.

#![no_std]

pub mod bitap;
pub mod bloom;
pub mod types;

pub use bitap::BitapSearcher;
pub use bloom::BloomFilter;
pub use types::*;

/// Maximum pattern length supported (32 characters)
pub const MAX_PATTERN_LEN: usize = 32;

/// Default threshold score (0-1000 scale)
pub const DEFAULT_THRESHOLD: u16 = 250;

/// Default max errors allowed
pub const DEFAULT_MAX_ERRORS: u32 = 2;
