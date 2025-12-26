//! Flash-Fuzzy FFI Layer
//! C-compatible exports for all platform bindings
//!
//! This crate provides C-ABI compatible exports for use with:
//! - Python (PyO3)
//! - Go (CGO)
//! - Java/Kotlin/Android (JNI)
//! - Any language with C FFI support

use flash_fuzzy_core::{bitap, bloom::{BloomFilter, to_lower}, BitapSearcher, ScoredResult};

// ============ Memory Constants ============

const MAX_RECORDS: usize = 100_000;
const MAX_RESULTS: usize = 100;
const STRING_POOL_SIZE: usize = 4 * 1024 * 1024; // 4MB
const SCRATCHPAD_SIZE: usize = 64 * 1024; // 64KB
const MAX_PATTERN_LEN: usize = 32;

// ============ Record Structure ============

#[derive(Clone, Copy)]
struct Record {
    id: u32,
    text_start: u32,
    text_len: u16,
    bloom: u64,
    active: bool,
}

impl Default for Record {
    fn default() -> Self {
        Self { id: 0, text_start: 0, text_len: 0, bloom: 0, active: false }
    }
}

// ============ Global State ============

static mut RECORDS: [Record; MAX_RECORDS] = [Record { id: 0, text_start: 0, text_len: 0, bloom: 0, active: false }; MAX_RECORDS];
static mut RECORD_COUNT: usize = 0;

static mut STRING_POOL: [u8; STRING_POOL_SIZE] = [0; STRING_POOL_SIZE];
static mut STRING_POOL_USED: usize = 0;

static mut PATTERN: [u8; MAX_PATTERN_LEN] = [0; MAX_PATTERN_LEN];
static mut PATTERN_LEN: usize = 0;
static mut CHAR_MASKS: [u32; 256] = [0; 256];
static mut PATTERN_BLOOM: u64 = 0;

static mut MAX_ERRORS: u32 = 2;
static mut THRESHOLD: u16 = 250;
static mut MAX_RESULTS_CFG: usize = 50;

static mut RESULTS: [ScoredResult; MAX_RESULTS] = [ScoredResult { id: 0, score: 0, start: 0, end: 0 }; MAX_RESULTS];
static mut RESULT_COUNT: usize = 0;

static mut SCRATCHPAD: [u8; SCRATCHPAD_SIZE] = [0; SCRATCHPAD_SIZE];
static mut SCRATCHPAD_LEN: usize = 0;

// ============ FFI Exports ============

/// Initialize the engine (reset all state)
#[no_mangle]
pub extern "C" fn ff_init() {
    unsafe {
        RECORD_COUNT = 0;
        STRING_POOL_USED = 0;
        PATTERN_LEN = 0;
        RESULT_COUNT = 0;
        SCRATCHPAD_LEN = 0;
        MAX_ERRORS = 2;
        THRESHOLD = 250;
        MAX_RESULTS_CFG = 50;
        CHAR_MASKS = [0; 256];
    }
}

/// Get pointer to write buffer
#[no_mangle]
pub extern "C" fn ff_get_write_buffer(size: u32) -> *mut u8 {
    unsafe {
        if size as usize > SCRATCHPAD_SIZE {
            return core::ptr::null_mut();
        }
        SCRATCHPAD.as_mut_ptr()
    }
}

/// Commit written bytes to scratchpad
#[no_mangle]
pub extern "C" fn ff_commit_write(len: u32) {
    unsafe {
        let l = len as usize;
        SCRATCHPAD_LEN = if l > SCRATCHPAD_SIZE { SCRATCHPAD_SIZE } else { l };
    }
}

/// Add a record from scratchpad
/// Returns: 1 on success, negative on error
#[no_mangle]
pub extern "C" fn ff_add_record(id: u32) -> i32 {
    unsafe {
        if RECORD_COUNT >= MAX_RECORDS {
            return -1;
        }
        if SCRATCHPAD_LEN == 0 {
            return -2;
        }

        let text_start = STRING_POOL_USED;
        let text_len = SCRATCHPAD_LEN;

        if text_start + text_len > STRING_POOL_SIZE {
            return -3;
        }

        // Copy text to string pool
        let src = &SCRATCHPAD[..text_len];
        let dst = &mut STRING_POOL[text_start..text_start + text_len];
        dst.copy_from_slice(src);
        STRING_POOL_USED += text_len;

        // Pre-compute bloom filter
        let bloom = BloomFilter::from_text(src);

        RECORDS[RECORD_COUNT] = Record {
            id,
            text_start: text_start as u32,
            text_len: text_len as u16,
            bloom: bloom.bits(),
            active: true,
        };
        RECORD_COUNT += 1;
        SCRATCHPAD_LEN = 0;

        1
    }
}

/// Remove a record by ID
#[no_mangle]
pub extern "C" fn ff_remove_record(id: u32) -> i32 {
    unsafe {
        for i in 0..RECORD_COUNT {
            if RECORDS[i].id == id && RECORDS[i].active {
                RECORDS[i].active = false;
                return 1;
            }
        }
        0
    }
}

/// Set maximum errors allowed (0-3)
#[no_mangle]
pub extern "C" fn ff_set_max_errors(errors: u32) {
    unsafe {
        MAX_ERRORS = if errors > 3 { 3 } else { errors };
    }
}

/// Set minimum score threshold (0-1000)
#[no_mangle]
pub extern "C" fn ff_set_threshold(threshold: u32) {
    unsafe {
        THRESHOLD = threshold as u16;
    }
}

