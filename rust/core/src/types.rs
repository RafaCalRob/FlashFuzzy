//! Core types for Flash-Fuzzy

/// Search result from the Bitap algorithm
#[derive(Clone, Copy, Debug, Default)]
pub struct SearchMatch {
    /// Number of errors (edit distance)
    pub errors: u32,
    /// End position of match in text
    pub end_pos: usize,
}

/// Scored search result
#[derive(Clone, Copy, Debug, Default)]
pub struct ScoredResult {
    /// Record ID
    pub id: u32,
    /// Score (0-1000)
    pub score: u16,
    /// Start position of match
    pub start: u16,
    /// End position of match
    pub end: u16,
}

impl ScoredResult {
    /// Create a new scored result
    pub fn new(id: u32, score: u16, start: u16, end: u16) -> Self {
        Self { id, score, start, end }
    }
}

/// Configuration for the search engine
#[derive(Clone, Copy, Debug)]
pub struct SearchConfig {
    /// Maximum number of errors allowed (0-3)
    pub max_errors: u32,
    /// Minimum score threshold (0-1000)
    pub threshold: u16,
    /// Maximum number of results to return
    pub max_results: usize,
}

impl Default for SearchConfig {
    fn default() -> Self {
        Self {
            max_errors: 2,
            threshold: 250,
            max_results: 50,
        }
    }
}
