/**
 * Flash-Fuzzy
 * High-performance fuzzy search engine using Rust + WebAssembly
 */

interface WasmExports {
  memory: WebAssembly.Memory;
  init(): void;
  reset(): void;
  getWriteBuffer(len: number): number;
  commitWrite(len: number): void;
  addRecord(id: number): number;
  removeRecord(id: number): number;
  compact(): number;
  getRecordCount(): number;
  setThreshold(threshold: number): void;
  setMaxErrors(maxErrors: number): void;
  setMaxResults(maxResults: number): void;
  preparePattern(): void;
  search(): number;
  getResultId(i: number): number;
  getResultScore(i: number): number;
  getResultStart(i: number): number;
  getResultEnd(i: number): number;
  getStringPoolUsed(): number;
  getAvailableMemory(): number;
}

export interface FlashFuzzyOptions {
  threshold?: number;
  maxErrors?: number;
  maxResults?: number;
  wasmUrl?: string;
}

export interface FieldSchema {
  name: string;
  weight?: number;
}

export interface SchemaConfig {
  fields: FieldSchema[];
}

export interface SearchResult {
  id: number;
  score: number;
  matches: Record<string, [number, number][]>;
}

export interface IndexStats {
  recordCount: number;
  stringPoolUsed: number;
  availableMemory: number;
  usedMemory: number; // Alias for stringPoolUsed
}

export class FlashFuzzy {
  private wasm: WasmExports | null = null;
  private memoryBuffer: Uint8Array | null = null;
  private encoder = new TextEncoder();
  private schema: SchemaConfig | null = null;
  private initialized = false;

  private constructor() {}

  static async init(options: FlashFuzzyOptions = {}): Promise<FlashFuzzy> {
    const instance = new FlashFuzzy();
    await instance.initialize(options);
    return instance;
  }

  private async initialize(options: FlashFuzzyOptions): Promise<void> {
    const {
      threshold = 0.25,
      maxErrors = 2,
      maxResults = 50,
      wasmUrl,
    } = options;

    const wasmModule = await this.loadWasm(wasmUrl);
    const instance = await WebAssembly.instantiate(wasmModule, {});

    this.wasm = instance.exports as unknown as WasmExports;
    this.memoryBuffer = new Uint8Array(this.wasm.memory.buffer);

    this.wasm.init();
    this.wasm.setThreshold(Math.floor(threshold * 1000));
    this.wasm.setMaxErrors(maxErrors);
    this.wasm.setMaxResults(maxResults);

    this.initialized = true;
  }

  private async loadWasm(wasmUrl?: string): Promise<WebAssembly.Module> {
    const isNode =
      typeof process !== "undefined" &&
      process.versions != null &&
      process.versions.node != null;

    if (isNode && wasmUrl) {
      try {
        const fs = await import("fs");
        const path = await import("path");
        const resolvedPath = path.resolve(wasmUrl);
        const buffer = fs.readFileSync(resolvedPath);
        return await WebAssembly.compile(buffer);
      } catch (e) {
        // Fall through to fetch
      }
    }

    const urls = wasmUrl
      ? [wasmUrl]
      : [
          "./flash-fuzzy.wasm",
          "/flash-fuzzy.wasm",
          typeof import.meta !== "undefined" && import.meta.url
            ? new URL("./flash-fuzzy.wasm", import.meta.url).href
            : "./flash-fuzzy.wasm",
        ];

    let lastError: Error | null = null;

    for (const url of urls) {
      try {
        const response = await fetch(url);
        if (!response.ok) continue;
        const buffer = await response.arrayBuffer();
        return await WebAssembly.compile(buffer);
      } catch (e) {
        lastError = e as Error;
      }
    }

    throw new Error(`Failed to load WASM: ${lastError?.message}`);
  }

  private refreshMemory(): void {
    if (this.wasm) {
      this.memoryBuffer = new Uint8Array(this.wasm.memory.buffer);
    }
  }

