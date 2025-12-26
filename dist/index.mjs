// js/src/index.ts
var FlashFuzzy = class _FlashFuzzy {
  constructor() {
    this.wasm = null;
    this.memoryBuffer = null;
    this.encoder = new TextEncoder();
    this.schema = null;
    this.initialized = false;
  }
  static async init(options = {}) {
    const instance = new _FlashFuzzy();
    await instance.initialize(options);
    return instance;
  }
  async initialize(options) {
    const {
      threshold = 0.25,
      maxErrors = 2,
      maxResults = 50,
      wasmUrl
    } = options;
    const wasmModule = await this.loadWasm(wasmUrl);
    const instance = await WebAssembly.instantiate(wasmModule, {});
    this.wasm = instance.exports;
    this.memoryBuffer = new Uint8Array(this.wasm.memory.buffer);
    this.wasm.init();
    this.wasm.setThreshold(Math.floor(threshold * 1e3));
    this.wasm.setMaxErrors(maxErrors);
    this.wasm.setMaxResults(maxResults);
    this.initialized = true;
  }
  async loadWasm(wasmUrl) {
    const isNode = typeof process !== "undefined" && process.versions != null && process.versions.node != null;
    if (isNode && wasmUrl) {
      try {
        const fs = await import("fs");
        const path = await import("path");
        const resolvedPath = path.resolve(wasmUrl);
        const buffer = fs.readFileSync(resolvedPath);
        return await WebAssembly.compile(buffer);
      } catch (e) {
      }
    }
    const urls = wasmUrl ? [wasmUrl] : [
      "./flash-fuzzy.wasm",
      "/flash-fuzzy.wasm",
      typeof import.meta !== "undefined" && import.meta.url ? new URL("./flash-fuzzy.wasm", import.meta.url).href : "./flash-fuzzy.wasm"
    ];
    let lastError = null;
    for (const url of urls) {
      try {
        const response = await fetch(url);
        if (!response.ok) continue;
        const buffer = await response.arrayBuffer();
        return await WebAssembly.compile(buffer);
      } catch (e) {
        lastError = e;
      }
    }
    throw new Error(`Failed to load WASM: ${lastError?.message}`);
  }
  refreshMemory() {
    if (this.wasm) {
      this.memoryBuffer = new Uint8Array(this.wasm.memory.buffer);
    }
  }
  setSchema(config) {
    this.schema = config;
  }
  add(records) {
    if (!this.wasm || !this.initialized) {
      throw new Error("FlashFuzzy not initialized");
    }
    const items = Array.isArray(records) ? records : [records];
    let added = 0;
    for (const record of items) {
      const id = record.id ?? added;
      const text = this.extractText(record);
      this.refreshMemory();
      const encoded = this.encoder.encode(text);
      const ptr = this.wasm.getWriteBuffer(encoded.length);
      if (ptr === 0) continue;
      this.memoryBuffer.set(encoded, ptr);
      this.wasm.commitWrite(encoded.length);
      if (this.wasm.addRecord(id) === 1) {
        added++;
      }
    }
    return added;
  }
  addBatch(records) {
    return this.add(records);
  }
  extractText(record) {
    if (!this.schema) {
      return Object.values(record).filter((v) => typeof v === "string").join(" ");
    }
    const parts = [];
    for (const field of this.schema.fields) {
      const value = record[field.name];
      if (typeof value === "string") {
        parts.push(value);
      }
    }
    return parts.join(" ");
  }
  search(query) {
    if (!this.wasm || !this.initialized) {
      throw new Error("FlashFuzzy not initialized");
    }
    if (query.length === 0) return [];
    this.refreshMemory();
    const encoded = this.encoder.encode(query);
    const ptr = this.wasm.getWriteBuffer(encoded.length);
    if (ptr === 0) return [];
    this.memoryBuffer.set(encoded, ptr);
    this.wasm.commitWrite(encoded.length);
    this.wasm.preparePattern();
    const count = this.wasm.search();
    if (count === 0) return [];
    const results = new Array(count);
    for (let i = 0; i < count; i++) {
      results[i] = {
        id: this.wasm.getResultId(i),
        score: this.wasm.getResultScore(i) / 1e3,
        matches: {
          _default: [[this.wasm.getResultStart(i), this.wasm.getResultEnd(i)]]
        }
      };
    }
    return results;
  }
  remove(id) {
    return this.wasm?.removeRecord(id) === 1;
  }
  compact() {
    this.wasm?.compact();
  }
  reset() {
    this.wasm?.reset();
  }
  setThreshold(threshold) {
    this.wasm?.setThreshold(Math.floor(Math.max(0, Math.min(1, threshold)) * 1e3));
  }
  setMaxErrors(maxErrors) {
    this.wasm?.setMaxErrors(Math.max(0, Math.min(3, maxErrors)));
  }
  setMaxResults(maxResults) {
    this.wasm?.setMaxResults(Math.max(1, Math.min(100, maxResults)));
  }
  stats() {
    if (!this.wasm) {
      return { recordCount: 0, stringPoolUsed: 0, availableMemory: 0, usedMemory: 0 };
    }
    const stringPoolUsed = this.wasm.getStringPoolUsed();
    return {
      recordCount: this.wasm.getRecordCount(),
      stringPoolUsed,
      availableMemory: this.wasm.getAvailableMemory(),
      usedMemory: stringPoolUsed
      // Alias
    };
  }
  get count() {
    return this.wasm?.getRecordCount() ?? 0;
  }
  // Stubs for API compatibility
  serialize() {
    return null;
  }
  restore(_data) {
    return false;
  }
  setScoring(_config) {
  }
  getRecordCount() {
    return this.count;
  }
};
var index_default = FlashFuzzy;
export {
  FlashFuzzy,
  index_default as default
};
