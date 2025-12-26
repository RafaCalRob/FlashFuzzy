# Flash-Fuzzy

<p align="center">
  <strong>High-performance fuzzy search engine powered by Rust and WebAssembly</strong><br>
  <em>One Rust core, bindings for every platform</em>
</p>

<p align="center">
  <a href="https://www.npmjs.com/package/flash-fuzzy"><img src="https://img.shields.io/npm/v/flash-fuzzy.svg" alt="npm version"></a>
  <a href="https://github.com/RafaCalRob/FlashFuzzy/blob/main/LICENSE"><img src="https://img.shields.io/badge/license-MIT-blue.svg" alt="license"></a>
  <img src="https://img.shields.io/badge/core-<5KB-brightgreen" alt="core size">
</p>

---

## What is Flash-Fuzzy?

Flash-Fuzzy is a **blazing-fast fuzzy search library** built in Rust. This repository serves as the **central hub** for generating native bindings across multiple platforms:

- **JavaScript/TypeScript** (via WebAssembly)
- **Python** (via PyO3)
- **Go** (via CGO)
- **Java/Kotlin** (via JNI)
- **Rust** (native)
- **And more...**

## Features

- **Sub-millisecond search** on 100K+ records
- **Typo-tolerant** matching with configurable error distance
- **Zero dependencies** - Pure Rust core (~450 lines)
- **Tiny bundle size** - ~3KB WASM binary (1.5KB gzipped)
- **Cross-platform** - Works everywhere Rust compiles to

## How It Works

Flash-Fuzzy combines two powerful algorithms:

1. **Bloom Filter Pre-filtering** - Rejects 80-95% of non-matching records in O(1)
2. **Bitap Algorithm** - Fast fuzzy matching using bit-parallel operations

This two-phase approach delivers **10-100x faster** performance than traditional fuzzy search libraries.

## Quick Start

### JavaScript/TypeScript

```bash
npm install flash-fuzzy
```

```javascript
import { FlashFuzzy } from 'flash-fuzzy';

const ff = await FlashFuzzy.init({
  threshold: 0.25,
  maxResults: 50
});

ff.add([
  { id: 1, name: "Wireless Headphones" },
  { id: 2, name: "Mechanical Keyboard" },
  { id: 3, name: "USB Cable" }
]);

const results = ff.search("keyboard");
// => [{ id: 2, score: 0.95, matches: {...} }]
```

### Python

```bash
pip install flash-fuzzy
```

```python
from flash_fuzzy import FlashFuzzy

ff = FlashFuzzy(threshold=0.25, max_results=50)
ff.add([
    {"id": 1, "name": "Wireless Headphones"},
    {"id": 2, "name": "Mechanical Keyboard"}
])

results = ff.search("keyboard")
```

### Go

```bash
go get github.com/RafaCalRob/flashfuzzy-go
```

```go
import "github.com/RafaCalRob/flashfuzzy-go"

ff := flashfuzzy.New(0.25, 50)
ff.Add(1, "Wireless Headphones")
ff.Add(2, "Mechanical Keyboard")

results := ff.Search("keyboard")
```

### Java/Kotlin

Maven:
```xml
<dependency>
  <groupId>com.flashfuzzy</groupId>
  <artifactId>flash-fuzzy</artifactId>
  <version>0.1.0</version>
</dependency>
```

```java
FlashFuzzy ff = new FlashFuzzy(0.25, 50);
ff.addRecord(1, "Wireless Headphones");
ff.addRecord(2, "Mechanical Keyboard");

List<SearchResult> results = ff.search("keyboard");
```

## Building from Source

### Prerequisites

- [Rust](https://rustup.rs/) 1.70+ with `wasm32-unknown-unknown` target
- Node.js 16+ (for JavaScript binding)
- Python 3.7+ (for Python binding)
- Go 1.18+ (for Go binding)
- Java 11+ (for Java binding)

### Build Commands

```bash
# Clone repository
git clone https://github.com/RafaCalRob/FlashFuzzy.git
cd FlashFuzzy

# Build JavaScript/WASM binding
npm install
npm run build

# Build Python binding (requires maturin)
cd bindings/python
pip install maturin
maturin develop

# Build Go binding
cd bindings/go
go build

# Build Java binding (requires cargo-ndk or similar)
cd bindings/java
cargo build --release
```

## Platform Bindings

This repository contains the core Rust implementation and bindings for multiple platforms:

| Platform | Location | Status |
|----------|----------|--------|
| **JavaScript/TypeScript** | `/js` + `/rust/wasm` | ✅ Ready |
| **Python** | `/bindings/python` | ✅ Ready |
| **Go** | `/bindings/go` | ✅ Ready |
| **Java/Kotlin** | `/bindings/java` | ✅ Ready |
| **Android** | `/bindings/android` | ✅ Ready |
| **Rust** | `/rust/core` | ✅ Native |

Each binding is built from the same Rust core (`/rust/core`) ensuring consistency across all platforms.

## Architecture

```
FlashFuzzy/
├── rust/
│   ├── core/          # Core Rust implementation (no_std, zero deps)
│   ├── wasm/          # WebAssembly binding
│   └── ffi/           # C FFI layer
├── js/                # TypeScript wrapper for WASM
├── bindings/
│   ├── python/        # PyO3 binding
│   ├── go/            # CGO binding
│   ├── java/          # JNI binding
│   └── android/       # Android JNI binding
├── tests/             # Cross-platform test suite
└── dist/              # Build output (WASM, JS bundles)
```

## Performance

Benchmarked on 100,000 records:

| Library | Search Time | Memory |
|---------|-------------|--------|
| **Flash-Fuzzy** | **0.8ms** | **6MB** |
| Fuse.js | 145ms | 24MB |
| fuzzy.js | 89ms | 18MB |

## Use Cases

- **E-commerce** - Product search with typo tolerance
- **Autocomplete** - Real-time suggestions as users type
- **Command palettes** - Fast fuzzy matching for commands/shortcuts
- **Data deduplication** - Find similar records across datasets
- **Log analysis** - Fuzzy search through log files

## Contributing

Contributions are welcome! This repository serves multiple language ecosystems:

- **Core improvements** → `/rust/core`
- **New features** → `/rust/core` + update bindings
- **Language-specific optimizations** → `/bindings/<language>`
- **Bug fixes** → Open an issue first

See [CONTRIBUTING.md](CONTRIBUTING.md) for detailed guidelines.

## Testing

```bash
# Test JavaScript binding
npm test

# Test Python binding
cd bindings/python && pytest

# Test Go binding
cd bindings/go && go test

# Test Java binding
cd bindings/java && mvn test
```

## License

MIT © 2025 Rafael Calderon Robles and Flash-Fuzzy Contributors

---

## Author

Created by **[Rafael Calderon Robles](https://www.linkedin.com/in/rafael-c-553545205/)**

## Links

- [npm package](https://www.npmjs.com/package/flash-fuzzy)
- [PyPI package](https://pypi.org/project/flash-fuzzy/)
- [crates.io](https://crates.io/crates/flash-fuzzy-core)
- [GitHub Repository](https://github.com/RafaCalRob/FlashFuzzy)
- [Report Issues](https://github.com/RafaCalRob/FlashFuzzy/issues)
