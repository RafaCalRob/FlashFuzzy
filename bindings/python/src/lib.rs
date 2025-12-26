//! Flash-Fuzzy Python Bindings
//! High-performance fuzzy search using PyO3

use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;
use flash_fuzzy_core::{bitap, bloom::BloomFilter, BitapSearcher, ScoredResult, SearchConfig};

const MAX_RECORDS: usize = 100_000;
const MAX_RESULTS: usize = 100;

/// A single search result
#[pyclass]
#[derive(Clone)]
struct SearchResult {
    #[pyo3(get)]
    id: u32,
    #[pyo3(get)]
    score: f32,
    #[pyo3(get)]
    start: u32,
    #[pyo3(get)]
    end: u32,
}

#[pymethods]
impl SearchResult {
    fn __repr__(&self) -> String {
        format!("SearchResult(id={}, score={:.3}, start={}, end={})",
                self.id, self.score, self.start, self.end)
    }
}

/// Record stored in the index
struct Record {
    id: u32,
    text: String,
    bloom: BloomFilter,
}

/// High-performance fuzzy search engine
#[pyclass]
struct FlashFuzzy {
    records: Vec<Record>,
    config: SearchConfig,
}

#[pymethods]
impl FlashFuzzy {
    /// Create a new FlashFuzzy instance
    ///
    /// Args:
    ///     threshold: Minimum score (0.0-1.0) for results. Default: 0.25
    ///     max_errors: Maximum edit distance (0-3). Default: 2
    ///     max_results: Maximum results to return. Default: 50
    #[new]
    #[pyo3(signature = (threshold=0.25, max_errors=2, max_results=50))]
    fn new(threshold: f32, max_errors: u32, max_results: usize) -> PyResult<Self> {
        if threshold < 0.0 || threshold > 1.0 {
            return Err(PyValueError::new_err("threshold must be between 0.0 and 1.0"));
        }
        if max_errors > 3 {
            return Err(PyValueError::new_err("max_errors must be between 0 and 3"));
        }
        if max_results > MAX_RESULTS {
            return Err(PyValueError::new_err(format!("max_results must be <= {}", MAX_RESULTS)));
        }

        Ok(Self {
            records: Vec::with_capacity(1000),
            config: SearchConfig {
                threshold: (threshold * 1000.0) as u16,
                max_errors,
                max_results,
            },
        })
    }

    /// Add a single record or list of records
    ///
    /// Args:
    ///     records: A dict or list of dicts with 'id' and text fields
    ///
    /// Returns:
    ///     Number of records added
    fn add(&mut self, records: &Bound<'_, PyAny>) -> PyResult<usize> {
        // Handle list of records
        if let Ok(list) = records.downcast::<pyo3::types::PyList>() {
            let mut added = 0;
            for item in list.iter() {
                added += self.add_single(&item)?;
            }
            return Ok(added);
        }

        // Handle single record
        self.add_single(records)
    }

    /// Search for matching records
    ///
    /// Args:
    ///     query: The search query string
    ///
    /// Returns:
    ///     List of SearchResult objects sorted by score (descending)
    fn search(&self, query: &str) -> Vec<SearchResult> {
        if query.is_empty() || self.records.is_empty() {
            return Vec::new();
        }

        let query_bytes = query.as_bytes();
        let searcher = BitapSearcher::new(query_bytes);
        let pattern_bloom = searcher.bloom();
        let pattern_len = searcher.pattern_len();

        let mut results: Vec<ScoredResult> = Vec::with_capacity(self.config.max_results);

        for record in &self.records {
            // Bloom filter pre-check
            if !record.bloom.might_contain(pattern_bloom) {
                continue;
            }

            let text_bytes = record.text.as_bytes();
            if let Some(m) = searcher.search(text_bytes, self.config.max_errors) {
                let score = bitap::compute_score(m.errors, pattern_len as u32, m.end_pos);

                if score >= self.config.threshold {
                    let start_pos = m.end_pos.saturating_sub(pattern_len);

                    let result = ScoredResult::new(
                        record.id,
                        score,
                        start_pos as u16,
                        m.end_pos as u16,
                    );

                    // Insert sorted (descending by score)
                    Self::insert_sorted(&mut results, result, self.config.max_results);
                }
            }
        }

        // Convert to Python objects
        results.into_iter().map(|r| SearchResult {
            id: r.id,
            score: r.score as f32 / 1000.0,
            start: r.start as u32,
            end: r.end as u32,
        }).collect()
    }

