# FlashFuzzy

<p align="center">
  <strong>High-performance fuzzy search engine for JavaScript/TypeScript</strong><br>
  <em>Powered by Rust and WebAssembly - Fast, lightweight, zero dependencies</em>
</p>

<p align="center">
  <a href="https://www.npmjs.com/package/flashfuzzy"><img src="https://img.shields.io/npm/v/flashfuzzy.svg" alt="npm version"></a>
  <a href="https://github.com/RafaCalRob/FlashFuzzy/blob/main/LICENSE"><img src="https://img.shields.io/badge/license-MIT-blue.svg" alt="license"></a>
  <img src="https://img.shields.io/badge/size-~3KB-brightgreen" alt="bundle size">
  <img src="https://img.shields.io/badge/WASM-yes-orange" alt="WebAssembly">
</p>

<p align="center">
  <a href="https://bdovenbird.com/flash-fuzzy/">📖 Documentation</a> •
  <a href="https://bdovenbird.com/flash-fuzzy/playground">🎮 Live Demo</a> •
  <a href="https://github.com/RafaCalRob/FlashFuzzy">💻 GitHub</a>
</p>

---

## Why FlashFuzzy?

FlashFuzzy is a **blazing-fast fuzzy search library** for JavaScript/TypeScript applications. Built with Rust and compiled to WebAssembly, it delivers exceptional performance for autocomplete, command palettes, product search, and more.

### Features

- ⚡ **Sub-millisecond search** on 100K+ records
- 🎯 **Typo-tolerant** matching with configurable error distance
- 🪶 **Tiny bundle** - ~3KB WASM binary (1.5KB gzipped)
- 🚀 **Zero dependencies** - Pure Rust core
- 🔧 **Framework agnostic** - Works with React, Vue, Angular, Svelte, vanilla JS
- 📦 **Tree-shakeable** - ESM and CommonJS builds
- 💪 **TypeScript** support with full type definitions
- 🌐 **Universal** - Node.js, browsers, Deno, Bun

### Performance

FlashFuzzy combines two powerful algorithms for maximum speed:

1. **Bloom Filter Pre-filtering** - Rejects 80-95% of non-matching records in O(1)
2. **Bitap Algorithm** - Bit-parallel fuzzy matching

**Result:** 10-100x faster than traditional fuzzy search libraries.

| Library | Search Time (10K records) | Bundle Size |
|---------|--------------------------|-------------|
| **FlashFuzzy** | **0.8ms** | **3KB** |
| Fuse.js | 145ms | 12KB |
| fuzzy.js | 89ms | 8KB |

---

## Installation

```bash
npm install flashfuzzy
```

Or with other package managers:

```bash
yarn add flashfuzzy
pnpm add flashfuzzy
bun add flashfuzzy
```

---

## Quick Start

### Basic Usage

```javascript
import { FlashFuzzy } from 'flashfuzzy';

// Initialize
const ff = await FlashFuzzy.init({
  threshold: 0.25,     // Lower = stricter matching
  maxResults: 50,      // Limit results
  maxErrors: 2         // Max typos allowed
});

// Add records
ff.add([
  { id: 1, name: "Wireless Headphones" },
  { id: 2, name: "Mechanical Keyboard" },
  { id: 3, name: "USB Cable" },
  { id: 4, name: "Laptop Stand" }
]);

// Search
const results = ff.search("keyboard");
// => [{ id: 2, score: 0.95, matches: {...} }]

console.log(results);
// [
//   {
//     id: 2,
//     score: 0.95,
//     matches: {
//       name: {
//         value: "Mechanical Keyboard",
//         ranges: [[11, 19]]  // Matched "Keyboard"
//       }
//     }
//   }
// ]
```

### React Example

