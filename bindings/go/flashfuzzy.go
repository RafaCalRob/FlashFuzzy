// Package flashfuzzy provides high-performance fuzzy search using Rust core via CGO.
package flashfuzzy

/*
#cgo LDFLAGS: -L${SRCDIR}/lib -lflash_fuzzy_ffi
#include <stdint.h>
#include <stdlib.h>

// FFI function declarations
void ff_init(void);
uint8_t* ff_get_write_buffer(uint32_t size);
void ff_commit_write(uint32_t len);
int32_t ff_add_record(uint32_t id);
int32_t ff_remove_record(uint32_t id);
void ff_set_max_errors(uint32_t errors);
void ff_set_threshold(uint32_t threshold);
void ff_set_max_results(uint32_t max);
uint32_t ff_get_record_count(void);
void ff_prepare_pattern(void);
uint32_t ff_search(void);
uint32_t ff_get_result_id(uint32_t index);
uint32_t ff_get_result_score(uint32_t index);
uint32_t ff_get_result_start(uint32_t index);
uint32_t ff_get_result_end(uint32_t index);
void ff_reset(void);
uint32_t ff_compact(void);
uint32_t ff_get_string_pool_used(void);
uint32_t ff_get_available_memory(void);
*/
import "C"
import (
	"unsafe"
)

// SearchResult represents a single search result
type SearchResult struct {
	ID    uint32
	Score float32
	Start uint32
	End   uint32
}

// FlashFuzzy is a high-performance fuzzy search engine
type FlashFuzzy struct {
	threshold  float32
	maxErrors  uint32
	maxResults uint32
}

// Options for creating a new FlashFuzzy instance
type Options struct {
	Threshold  float32 // Minimum score (0.0-1.0), default: 0.25
	MaxErrors  uint32  // Maximum edit distance (0-3), default: 2
	MaxResults uint32  // Maximum results to return, default: 50
}

// DefaultOptions returns the default options
func DefaultOptions() Options {
	return Options{
		Threshold:  0.25,
		MaxErrors:  2,
		MaxResults: 50,
	}
}

// New creates a new FlashFuzzy instance with the given options
func New(opts Options) *FlashFuzzy {
	C.ff_init()

	if opts.Threshold == 0 {
		opts.Threshold = 0.25
	}
	if opts.MaxResults == 0 {
		opts.MaxResults = 50
	}

	C.ff_set_threshold(C.uint32_t(opts.Threshold * 1000))
	C.ff_set_max_errors(C.uint32_t(opts.MaxErrors))
	C.ff_set_max_results(C.uint32_t(opts.MaxResults))

	return &FlashFuzzy{
		threshold:  opts.Threshold,
		maxErrors:  opts.MaxErrors,
		maxResults: opts.MaxResults,
	}
}

// Add adds a record to the search index
func (ff *FlashFuzzy) Add(id uint32, text string) bool {
	if len(text) == 0 {
		return false
	}

	textBytes := []byte(text)
	ptr := C.ff_get_write_buffer(C.uint32_t(len(textBytes)))
	if ptr == nil {
		return false
	}

	// Copy text to write buffer
	cBytes := (*[1 << 30]byte)(unsafe.Pointer(ptr))[:len(textBytes):len(textBytes)]
	copy(cBytes, textBytes)

	C.ff_commit_write(C.uint32_t(len(textBytes)))
	result := C.ff_add_record(C.uint32_t(id))

	return result == 1
}

// AddBatch adds multiple records efficiently
func (ff *FlashFuzzy) AddBatch(records map[uint32]string) int {
	added := 0
	for id, text := range records {
		if ff.Add(id, text) {
			added++
		}
	}
	return added
}

// Search performs a fuzzy search and returns matching results
func (ff *FlashFuzzy) Search(query string) []SearchResult {
	if len(query) == 0 {
		return nil
	}

	queryBytes := []byte(query)
	ptr := C.ff_get_write_buffer(C.uint32_t(len(queryBytes)))
	if ptr == nil {
		return nil
	}

	// Copy query to write buffer
	cBytes := (*[1 << 30]byte)(unsafe.Pointer(ptr))[:len(queryBytes):len(queryBytes)]
	copy(cBytes, queryBytes)

	C.ff_commit_write(C.uint32_t(len(queryBytes)))
	C.ff_prepare_pattern()

	count := uint32(C.ff_search())
	if count == 0 {
		return nil
	}

	results := make([]SearchResult, count)
	for i := uint32(0); i < count; i++ {
		results[i] = SearchResult{
			ID:    uint32(C.ff_get_result_id(C.uint32_t(i))),
			Score: float32(C.ff_get_result_score(C.uint32_t(i))) / 1000.0,
			Start: uint32(C.ff_get_result_start(C.uint32_t(i))),
			End:   uint32(C.ff_get_result_end(C.uint32_t(i))),
		}
	}

	return results
}

// Remove removes a record by ID
func (ff *FlashFuzzy) Remove(id uint32) bool {
	return C.ff_remove_record(C.uint32_t(id)) == 1
}

// Reset clears all records
func (ff *FlashFuzzy) Reset() {
	C.ff_reset()
}

// Count returns the number of records
func (ff *FlashFuzzy) Count() uint32 {
	return uint32(C.ff_get_record_count())
}

// SetThreshold sets the minimum score threshold
func (ff *FlashFuzzy) SetThreshold(threshold float32) {
	ff.threshold = threshold
	C.ff_set_threshold(C.uint32_t(threshold * 1000))
}

// SetMaxErrors sets the maximum number of errors allowed
func (ff *FlashFuzzy) SetMaxErrors(maxErrors uint32) {
	ff.maxErrors = maxErrors
	C.ff_set_max_errors(C.uint32_t(maxErrors))
}

// SetMaxResults sets the maximum number of results to return
func (ff *FlashFuzzy) SetMaxResults(maxResults uint32) {
	ff.maxResults = maxResults
	C.ff_set_max_results(C.uint32_t(maxResults))
}

// Stats returns statistics about the index
type Stats struct {
	RecordCount     uint32
	StringPoolUsed  uint32
	AvailableMemory uint32
}

// GetStats returns current index statistics
func (ff *FlashFuzzy) GetStats() Stats {
	return Stats{
		RecordCount:     uint32(C.ff_get_record_count()),
		StringPoolUsed:  uint32(C.ff_get_string_pool_used()),
		AvailableMemory: uint32(C.ff_get_available_memory()),
	}
}
