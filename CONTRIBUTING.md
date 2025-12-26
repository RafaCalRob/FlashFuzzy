# Contributing to Flash-Fuzzy

Thank you for your interest in contributing to Flash-Fuzzy!

## Development Setup

### Prerequisites

- [Rust](https://rustup.rs/) 1.70+
- Node.js 16+
- Git

### Getting Started

```bash
# Clone the repository
git clone https://github.com/RafaCalRob/FlashFuzzy.git
cd FlashFuzzy

# Add WASM target
rustup target add wasm32-unknown-unknown

# Build JavaScript/WASM binding
npm install
npm run build

# Run tests
npm test
```

## How to Contribute

### Reporting Bugs

1. Check existing issues to avoid duplicates
2. Include:
   - Steps to reproduce
   - Expected vs actual behavior
   - Environment (OS, Node version, browser)
   - Minimal reproduction code

### Suggesting Features

1. Open an issue describing the use case
2. Explain why existing features don't solve it

### Pull Requests

1. Fork the repository
2. Create a feature branch: `git checkout -b feature/my-feature`
3. Make your changes
4. Add tests if applicable
5. Run tests: `npm test`
6. Commit with a descriptive message
7. Push and open a PR

## Code Guidelines

### Rust

- Use `rustfmt` for formatting
- Keep `no_std` compatibility
- Document public functions
- No `unsafe` without justification

### TypeScript

- Use strict mode
- Export all public types
- Add JSDoc comments to public API

### Testing

- Write tests for new features
- Cover edge cases
- Keep tests fast and deterministic

## Project Structure

```
FlashFuzzy/
├── rust/
│   ├── core/          # Core Rust implementation
│   ├── wasm/          # WebAssembly binding
│   └── ffi/           # C FFI layer
├── js/                # TypeScript wrapper
├── bindings/
│   ├── python/        # PyO3 binding
│   ├── go/            # CGO binding
│   ├── java/          # JNI binding
│   └── android/       # Android binding
└── tests/             # Test suite
```

## Contribution Areas

### Core Improvements
- Algorithm optimizations
- Memory efficiency
- New search features

### New Bindings
We welcome contributions for new language bindings:
- C# (.NET)
- Swift (iOS/macOS)
- Ruby
- PHP
- Others

If you want to create a binding, please open an issue first to discuss the approach.

## Questions?

Open a GitHub issue with the `question` label.

## License

By contributing, you agree that your contributions will be licensed under the MIT License.

---

Thank you for helping make Flash-Fuzzy better!