```jsx
import { useState, useEffect } from 'react';
import { FlashFuzzy } from 'flashfuzzy';

function SearchComponent() {
  const [ff, setFf] = useState(null);
  const [query, setQuery] = useState('');
  const [results, setResults] = useState([]);

  useEffect(() => {
    FlashFuzzy.init({ threshold: 0.3, maxResults: 10 })
      .then(fuzzy => {
        fuzzy.add([
          { id: 1, title: "React Documentation" },
          { id: 2, title: "TypeScript Guide" },
          { id: 3, title: "WebAssembly Tutorial" }
        ]);
        setFf(fuzzy);
      });
  }, []);

  const handleSearch = (e) => {
    const value = e.target.value;
    setQuery(value);

    if (ff && value.trim()) {
      setResults(ff.search(value));
    } else {
      setResults([]);
    }
  };

  return (
    <div>
      <input
        type="text"
        value={query}
        onChange={handleSearch}
        placeholder="Search..."
      />
      <ul>
        {results.map(result => (
          <li key={result.id}>
            {result.matches.title.value} (score: {result.score})
          </li>
        ))}
      </ul>
    </div>
  );
}
```

### Vue 3 Example

```vue
<template>
  <div>
    <input v-model="query" @input="search" placeholder="Search..." />
    <ul>
      <li v-for="result in results" :key="result.id">
        {{ result.matches.name.value }} ({{ result.score }})
      </li>
    </ul>
  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue';
import { FlashFuzzy } from 'flashfuzzy';

const query = ref('');
const results = ref([]);
let ff = null;

onMounted(async () => {
  ff = await FlashFuzzy.init({ threshold: 0.3 });
  ff.add([
    { id: 1, name: 'Product A' },
    { id: 2, name: 'Product B' }
  ]);
});

const search = () => {
  results.value = ff ? ff.search(query.value) : [];
};
</script>
```

### Next.js Example

```tsx
'use client';

import { useEffect, useState } from 'react';
import { FlashFuzzy } from 'flashfuzzy';

export default function SearchPage() {
  const [ff, setFf] = useState<any>(null);

  useEffect(() => {
    FlashFuzzy.init({ threshold: 0.25 }).then(setFf);
  }, []);

  // ... rest of component
}
```

---

## Advanced Usage

### Schema Support (Multi-field Search)

Search across multiple fields with different weights:

```javascript
const ff = await FlashFuzzy.init({
  threshold: 0.25,
  schema: {
    fields: [
      { name: 'title', weight: 2.0 },    // Title is more important
      { name: 'description', weight: 1.0 },
      { name: 'tags', weight: 1.5 }
    ]
  }
});

ff.add([
  {
    id: 1,
    title: "MacBook Pro",
    description: "Powerful laptop for developers",
    tags: "apple computer notebook"
  }
]);

const results = ff.search("laptop");
// Matches in 'description' field
```

### Autocomplete

```javascript
import { FlashFuzzy } from 'flashfuzzy';

const autocomplete = await FlashFuzzy.init({
  threshold: 0.2,
  maxResults: 5,
  maxErrors: 1  // Strict for autocomplete
});

autocomplete.add([
  { id: 1, command: "Open File" },
  { id: 2, command: "Save File" },
  { id: 3, command: "Close Window" }
]);

// User types: "opn"
const suggestions = autocomplete.search("opn");
// => [{ id: 1, command: "Open File", score: 0.85 }]
```

### Real-time Updates

```javascript
// Add records dynamically
ff.addRecord({ id: 100, name: "New Product" });

// Remove records
ff.removeRecord(100);

// Clear all
ff.reset();

// Get stats
const stats = ff.getStats();
console.log(stats);
// {
//   recordCount: 1000,
//   stringPoolUsed: 45123,
//   availableMemory: 4149877
// }
```

---

## API Reference

### `FlashFuzzy.init(options)`

Initialize the fuzzy search engine.

**Options:**

```typescript
{
  threshold?: number;        // 0-1, default: 0.25 (lower = stricter)
  maxResults?: number;       // default: 100
  maxErrors?: number;        // default: adaptive based on pattern length
  caseSensitive?: boolean;   // default: false
  schema?: {
    fields: Array<{
      name: string;
      weight?: number;       // default: 1.0
    }>
  }
}
```

**Returns:** `Promise<FlashFuzzyInstance>`

### Instance Methods

#### `search(query: string): Result[]`

Search for records matching the query.

```typescript
interface Result {
  id: number;
  score: number;  // 0-1, higher is better
  matches: {
    [fieldName: string]: {
      value: string;
      ranges: [number, number][];  // Matched character ranges
    }
  }
}
```

#### `add(records: Record[]): void`

Add multiple records at once.

```typescript
interface Record {
  id: number;
  [key: string]: any;  // Your searchable fields
}
```

#### `addRecord(record: Record): void`

