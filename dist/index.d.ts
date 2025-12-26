/**
 * Flash-Fuzzy
 * High-performance fuzzy search engine using Rust + WebAssembly
 */
interface FlashFuzzyOptions {
    threshold?: number;
    maxErrors?: number;
    maxResults?: number;
    wasmUrl?: string;
}
interface FieldSchema {
    name: string;
    weight?: number;
}
interface SchemaConfig {
    fields: FieldSchema[];
}
interface SearchResult {
    id: number;
    score: number;
    matches: Record<string, [number, number][]>;
}
interface IndexStats {
    recordCount: number;
    stringPoolUsed: number;
    availableMemory: number;
    usedMemory: number;
}
declare class FlashFuzzy {
    private wasm;
    private memoryBuffer;
    private encoder;
    private schema;
    private initialized;
    private constructor();
    static init(options?: FlashFuzzyOptions): Promise<FlashFuzzy>;
    private initialize;
    private loadWasm;
    private refreshMemory;
    setSchema(config: SchemaConfig): void;
    add<T extends Record<string, unknown>>(records: T | T[]): number;
    addBatch<T extends Record<string, unknown>>(records: T[]): number;
    private extractText;
    search(query: string): SearchResult[];
    remove(id: number): boolean;
    compact(): void;
    reset(): void;
    setThreshold(threshold: number): void;
    setMaxErrors(maxErrors: number): void;
    setMaxResults(maxResults: number): void;
    stats(): IndexStats;
    get count(): number;
    serialize(): Uint8Array | null;
    restore(_data: Uint8Array): boolean;
    setScoring(_config: unknown): void;
    getRecordCount(): number;
}

export { type FieldSchema, FlashFuzzy, type FlashFuzzyOptions, type IndexStats, type SchemaConfig, type SearchResult, FlashFuzzy as default };
