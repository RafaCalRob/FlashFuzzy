//! Bitap (Shift-Or) algorithm for fuzzy string matching
//!
//! Implements Wu-Manber extension for approximate matching with errors.

use crate::bloom::{to_lower, BloomFilter};
use crate::types::SearchMatch;
use crate::MAX_PATTERN_LEN;

/// Bitap searcher with pre-computed pattern masks
pub struct BitapSearcher {
    /// Character bitmasks (256 ASCII chars)
    char_masks: [u32; 256],
    /// Pattern length
    pattern_len: usize,
    /// Bloom filter for the pattern
    pattern_bloom: BloomFilter,
}

impl BitapSearcher {
    /// Create a new Bitap searcher from a pattern
    pub fn new(pattern: &[u8]) -> Self {
        let len = pattern.len().min(MAX_PATTERN_LEN);
        let mut char_masks = [0u32; 256];
        let mut bloom_bits = 0u64;

        for (i, &ch) in pattern.iter().take(len).enumerate() {
            let lower = to_lower(ch);
            let bit = 1u32 << i;

            // Set bit for lowercase
            char_masks[lower as usize] |= bit;

            // Also set for original case if different
            if lower != ch {
                char_masks[ch as usize] |= bit;
            }

            // Build bloom filter
            let bloom_idx = (lower & 0x3F) as u64;
            bloom_bits |= 1u64 << bloom_idx;
        }

        Self {
            char_masks,
            pattern_len: len,
            pattern_bloom: BloomFilter(bloom_bits),
        }
    }

    /// Get the pattern length
    #[inline]
    pub fn pattern_len(&self) -> usize {
        self.pattern_len
    }

    /// Get the pattern's bloom filter
    #[inline]
    pub fn bloom(&self) -> BloomFilter {
        self.pattern_bloom
    }

    /// Search for pattern in text with up to max_errors
    /// Returns the best match found (lowest error count)
    pub fn search(&self, text: &[u8], max_errors: u32) -> Option<SearchMatch> {
        if self.pattern_len == 0 || text.is_empty() {
            return None;
        }

        let pattern_len = self.pattern_len as u32;

        // Adaptive max_errors based on pattern length
        let effective_max_errors = if pattern_len <= 3 {
            0 // Exact match only for very short patterns
        } else if pattern_len <= 5 {
            max_errors.min(1) // Max 1 error for short patterns
        } else {
            max_errors
        };

        // Initialize R array (1 = matched position)
        let mut r = [0u32; MAX_PATTERN_LEN + 1];

        let pattern_mask = (1u32 << self.pattern_len) - 1;
        let match_bit = 1u32 << (self.pattern_len - 1);

        let mut best_errors = effective_max_errors + 1;
        let mut best_pos = 0usize;

        for (pos, &ch) in text.iter().enumerate() {
            let lower = to_lower(ch);
            let char_mask = self.char_masks[lower as usize];

            // Save old values for error propagation
            let mut old_r = r[0];

            // Exact match: shift left, seed new match at pos 0, filter by char
            r[0] = ((r[0] << 1) | 1) & char_mask;

            // Error levels
            for k in 1..=(effective_max_errors as usize) {
                let new_r = r[k];

                // Error transitions:
                // - Exact match at this level
                // - Substitution: was at pos i with k-1 errors, now at i+1 with k
                // - Deletion: was at pos i with k-1 errors, stay at i with k
                // - Insertion: was at pos i with k errors, now at i+1 with k
                r[k] = (((r[k] << 1) | 1) & char_mask) |  // exact match
                       (old_r << 1) |                      // substitution
                       old_r |                              // deletion
                       (r[k - 1] << 1);                     // insertion

                old_r = new_r;
            }

            // Mask to pattern length
            for i in 0..=(effective_max_errors as usize) {
                r[i] &= pattern_mask;
            }

            // Check for matches
            for k in 0..=effective_max_errors {
                if (r[k as usize] & match_bit) != 0 {
                    if k < best_errors {
                        best_errors = k;
                        best_pos = pos + 1;
                    }
                    break;
                }
            }
        }

        if best_errors <= effective_max_errors {
            Some(SearchMatch {
                errors: best_errors,
                end_pos: best_pos,
            })
        } else {
            None
        }
    }
}

/// Compute score from match result
///
/// Score formula:
/// - Base: 1000 - (errors * 250)
/// - Position bonus: +50 for start, +25 for near start
pub fn compute_score(errors: u32, pattern_len: u32, end_pos: usize) -> u16 {
    let base = 1000u32.saturating_sub(errors * 250);

    let start_pos = end_pos.saturating_sub(pattern_len as usize);

    let pos_bonus = if start_pos == 0 {
        50
    } else if start_pos < 10 {
        25
    } else {
        0
    };

    let score = base + pos_bonus;
    if score > 1000 { 1000 } else { score as u16 }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exact_match() {
        let searcher = BitapSearcher::new(b"hello");
        let result = searcher.search(b"hello world", 0);

        assert!(result.is_some());
        let m = result.unwrap();
        assert_eq!(m.errors, 0);
        assert_eq!(m.end_pos, 5);
    }

    #[test]
    fn test_fuzzy_match() {
        let searcher = BitapSearcher::new(b"hello");
        let result = searcher.search(b"hallo world", 2);

        assert!(result.is_some());
        let m = result.unwrap();
        assert_eq!(m.errors, 1); // 'e' -> 'a' substitution
    }

    #[test]
    fn test_no_match() {
        let searcher = BitapSearcher::new(b"xyz");
        let result = searcher.search(b"hello world", 0);

        assert!(result.is_none());
    }

    #[test]
    fn test_case_insensitive() {
        let searcher = BitapSearcher::new(b"HELLO");
        let result = searcher.search(b"hello world", 0);

        assert!(result.is_some());
        assert_eq!(result.unwrap().errors, 0);
    }

    #[test]
    fn test_score_computation() {
        // Exact match at start
        assert_eq!(compute_score(0, 5, 5), 1000);

        // 1 error
        assert_eq!(compute_score(1, 5, 5), 800); // 750 + 50

        // Match not at start
        assert_eq!(compute_score(0, 5, 15), 1000); // 1000 + 0
    }
}
