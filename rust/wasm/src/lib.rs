//! Flash-Fuzzy WASM Module
//! WebAssembly-specific wrapper around the core library

#![no_std]
#![no_main]

use flash_fuzzy_core::{bitap, bloom::BloomFilter, BitapSearcher, ScoredResult};

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

// ============ Panic Handler ============

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

// ============ WASM Exports ============
// Note: WASM uses short names without ff_ prefix for smaller binary

#[no_mangle]
pub extern "C" fn init() {
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

#[no_mangle]
pub extern "C" fn getWriteBuffer(size: usize) -> *mut u8 {
    unsafe {
        if size > SCRATCHPAD_SIZE {
            return core::ptr::null_mut();
        }
        SCRATCHPAD.as_mut_ptr()
    }
}

#[no_mangle]
pub extern "C" fn commitWrite(len: usize) {
    unsafe {
        SCRATCHPAD_LEN = if len > SCRATCHPAD_SIZE { SCRATCHPAD_SIZE } else { len };
    }
}

#[no_mangle]
pub extern "C" fn addRecord(id: u32) -> i32 {
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

        let src = &SCRATCHPAD[..text_len];
        let dst = &mut STRING_POOL[text_start..text_start + text_len];
        dst.copy_from_slice(src);
        STRING_POOL_USED += text_len;

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

#[no_mangle]
pub extern "C" fn setMaxErrors(errors: u32) {
    unsafe {
        MAX_ERRORS = if errors > 3 { 3 } else { errors };
    }
}

#[no_mangle]
pub extern "C" fn setThreshold(threshold: u32) {
    unsafe {
        THRESHOLD = threshold as u16;
    }
}

#[no_mangle]
pub extern "C" fn setMaxResults(max: u32) {
    unsafe {
        let m = max as usize;
        MAX_RESULTS_CFG = if m > MAX_RESULTS { MAX_RESULTS } else { m };
    }
}

#[no_mangle]
pub extern "C" fn getRecordCount() -> u32 {
    unsafe { RECORD_COUNT as u32 }
}

#[no_mangle]
pub extern "C" fn preparePattern() {
    unsafe {
        let len = if SCRATCHPAD_LEN > MAX_PATTERN_LEN { MAX_PATTERN_LEN } else { SCRATCHPAD_LEN };
        PATTERN_LEN = len;

        let src = &SCRATCHPAD[..len];
        let dst = &mut PATTERN[..len];
        dst.copy_from_slice(src);

        CHAR_MASKS = [0; 256];
        PATTERN_BLOOM = 0;

        for (i, &ch) in src.iter().enumerate() {
            let lower = to_lower(ch);
            let bit = 1u32 << i;
            CHAR_MASKS[lower as usize] |= bit;
            if lower != ch {
                CHAR_MASKS[ch as usize] |= bit;
            }
            let bloom_idx = (lower & 0x3F) as u64;
            PATTERN_BLOOM |= 1u64 << bloom_idx;
        }

        SCRATCHPAD_LEN = 0;
    }
}

#[no_mangle]
pub extern "C" fn search() -> u32 {
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

#[inline]
fn to_lower(c: u8) -> u8 {
    c | (0x20 * ((c >= b'A' && c <= b'Z') as u8))
}

fn insert_result(result: ScoredResult) {
    unsafe {
        if RESULT_COUNT >= MAX_RESULTS_CFG {
            if result.score <= RESULTS[RESULT_COUNT - 1].score {
                return;
            }
            RESULT_COUNT -= 1;
        }

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

#[no_mangle]
pub extern "C" fn getResultId(index: u32) -> u32 {
    unsafe {
        let i = index as usize;
        if i < RESULT_COUNT { RESULTS[i].id } else { 0 }
    }
}

#[no_mangle]
pub extern "C" fn getResultScore(index: u32) -> u32 {
    unsafe {
        let i = index as usize;
        if i < RESULT_COUNT { RESULTS[i].score as u32 } else { 0 }
    }
}

#[no_mangle]
pub extern "C" fn getResultStart(index: u32) -> u32 {
    unsafe {
        let i = index as usize;
        if i < RESULT_COUNT { RESULTS[i].start as u32 } else { 0 }
    }
}

#[no_mangle]
pub extern "C" fn getResultEnd(index: u32) -> u32 {
    unsafe {
        let i = index as usize;
        if i < RESULT_COUNT { RESULTS[i].end as u32 } else { 0 }
    }
}

#[no_mangle]
pub extern "C" fn reset() {
    unsafe {
        RECORD_COUNT = 0;
        STRING_POOL_USED = 0;
        RESULT_COUNT = 0;
        PATTERN_LEN = 0;
    }
}

#[no_mangle]
pub extern "C" fn compact() -> u32 {
    unsafe { RECORD_COUNT as u32 }
}

#[no_mangle]
pub extern "C" fn removeRecord(id: u32) -> i32 {
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

#[no_mangle]
pub extern "C" fn getStringPoolUsed() -> u32 {
    unsafe { STRING_POOL_USED as u32 }
}

#[no_mangle]
pub extern "C" fn getAvailableMemory() -> u32 {
    unsafe { (STRING_POOL_SIZE - STRING_POOL_USED) as u32 }
}