  setSchema(config: SchemaConfig): void {
    this.schema = config;
  }

  add<T extends Record<string, unknown>>(records: T | T[]): number {
    if (!this.wasm || !this.initialized) {
      throw new Error("FlashFuzzy not initialized");
    }

    const items = Array.isArray(records) ? records : [records];
    let added = 0;

    for (const record of items) {
      const id = (record.id as number) ?? added;
      const text = this.extractText(record);

      this.refreshMemory();
      const encoded = this.encoder.encode(text);
      const ptr = this.wasm.getWriteBuffer(encoded.length);
      if (ptr === 0) continue;

      this.memoryBuffer!.set(encoded, ptr);
      this.wasm.commitWrite(encoded.length);

      if (this.wasm.addRecord(id) === 1) {
        added++;
      }
    }

    return added;
  }

  addBatch<T extends Record<string, unknown>>(records: T[]): number {
    // Rust version doesn't have batch mode, use add() in loop
    return this.add(records);
  }

  private extractText(record: Record<string, unknown>): string {
    if (!this.schema) {
      return Object.values(record)
        .filter((v) => typeof v === "string")
        .join(" ");
    }

    const parts: string[] = [];
    for (const field of this.schema.fields) {
      const value = record[field.name];
      if (typeof value === "string") {
        parts.push(value);
      }
    }
    return parts.join(" ");
  }

  search(query: string): SearchResult[] {
    if (!this.wasm || !this.initialized) {
      throw new Error("FlashFuzzy not initialized");
    }

    if (query.length === 0) return [];

    this.refreshMemory();

    // Write query to scratchpad
    const encoded = this.encoder.encode(query);
    const ptr = this.wasm.getWriteBuffer(encoded.length);
    if (ptr === 0) return [];

    this.memoryBuffer!.set(encoded, ptr);
    this.wasm.commitWrite(encoded.length);
    this.wasm.preparePattern();

    // Execute search
    const count = this.wasm.search();
    if (count === 0) return [];

    // Collect results
    const results = new Array<SearchResult>(count);
    for (let i = 0; i < count; i++) {
      results[i] = {
        id: this.wasm.getResultId(i),
        score: this.wasm.getResultScore(i) / 1000,
        matches: {
          _default: [[this.wasm.getResultStart(i), this.wasm.getResultEnd(i)]],
        },
      };
    }

    return results;
  }

  remove(id: number): boolean {
    return this.wasm?.removeRecord(id) === 1;
  }

  compact(): void {
    this.wasm?.compact();
  }

  reset(): void {
    this.wasm?.reset();
  }

  setThreshold(threshold: number): void {
    this.wasm?.setThreshold(Math.floor(Math.max(0, Math.min(1, threshold)) * 1000));
  }

  setMaxErrors(maxErrors: number): void {
    this.wasm?.setMaxErrors(Math.max(0, Math.min(3, maxErrors)));
  }

  setMaxResults(maxResults: number): void {
    this.wasm?.setMaxResults(Math.max(1, Math.min(100, maxResults)));
  }

  stats(): IndexStats {
    if (!this.wasm) {
      return { recordCount: 0, stringPoolUsed: 0, availableMemory: 0, usedMemory: 0 };
    }
    const stringPoolUsed = this.wasm.getStringPoolUsed();
    return {
      recordCount: this.wasm.getRecordCount(),
      stringPoolUsed,
      availableMemory: this.wasm.getAvailableMemory(),
      usedMemory: stringPoolUsed, // Alias
    };
  }

  get count(): number {
    return this.wasm?.getRecordCount() ?? 0;
  }

  // Stubs for API compatibility
  serialize(): Uint8Array | null { return null; }
  restore(_data: Uint8Array): boolean { return false; }
  setScoring(_config: unknown): void {}
  getRecordCount(): number { return this.count; }
}

export default FlashFuzzy;
