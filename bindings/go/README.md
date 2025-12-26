# Flash-Fuzzy Go

High-performance fuzzy search engine for Go, powered by Rust.

## Installation

```bash
go get github.com/RafaCalRob/flashfuzzy-go
```

**Note:** Requires the native library to be built first. See [Building](#building) below.

## Quick Start

```go
package main

import (
    "fmt"
    flashfuzzy "github.com/RafaCalRob/flashfuzzy-go"
)

func main() {
    // Create instance with default options
    ff := flashfuzzy.New(flashfuzzy.DefaultOptions())

    // Add records
    ff.Add(1, "Wireless Headphones")
    ff.Add(2, "Mechanical Keyboard")
    ff.Add(3, "USB-C Cable")

    // Search with typos
    results := ff.Search("keybord") // Note the typo
    for _, r := range results {
        fmt.Printf("ID: %d, Score: %.2f\n", r.ID, r.Score)
    }
}
```

## API

### Creating an Instance

```go
// With default options
ff := flashfuzzy.New(flashfuzzy.DefaultOptions())

// With custom options
ff := flashfuzzy.New(flashfuzzy.Options{
    Threshold:  0.3,   // Minimum score (0.0-1.0)
    MaxErrors:  2,     // Max edit distance (0-3)
    MaxResults: 100,   // Max results to return
})
```

### Methods

| Method | Description |
|--------|-------------|
| `Add(id uint32, text string) bool` | Add a record |
| `AddBatch(records map[uint32]string) int` | Add multiple records |
| `Search(query string) []SearchResult` | Search for matches |
| `Remove(id uint32) bool` | Remove a record |
| `Reset()` | Clear all records |
| `Count() uint32` | Get record count |
| `SetThreshold(float32)` | Set score threshold |
| `SetMaxErrors(uint32)` | Set max edit distance |
| `SetMaxResults(uint32)` | Set max results |
| `GetStats() Stats` | Get index statistics |

### SearchResult

```go
type SearchResult struct {
    ID    uint32  // Record ID
    Score float32 // Match score (0.0-1.0)
    Start uint32  // Match start position
    End   uint32  // Match end position
}
```

## Building

The Go binding requires the native FFI library. Build it from the repository root:

```bash
# Build the FFI library
cd rust
cargo build --release -p flash-fuzzy-ffi

# Copy to Go binding
mkdir -p bindings/go/lib
cp target/release/libflash_fuzzy_ffi.a bindings/go/lib/  # Linux/macOS
# or
cp target/release/flash_fuzzy_ffi.lib bindings/go/lib/   # Windows
```

Then build the Go package:

```bash
cd bindings/go
go build
go test -v
```

## Performance

- Sub-millisecond search in 100k+ records
- Bloom filter pre-filtering for O(1) rejection
- Bitap algorithm with bit-parallel operations
- Zero allocations in hot path

## License

MIT
