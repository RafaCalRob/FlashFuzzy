# FlashFuzzy

<p align="center">
  <strong>High-performance fuzzy search engine for JavaScript/TypeScript</strong>
</p>

<p align="center">
  <a href="https://www.npmjs.com/package/flashfuzzy"><img src="https://img.shields.io/npm/v/flashfuzzy.svg" alt="npm version"></a>
  <a href="https://github.com/RafaCalRob/FlashFuzzy/blob/main/LICENSE"><img src="https://img.shields.io/badge/license-MIT-blue.svg" alt="license"></a>
  <img src="https://img.shields.io/badge/size-~3KB-brightgreen" alt="bundle size">
</p>

<p align="center">
  <a href="https://bdovenbird.com/flash-fuzzy/">Documentation</a> •
  <a href="https://bdovenbird.com/flash-fuzzy/playground">Live Demo</a> •
  <a href="https://github.com/RafaCalRob/FlashFuzzy">GitHub</a>
</p>

---

## Overview

FlashFuzzy is a high-performance fuzzy search library for JavaScript/TypeScript applications. Built with Rust and compiled to WebAssembly, it delivers exceptional performance for autocomplete, command palettes, product search, and more.

### Key Features

- Sub-millisecond search on 100K+ records
- Typo-tolerant matching with configurable error distance
- Tiny bundle size: ~3KB WASM binary (1.5KB gzipped)
- Zero dependencies
- Framework agnostic: React, Vue, Angular, Svelte, vanilla JS
- Tree-shakeable ESM and CommonJS builds
- Full TypeScript support with type definitions
- Universal: Node.js, browsers, Deno, Bun

### Performance

FlashFuzzy combines Bloom Filter pre-filtering with the Bitap algorithm for maximum speed. The Bloom filter rejects 80-95% of non-matching records in O(1) time before running fuzzy matching.

**Result:** 10-100x faster than traditional fuzzy search libraries.

| Library | Search Time (100K records) | Bundle Size |
|---------|---------------------------|-------------|
| **FlashFuzzy** | **0.8ms** | **3KB** |
| Alternative A | 145ms | 12KB |

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

```javascript
import { FlashFuzzy } from 'flashfuzzy';

const searchEngine = await FlashFuzzy.init({
  threshold: 0.25,
  maxResults: 50,
  maxErrors: 2
});

searchEngine.add([
  { id: 1, name: "Wireless Headphones" },
  { id: 2, name: "Mechanical Keyboard" },
  { id: 3, name: "USB Cable" },
  { id: 4, name: "Laptop Stand" }
]);

const results = searchEngine.search("keyboard");
console.log(results);
// [
//   {
//     id: 2,
//     score: 0.95,
//     matches: {
//       name: {
//         value: "Mechanical Keyboard",
//         ranges: [[11, 19]]
//       }
//     }
//   }
// ]
```

---

## React Integration

### Custom Hook Pattern

```typescript
import { useState, useEffect, useCallback, useMemo } from 'react';
import { FlashFuzzy } from 'flashfuzzy';

interface SearchItem {
  id: number;
  title: string;
  description: string;
}

const useFlashFuzzy = <T extends SearchItem>(
  items: T[],
  config = { threshold: 0.3, maxResults: 10 }
) => {
  const [engine, setEngine] = useState<any>(null);
  const [isReady, setIsReady] = useState(false);

  useEffect(() => {
    FlashFuzzy.init(config).then(instance => {
      instance.add(items);
      setEngine(instance);
      setIsReady(true);
    });
  }, []);

  const search = useCallback(
    (query: string) => (engine && query.trim() ? engine.search(query) : []),
    [engine]
  );

  const addItem = useCallback(
    (item: T) => engine?.addRecord(item),
    [engine]
  );

  const removeItem = useCallback(
    (id: number) => engine?.removeRecord(id),
    [engine]
  );

  return { search, addItem, removeItem, isReady };
};

// Usage
export default function SearchableList() {
  const [query, setQuery] = useState('');
  const { search, isReady } = useFlashFuzzy([
    { id: 1, title: "React Patterns", description: "Advanced patterns" },
    { id: 2, title: "TypeScript Guide", description: "Type safety" }
  ]);

  const results = useMemo(() => search(query), [query, search]);

  if (!isReady) return <div>Loading...</div>;

  return (
    <div>
      <input
        type="text"
        value={query}
        onChange={e => setQuery(e.target.value)}
        placeholder="Search..."
      />
      {results.map(({ id, matches }) => (
        <div key={id}>{matches.title.value}</div>
      ))}
    </div>
  );
}
```

