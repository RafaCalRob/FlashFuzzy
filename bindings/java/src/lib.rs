//! Flash-Fuzzy JNI Bindings for Java/Kotlin/Android

use jni::objects::{JClass, JObject, JString, JValue};
use jni::sys::{jboolean, jfloat, jint, jobjectArray, JNI_TRUE};
use jni::JNIEnv;
use std::sync::Mutex;

use flash_fuzzy_core::{bitap, bloom::BloomFilter, BitapSearcher, ScoredResult, SearchConfig};

// Global state wrapped in Mutex for thread safety
struct FlashFuzzyState {
    records: Vec<Record>,
    config: SearchConfig,
}

struct Record {
    id: i32,
    text: String,
    bloom: BloomFilter,
}

static STATE: Mutex<Option<FlashFuzzyState>> = Mutex::new(None);

fn get_state() -> std::sync::MutexGuard<'static, Option<FlashFuzzyState>> {
    STATE.lock().unwrap()
}

/// Initialize the engine
#[no_mangle]
pub extern "system" fn Java_com_flashfuzzy_FlashFuzzy_nativeInit(
    _env: JNIEnv,
    _class: JClass,
    threshold: jfloat,
    max_errors: jint,
    max_results: jint,
) {
    let mut state = get_state();
    *state = Some(FlashFuzzyState {
        records: Vec::with_capacity(1000),
        config: SearchConfig {
            threshold: (threshold * 1000.0) as u16,
            max_errors: max_errors as u32,
            max_results: max_results as usize,
        },
    });
}

/// Add a record
#[no_mangle]
pub extern "system" fn Java_com_flashfuzzy_FlashFuzzy_nativeAdd<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
    id: jint,
    text: JString<'local>,
) -> jboolean {
    let text: String = match env.get_string(&text) {
        Ok(s) => s.into(),
        Err(_) => return 0,
    };

    if text.is_empty() {
        return 0;
    }

    let bloom = BloomFilter::from_text(text.as_bytes());

    let mut state = get_state();
    if let Some(ref mut s) = *state {
        s.records.push(Record { id, text, bloom });
        return JNI_TRUE as jboolean;
    }
    0
}

/// Search for matches
#[no_mangle]
pub extern "system" fn Java_com_flashfuzzy_FlashFuzzy_nativeSearch<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
    query: JString<'local>,
) -> jobjectArray {
    let query: String = match env.get_string(&query) {
        Ok(s) => s.into(),
        Err(_) => return std::ptr::null_mut(),
    };

    if query.is_empty() {
        // Return empty array
        let result_class = env.find_class("com/flashfuzzy/SearchResult").unwrap();
        return env.new_object_array(0, result_class, JObject::null()).unwrap().into_raw();
    }

    let state = get_state();
    let results = if let Some(ref s) = *state {
        let query_bytes = query.as_bytes();
        let searcher = BitapSearcher::new(query_bytes);
        let pattern_bloom = searcher.bloom();
        let pattern_len = searcher.pattern_len();

        let mut results: Vec<ScoredResult> = Vec::new();

        for record in &s.records {
            if !record.bloom.might_contain(pattern_bloom) {
                continue;
            }

            let text_bytes = record.text.as_bytes();
            if let Some(m) = searcher.search(text_bytes, s.config.max_errors) {
                let score = bitap::compute_score(m.errors, pattern_len as u32, m.end_pos);

                if score >= s.config.threshold {
                    let start_pos = m.end_pos.saturating_sub(pattern_len);
                    let result = ScoredResult::new(
                        record.id as u32,
                        score,
                        start_pos as u16,
                        m.end_pos as u16,
                    );

                    // Insert sorted
                    let pos = results.iter().position(|r| r.score < result.score).unwrap_or(results.len());
                    if results.len() < s.config.max_results {
                        results.insert(pos, result);
                    } else if pos < results.len() {
                        results.pop();
                        results.insert(pos, result);
                    }
                }
            }
        }
        results
    } else {
        Vec::new()
    };

    // Drop the lock before JNI calls
    drop(state);

    // Create Java array
    let result_class = env.find_class("com/flashfuzzy/SearchResult").unwrap();
    let array = env.new_object_array(results.len() as i32, &result_class, JObject::null()).unwrap();

    for (i, r) in results.iter().enumerate() {
        let obj = env.new_object(
            &result_class,
            "(IFII)V",
            &[
                JValue::Int(r.id as i32),
                JValue::Float(r.score as f32 / 1000.0),
                JValue::Int(r.start as i32),
                JValue::Int(r.end as i32),
            ],
        ).unwrap();
        env.set_object_array_element(&array, i as i32, obj).unwrap();
    }

    array.into_raw()
}

/// Remove a record by ID
#[no_mangle]
pub extern "system" fn Java_com_flashfuzzy_FlashFuzzy_nativeRemove(
    _env: JNIEnv,
    _class: JClass,
    id: jint,
) -> jboolean {
    let mut state = get_state();
    if let Some(ref mut s) = *state {
        if let Some(pos) = s.records.iter().position(|r| r.id == id) {
            s.records.remove(pos);
            return JNI_TRUE as jboolean;
        }
    }
    0
}

/// Reset all records
#[no_mangle]
pub extern "system" fn Java_com_flashfuzzy_FlashFuzzy_nativeReset(
    _env: JNIEnv,
    _class: JClass,
) {
    let mut state = get_state();
    if let Some(ref mut s) = *state {
        s.records.clear();
    }
}

/// Get record count
#[no_mangle]
pub extern "system" fn Java_com_flashfuzzy_FlashFuzzy_nativeGetCount(
    _env: JNIEnv,
    _class: JClass,
) -> jint {
    let state = get_state();
    if let Some(ref s) = *state {
        return s.records.len() as jint;
    }
    0
}

/// Set threshold
#[no_mangle]
pub extern "system" fn Java_com_flashfuzzy_FlashFuzzy_nativeSetThreshold(
    _env: JNIEnv,
    _class: JClass,
    threshold: jfloat,
) {
    let mut state = get_state();
    if let Some(ref mut s) = *state {
        s.config.threshold = (threshold * 1000.0) as u16;
    }
}

/// Set max errors
#[no_mangle]
pub extern "system" fn Java_com_flashfuzzy_FlashFuzzy_nativeSetMaxErrors(
    _env: JNIEnv,
    _class: JClass,
    max_errors: jint,
) {
    let mut state = get_state();
    if let Some(ref mut s) = *state {
        s.config.max_errors = max_errors as u32;
    }
}

/// Set max results
#[no_mangle]
pub extern "system" fn Java_com_flashfuzzy_FlashFuzzy_nativeSetMaxResults(
    _env: JNIEnv,
    _class: JClass,
    max_results: jint,
) {
    let mut state = get_state();
    if let Some(ref mut s) = *state {
        s.config.max_results = max_results as usize;
    }
}