Add a single record.

#### `removeRecord(id: number): void`

Remove a record by ID.

#### `reset(): void`

Clear all records.

#### `getStats(): Stats`

Get memory and performance statistics.

```typescript
interface Stats {
  recordCount: number;
  stringPoolUsed: number;
  availableMemory: number;
}
```

---

## Use Cases

### E-commerce Product Search

```javascript
const productSearch = await FlashFuzzy.init({
  threshold: 0.3,
  schema: {
    fields: [
      { name: 'name', weight: 2.0 },
      { name: 'brand', weight: 1.5 },
      { name: 'category', weight: 1.0 },
      { name: 'description', weight: 0.5 }
    ]
  }
});

productSearch.add(products);
```

### Command Palette (VSCode-style)

```javascript
const commands = await FlashFuzzy.init({
  threshold: 0.2,
  maxResults: 10,
  maxErrors: 2
});
```

### User Directory Search

```javascript
const users = await FlashFuzzy.init({
  schema: {
    fields: [
      { name: 'name', weight: 2.0 },
      { name: 'email', weight: 1.0 },
      { name: 'department', weight: 0.5 }
    ]
  }
});
```

---

## How It Works

FlashFuzzy uses a two-phase search algorithm:

### Phase 1: Bloom Filter Pre-filtering

Before running expensive fuzzy matching, each record is checked using a 64-bit Bloom filter. This rejects 80-95% of records in O(1) time.

```
Record: "Wireless Keyboard"
Bloom:  01001010 11000101... (64 bits)

Query:  "keyboard"
Bloom:  00001010 01000001... (64 bits)

Check:  (record_bloom & query_bloom) == query_bloom
        ✓ Might match → Run Bitap
        ✗ No match → Skip (most records)
```

### Phase 2: Bitap Algorithm

For records that pass the Bloom filter, the Bitap (Shift-Or) algorithm performs bit-parallel fuzzy matching with support for typos, insertions, and deletions.

This combination delivers exceptional performance while maintaining high accuracy.

---

## TypeScript Support

FlashFuzzy is written in TypeScript with full type definitions:

```typescript
import { FlashFuzzy, type FlashFuzzyOptions, type SearchResult } from '@bdovenbird/flashfuzzy';

const options: FlashFuzzyOptions = {
  threshold: 0.25,
  maxResults: 50
};

const ff = await FlashFuzzy.init(options);

const results: SearchResult[] = ff.search("query");
```

---

## Browser Support

FlashFuzzy works in all modern browsers with WebAssembly support:

- ✅ Chrome/Edge 57+
- ✅ Firefox 52+
- ✅ Safari 11+
- ✅ Opera 44+

For older browsers, you'll need a WASM polyfill.

---

## Node.js Support

Requires Node.js 16 or higher.

```javascript
// CommonJS
const { FlashFuzzy } = require('flashfuzzy');

// ESM
import { FlashFuzzy } from 'flashfuzzy';
```

---

## Deno & Bun

FlashFuzzy works with Deno and Bun out of the box:

```typescript
// Deno
import { FlashFuzzy } from "npm:flashfuzzy";

// Bun
import { FlashFuzzy } from "flashfuzzy";
```

---

## Benchmarks

Run the benchmarks yourself:

```bash
git clone https://github.com/RafaCalRob/FlashFuzzy.git
cd FlashFuzzy
npm install
npm test
```

---

## Links

- 📖 **Documentation:** https://bdovenbird.com/flash-fuzzy/
- 🎮 **Live Demo:** https://bdovenbird.com/flash-fuzzy/playground
- 💻 **GitHub:** https://github.com/RafaCalRob/FlashFuzzy
- 📦 **npm:** https://www.npmjs.com/package/flashfuzzy
- 🐛 **Issues:** https://github.com/RafaCalRob/FlashFuzzy/issues

---

## Contributing

Contributions are welcome! See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

---

## License

MIT © 2025 [Rafael Calderon Robles](https://www.linkedin.com/in/rafael-c-553545205/)

---

## Credits

Built with:
- **Rust** - Core implementation
- **WebAssembly** - Fast, portable execution
- **Bitap Algorithm** - Efficient fuzzy matching
- **Bloom Filters** - Fast pre-filtering

---

**Made with ❤️ for the JavaScript community**