### Higher-Order Component Pattern

```typescript
import { Component, ComponentType } from 'react';
import { FlashFuzzy } from 'flashfuzzy';

interface WithSearchProps {
  searchEngine: any;
  isSearchReady: boolean;
}

const withFlashFuzzy = <P extends object>(
  WrappedComponent: ComponentType<P & WithSearchProps>,
  items: any[],
  config = {}
) => {
  return class extends Component<P, { engine: any; ready: boolean }> {
    state = { engine: null, ready: false };

    async componentDidMount() {
      const engine = await FlashFuzzy.init(config);
      engine.add(items);
      this.setState({ engine, ready: true });
    }

    render() {
      return (
        <WrappedComponent
          {...(this.props as P)}
          searchEngine={this.state.engine}
          isSearchReady={this.state.ready}
        />
      );
    }
  };
};
```

---

## Vue 3 Composition API

```typescript
import { ref, computed, onMounted } from 'vue';
import { FlashFuzzy } from 'flashfuzzy';

export const useSearch = (items: any[], options = {}) => {
  const engine = ref(null);
  const query = ref('');
  const isReady = ref(false);

  onMounted(async () => {
    engine.value = await FlashFuzzy.init(options);
    engine.value.add(items);
    isReady.value = true;
  });

  const results = computed(() =>
    engine.value && query.value.trim()
      ? engine.value.search(query.value)
      : []
  );

  const addRecord = (item: any) => engine.value?.addRecord(item);
  const removeRecord = (id: number) => engine.value?.removeRecord(id);
  const reset = () => engine.value?.reset();

  return {
    query,
    results,
    isReady,
    addRecord,
    removeRecord,
    reset
  };
};

// Component usage
export default {
  setup() {
    const items = [
      { id: 1, name: 'Product Alpha' },
      { id: 2, name: 'Product Beta' }
    ];

    const { query, results, isReady } = useSearch(items, {
      threshold: 0.25,
      maxResults: 20
    });

    return { query, results, isReady };
  }
};
```

---

## Advanced Usage

### Multi-field Search with Weighted Scoring

```typescript
import { FlashFuzzy } from 'flashfuzzy';

interface Product {
  id: number;
  title: string;
  brand: string;
  category: string;
  description: string;
}

const createProductSearch = async (products: Product[]) => {
  const engine = await FlashFuzzy.init({
    threshold: 0.25,
    schema: {
      fields: [
        { name: 'title', weight: 2.0 },
        { name: 'brand', weight: 1.5 },
        { name: 'category', weight: 1.0 },
        { name: 'description', weight: 0.5 }
      ]
    }
  });

  engine.add(products);

  return {
    search: (query: string) => engine.search(query),
    addProduct: (product: Product) => engine.addRecord(product),
    removeProduct: (id: number) => engine.removeRecord(id),
    getStats: () => engine.getStats()
  };
};

// Usage
const productSearch = await createProductSearch([
  {
    id: 1,
    title: "MacBook Pro",
    brand: "Apple",
    category: "Laptops",
    description: "High-performance laptop for professionals"
  }
]);

const results = productSearch.search("laptop");
```

### Command Palette Implementation

```typescript
import { FlashFuzzy } from 'flashfuzzy';

interface Command {
  id: number;
  command: string;
  shortcut: string;
  category: string;
  action: () => void;
}

class CommandPalette {
  private engine: any;
  private commands: Map<number, Command>;

  constructor() {
    this.commands = new Map();
  }

  async initialize(commands: Command[]) {
    this.engine = await FlashFuzzy.init({
      threshold: 0.2,
      maxResults: 10,
      maxErrors: 2,
      schema: {
        fields: [
          { name: 'command', weight: 2.0 },
          { name: 'category', weight: 1.0 }
        ]
      }
    });

    commands.forEach(cmd => {
      this.commands.set(cmd.id, cmd);
      this.engine.addRecord({
        id: cmd.id,
        command: cmd.command,
        category: cmd.category
      });
    });
  }

  search(query: string) {
    return this.engine
      .search(query)
      .map(result => ({
        ...this.commands.get(result.id),
        score: result.score
      }));
  }

  execute(commandId: number) {
    const command = this.commands.get(commandId);
    command?.action();
  }
}

// Usage
const palette = new CommandPalette();
await palette.initialize([
  {
    id: 1,
    command: "Open File",
    shortcut: "Ctrl+O",
    category: "File",
    action: () => console.log("Opening file...")
  },
  {
    id: 2,
    command: "Save File",
    shortcut: "Ctrl+S",
    category: "File",
    action: () => console.log("Saving file...")
  }
]);

const matches = palette.search("open");
```