    /// Remove a record by ID
    ///
    /// Args:
    ///     id: The record ID to remove
    ///
    /// Returns:
    ///     True if found and removed, False otherwise
    fn remove(&mut self, id: u32) -> bool {
        if let Some(pos) = self.records.iter().position(|r| r.id == id) {
            self.records.remove(pos);
            true
        } else {
            false
        }
    }

    /// Clear all records
    fn reset(&mut self) {
        self.records.clear();
    }

    /// Get number of records
    #[getter]
    fn count(&self) -> usize {
        self.records.len()
    }

    /// Set threshold
    #[setter]
    fn set_threshold(&mut self, value: f32) -> PyResult<()> {
        if value < 0.0 || value > 1.0 {
            return Err(PyValueError::new_err("threshold must be between 0.0 and 1.0"));
        }
        self.config.threshold = (value * 1000.0) as u16;
        Ok(())
    }

    /// Set max errors
    #[setter]
    fn set_max_errors(&mut self, value: u32) -> PyResult<()> {
        if value > 3 {
            return Err(PyValueError::new_err("max_errors must be between 0 and 3"));
        }
        self.config.max_errors = value;
        Ok(())
    }

    /// Set max results
    #[setter]
    fn set_max_results(&mut self, value: usize) -> PyResult<()> {
        if value > MAX_RESULTS {
            return Err(PyValueError::new_err(format!("max_results must be <= {}", MAX_RESULTS)));
        }
        self.config.max_results = value;
        Ok(())
    }

    fn __repr__(&self) -> String {
        format!("FlashFuzzy(records={}, threshold={:.2}, max_errors={})",
                self.records.len(),
                self.config.threshold as f32 / 1000.0,
                self.config.max_errors)
    }
}

impl FlashFuzzy {
    fn add_single(&mut self, record: &Bound<'_, PyAny>) -> PyResult<usize> {
        if self.records.len() >= MAX_RECORDS {
            return Err(PyValueError::new_err("Maximum record limit reached"));
        }

        // Get ID
        let id: u32 = if let Ok(id_val) = record.get_item("id") {
            id_val.extract()?
        } else {
            self.records.len() as u32
        };

        // Extract text from all string fields
        let mut text_parts: Vec<String> = Vec::new();

        if let Ok(dict) = record.downcast::<pyo3::types::PyDict>() {
            for (_key, value) in dict.iter() {
                if let Ok(s) = value.extract::<String>() {
                    text_parts.push(s);
                }
            }
        }

        if text_parts.is_empty() {
            return Ok(0);
        }

        let text = text_parts.join(" ");
        let bloom = BloomFilter::from_text(text.as_bytes());

        self.records.push(Record { id, text, bloom });
        Ok(1)
    }

    fn insert_sorted(results: &mut Vec<ScoredResult>, result: ScoredResult, max_results: usize) {
        if results.len() >= max_results {
            if result.score <= results.last().unwrap().score {
                return;
            }
            results.pop();
        }

        let pos = results.iter().position(|r| r.score < result.score).unwrap_or(results.len());
        results.insert(pos, result);
    }
}

/// Flash-Fuzzy: High-performance fuzzy search engine
#[pymodule]
fn flash_fuzzy(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<FlashFuzzy>()?;
    m.add_class::<SearchResult>()?;
    Ok(())
}
