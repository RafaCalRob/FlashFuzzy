# Flash-Fuzzy

High-performance fuzzy search engine powered by Rust.

## Installation

```bash
pip install flash-fuzzy
```

## Quick Start

```python
from flash_fuzzy import FlashFuzzy

# Create instance
ff = FlashFuzzy(threshold=0.25, max_errors=2, max_results=50)

# Add records
ff.add([
    {"id": 1, "name": "Wireless Headphones", "category": "Electronics"},
    {"id": 2, "name": "Mechanical Keyboard", "category": "Computers"},
    {"id": 3, "name": "USB-C Cable", "category": "Accessories"},
])

# Search with typos
results = ff.search("keybord")  # Note the typo
for r in results:
    print(f"ID: {r.id}, Score: {r.score:.2f}")
```

## API

### FlashFuzzy

```python
FlashFuzzy(
    threshold: float = 0.25,   # Minimum score (0.0-1.0)
    max_errors: int = 2,       # Max edit distance (0-3)
    max_results: int = 50      # Max results to return
)
```

#### Methods

- `add(records)` - Add a dict or list of dicts
- `search(query)` - Search and return list of SearchResult
- `remove(id)` - Remove record by ID
- `reset()` - Clear all records

#### Properties

- `count` - Number of records
- `threshold` - Get/set threshold
- `max_errors` - Get/set max errors
- `max_results` - Get/set max results

### SearchResult

- `id: int` - Record ID
- `score: float` - Match score (0.0-1.0)
- `start: int` - Match start position
- `end: int` - Match end position

## Performance

- Sub-millisecond search in 100k+ records
- Bloom filter pre-filtering for O(1) rejection
- Bitap algorithm with bit-parallel operations
- Zero-copy where possible

## License

MIT