### Reactive Search with RxJS

```typescript
import { fromEvent, debounceTime, map, switchMap, distinctUntilChanged } from 'rxjs';
import { FlashFuzzy } from 'flashfuzzy';

const createReactiveSearch = async (items: any[], inputElement: HTMLInputElement) => {
  const engine = await FlashFuzzy.init({ threshold: 0.3 });
  engine.add(items);

  return fromEvent(inputElement, 'input').pipe(
    map(event => (event.target as HTMLInputElement).value),
    debounceTime(300),
    distinctUntilChanged(),
    map(query => engine.search(query))
  );
};

// Usage
const searchResults$ = await createReactiveSearch(
  [{ id: 1, name: "Item 1" }],
  document.querySelector('input')
);

searchResults$.subscribe(results => {
  console.log('Search results:', results);
});
```

---

## API Reference

### `FlashFuzzy.init(options)`

Initialize the fuzzy search engine.

**Options:**

```typescript
interface FlashFuzzyOptions {
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

```typescript
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

### User Directory Search

```typescript
const userSearch = await FlashFuzzy.init({
  schema: {
    fields: [
      { name: 'name', weight: 2.0 },
      { name: 'email', weight: 1.0 },
      { name: 'department', weight: 0.5 }
    ]
  }
});
```

### Autocomplete System

```typescript
const autocomplete = await FlashFuzzy.init({
  threshold: 0.2,
  maxResults: 5,
  maxErrors: 1
});
```

---

## How It Works

FlashFuzzy uses a two-phase search algorithm:

### Phase 1: Bloom Filter Pre-filtering

Each record is checked using a 64-bit Bloom filter before running expensive fuzzy matching. This rejects 80-95% of non-matching records in O(1) time.

```
Record: "Wireless Keyboard"
Bloom:  01001010 11000101... (64 bits)

Query:  "keyboard"
Bloom:  00001010 01000001... (64 bits)

Check:  (record_bloom & query_bloom) == query_bloom
        Pass → Run Bitap
        Fail → Skip
```

### Phase 2: Bitap Algorithm

For records that pass the Bloom filter, the Bitap (Shift-Or) algorithm performs bit-parallel fuzzy matching with support for typos, insertions, and deletions.

---

## TypeScript Support

Full TypeScript definitions included:

```typescript
import { FlashFuzzy, type FlashFuzzyOptions, type SearchResult } from 'flashfuzzy';

const options: FlashFuzzyOptions = {
  threshold: 0.25,
  maxResults: 50
};

const engine = await FlashFuzzy.init(options);
const results: SearchResult[] = engine.search("query");
```

---

## Platform Support

### Browser Compatibility

Works in all modern browsers with WebAssembly support:
- Chrome/Edge 57+
- Firefox 52+
- Safari 11+
- Opera 44+

### Node.js

Requires Node.js 16 or higher.

```javascript
// CommonJS
const { FlashFuzzy } = require('flashfuzzy');

// ESM
import { FlashFuzzy } from 'flashfuzzy';
```

### Deno & Bun

```typescript
// Deno
import { FlashFuzzy } from "npm:flashfuzzy";

// Bun
import { FlashFuzzy } from "flashfuzzy";
```

---

## Benchmarks

Run benchmarks yourself:

```bash
git clone https://github.com/RafaCalRob/FlashFuzzy.git
cd FlashFuzzy
npm install
npm test
```

---

## Links

- **Documentation:** https://bdovenbird.com/flash-fuzzy/
- **Live Demo:** https://bdovenbird.com/flash-fuzzy/playground
- **GitHub:** https://github.com/RafaCalRob/FlashFuzzy
- **npm:** https://www.npmjs.com/package/flashfuzzy
- **Issues:** https://github.com/RafaCalRob/FlashFuzzy/issues

---

## Contributing

Contributions are welcome! See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

---

## License

MIT © 2025 [Rafael Calderon Robles](https://www.linkedin.com/in/rafael-c-553545205/)

---

## Credits

Built with Rust, WebAssembly, Bitap Algorithm, and Bloom Filters.