/// Set maximum results to return
#[no_mangle]
pub extern "C" fn ff_set_max_results(max: u32) {
    unsafe {
        let m = max as usize;
        MAX_RESULTS_CFG = if m > MAX_RESULTS { MAX_RESULTS } else { m };
    }
}

/// Get current record count
#[no_mangle]
pub extern "C" fn ff_get_record_count() -> u32 {
    unsafe { RECORD_COUNT as u32 }
}

/// Prepare pattern from scratchpad
#[no_mangle]
pub extern "C" fn ff_prepare_pattern() {
    unsafe {
        let len = if SCRATCHPAD_LEN > MAX_PATTERN_LEN { MAX_PATTERN_LEN } else { SCRATCHPAD_LEN };
        PATTERN_LEN = len;

        let src = &SCRATCHPAD[..len];
        let dst = &mut PATTERN[..len];
        dst.copy_from_slice(src);

        // Build character masks and bloom filter using core
        let searcher = BitapSearcher::new(src);
        PATTERN_BLOOM = searcher.bloom().bits();

        // Rebuild char masks (we need them for search)
        CHAR_MASKS = [0; 256];
        for (i, &ch) in src.iter().enumerate() {
            let lower = to_lower(ch);
            let bit = 1u32 << i;
            CHAR_MASKS[lower as usize] |= bit;
            if lower != ch {
                CHAR_MASKS[ch as usize] |= bit;
            }
        }

        SCRATCHPAD_LEN = 0;
    }
}

/// Execute search, return result count
#[no_mangle]
pub extern "C" fn ff_search() -> u32 {
    unsafe {
        RESULT_COUNT = 0;

        if PATTERN_LEN == 0 {
            return 0;
        }

        let pattern = &PATTERN[..PATTERN_LEN];
        let searcher = BitapSearcher::new(pattern);
        let pattern_bloom = BloomFilter(PATTERN_BLOOM);
        let threshold = THRESHOLD;
        let max_errors = MAX_ERRORS;

        for i in 0..RECORD_COUNT {
            let record = &RECORDS[i];
            if !record.active {
                continue;
            }

            // Bloom filter pre-check
            let text_bloom = BloomFilter(record.bloom);
            if !text_bloom.might_contain(pattern_bloom) {
                continue;
            }

            let start = record.text_start as usize;
            let end = start + record.text_len as usize;
            let text = &STRING_POOL[start..end];

            if let Some(m) = searcher.search(text, max_errors) {
                let score = bitap::compute_score(m.errors, PATTERN_LEN as u32, m.end_pos);

                if score >= threshold {
                    let start_pos = m.end_pos.saturating_sub(PATTERN_LEN);

                    insert_result(ScoredResult::new(
                        record.id,
                        score,
                        start_pos as u16,
                        m.end_pos as u16,
                    ));
                }
            }
        }

        RESULT_COUNT as u32
    }
}

fn insert_result(result: ScoredResult) {
    unsafe {
        if RESULT_COUNT >= MAX_RESULTS_CFG {
            if result.score <= RESULTS[RESULT_COUNT - 1].score {
                return;
            }
            RESULT_COUNT -= 1;
        }

        // Insertion sort (descending by score)
        let mut pos = RESULT_COUNT;
        while pos > 0 && RESULTS[pos - 1].score < result.score {
            if pos < MAX_RESULTS {
                RESULTS[pos] = RESULTS[pos - 1];
            }
            pos -= 1;
        }

        if pos < MAX_RESULTS {
            RESULTS[pos] = result;
            RESULT_COUNT += 1;
        }
    }
}

/// Get result ID at index
#[no_mangle]
pub extern "C" fn ff_get_result_id(index: u32) -> u32 {
    unsafe {
        let i = index as usize;
        if i < RESULT_COUNT { RESULTS[i].id } else { 0 }
    }
}

/// Get result score at index
#[no_mangle]
pub extern "C" fn ff_get_result_score(index: u32) -> u32 {
    unsafe {
        let i = index as usize;
        if i < RESULT_COUNT { RESULTS[i].score as u32 } else { 0 }
    }
}

/// Get result start position at index
#[no_mangle]
pub extern "C" fn ff_get_result_start(index: u32) -> u32 {
    unsafe {
        let i = index as usize;
        if i < RESULT_COUNT { RESULTS[i].start as u32 } else { 0 }
    }
}

/// Get result end position at index
#[no_mangle]
pub extern "C" fn ff_get_result_end(index: u32) -> u32 {
    unsafe {
        let i = index as usize;
        if i < RESULT_COUNT { RESULTS[i].end as u32 } else { 0 }
    }
}

/// Reset all data
#[no_mangle]
pub extern "C" fn ff_reset() {
    unsafe {
        RECORD_COUNT = 0;
        STRING_POOL_USED = 0;
        RESULT_COUNT = 0;
        PATTERN_LEN = 0;
    }
}

/// Compact records (remove inactive)
#[no_mangle]
pub extern "C" fn ff_compact() -> u32 {
    unsafe { RECORD_COUNT as u32 }
}

/// Get string pool used bytes
#[no_mangle]
pub extern "C" fn ff_get_string_pool_used() -> u32 {
    unsafe { STRING_POOL_USED as u32 }
}

/// Get available memory
#[no_mangle]
pub extern "C" fn ff_get_available_memory() -> u32 {
    unsafe { (STRING_POOL_SIZE - STRING_POOL_USED) as u32 }
}

