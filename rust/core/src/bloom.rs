//! Bloom filter for O(1) candidate rejection

/// 64-bit bloom filter using character positions
#[derive(Clone, Copy, Debug, Default)]
pub struct BloomFilter(pub u64);

impl BloomFilter {
    /// Create a new empty bloom filter
    #[inline]
    pub const fn new() -> Self {
        Self(0)
    }

    /// Create bloom filter from text
    #[inline]
    pub fn from_text(text: &[u8]) -> Self {
        let mut bits = 0u64;
        for &c in text {
            let lower = to_lower(c);
            let idx = (lower & 0x3F) as u64;
            bits |= 1u64 << idx;
        }
        Self(bits)
    }

    /// Check if pattern bloom might be contained in this text bloom
    /// Returns true if all pattern bits are present (might match)
    /// Returns false if any pattern bit is missing (definitely no match)
    #[inline]
    pub fn might_contain(&self, pattern_bloom: BloomFilter) -> bool {
        (self.0 & pattern_bloom.0) == pattern_bloom.0
    }

    /// Get raw bits
    #[inline]
    pub fn bits(&self) -> u64 {
        self.0
    }
}

/// Convert ASCII uppercase to lowercase (branch-free)
#[inline]
pub fn to_lower(c: u8) -> u8 {
    c | (0x20 * ((c >= b'A' && c <= b'Z') as u8))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bloom_basic() {
        let text_bloom = BloomFilter::from_text(b"hello world");
        let pattern_bloom = BloomFilter::from_text(b"hello");

        assert!(text_bloom.might_contain(pattern_bloom));
    }

    #[test]
    fn test_bloom_rejection() {
        let text_bloom = BloomFilter::from_text(b"hello");
        let pattern_bloom = BloomFilter::from_text(b"xyz");

        // 'x', 'y', 'z' are not in "hello", so should reject
        assert!(!text_bloom.might_contain(pattern_bloom));
    }

    #[test]
    fn test_to_lower() {
        assert_eq!(to_lower(b'A'), b'a');
        assert_eq!(to_lower(b'Z'), b'z');
        assert_eq!(to_lower(b'a'), b'a');
        assert_eq!(to_lower(b'1'), b'1');
    }
}
