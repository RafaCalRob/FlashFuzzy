/**
 * Tests para Flash-Fuzzy
 *
 * Nota: Estos tests requieren que el archivo WASM esté compilado.
 * Ejecutar `npm run build:zig` antes de correr los tests.
 */

import { describe, it, expect, beforeAll, beforeEach } from "vitest";
import { FlashFuzzy, type SearchResult } from "../js/src/index";
import * as fs from "fs";
import * as path from "path";

// Mock para entorno de testing sin WASM real
const mockWasmExists = fs.existsSync(
  path.join(__dirname, "../dist/flash-fuzzy.wasm")
);

describe("FlashFuzzy", () => {
  describe("Types and Interfaces", () => {
    it("should export FlashFuzzy class", () => {
      expect(FlashFuzzy).toBeDefined();
      expect(typeof FlashFuzzy.init).toBe("function");
    });

    it("should have correct SearchResult interface", () => {
      const result: SearchResult = {
        id: 1,
        score: 0.95,
        matches: {
          name: [[0, 5]],
        },
      };

      expect(result.id).toBe(1);
      expect(result.score).toBe(0.95);
      expect(result.matches.name).toEqual([[0, 5]]);
    });
  });

  // Tests de integración (requieren WASM compilado)
  describe.skipIf(!mockWasmExists)("Integration Tests", () => {
    let ff: FlashFuzzy;

    beforeAll(async () => {
      ff = await FlashFuzzy.init({
        wasmUrl: path.join(__dirname, "../dist/flash-fuzzy.wasm"),
        threshold: 0.25,
        maxErrors: 2,
        maxResults: 50,
      });
    });

    beforeEach(() => {
      ff.reset();
    });

    describe("Indexing", () => {
      it("should add single record", () => {
        const added = ff.add({ id: 1, name: "Hello World" });
        expect(added).toBe(1);
        expect(ff.count).toBe(1);
      });

      it("should add multiple records", () => {
        const records = [
          { id: 1, name: "Apple" },
          { id: 2, name: "Banana" },
          { id: 3, name: "Cherry" },
        ];

        const added = ff.add(records);
        expect(added).toBe(3);
        expect(ff.count).toBe(3);
      });

      it("should add batch records efficiently", () => {
        const records = Array.from({ length: 1000 }, (_, i) => ({
          id: i,
          name: `Product ${i}`,
        }));

        const added = ff.addBatch(records);
        expect(added).toBe(1000);
        expect(ff.count).toBe(1000);
      });
    });

    describe("Search", () => {
      beforeEach(() => {
        ff.add([
          { id: 1, name: "Auriculares Bluetooth" },
          { id: 2, name: "Teclado Mecánico Gaming" },
          { id: 3, name: "Mouse Inalámbrico" },
          { id: 4, name: "Monitor 4K HDR" },
        ]);
      });

      it("should find exact matches", () => {
        const results = ff.search("teclado");
        expect(results.length).toBeGreaterThan(0);
        expect(results[0].id).toBe(2);
      });

      it("should find fuzzy matches with typos", () => {
        const results = ff.search("tecldo"); // typo
        expect(results.length).toBeGreaterThan(0);
      });

      it("should be case insensitive", () => {
        const results = ff.search("BLUETOOTH");
        expect(results.length).toBeGreaterThan(0);
        expect(results[0].id).toBe(1);
      });

      it("should return empty for no matches", () => {
        const results = ff.search("xyz123");
        expect(results).toEqual([]);
      });

      it("should return empty for empty query", () => {
        const results = ff.search("");
        expect(results).toEqual([]);
      });
    });

    describe("Schema", () => {
      it("should use schema weights", () => {
        ff.setSchema({
          fields: [
            { name: "name", weight: 1.0 },
            { name: "tags", weight: 0.4 },
          ],
        });

        ff.add([
          { id: 1, name: "Laptop Pro", tags: "gaming electronics" },
          { id: 2, name: "Gaming Console", tags: "laptop accessories" },
        ]);

        const results = ff.search("laptop");
        expect(results.length).toBe(2);
        // El que tiene "laptop" en name debería tener mejor score
        expect(results[0].id).toBe(1);
      });
    });

    describe("Remove and Compact", () => {
      beforeEach(() => {
        ff.add([
          { id: 1, name: "First" },
          { id: 2, name: "Second" },
          { id: 3, name: "Third" },
        ]);
      });

      it("should remove record by id", () => {
        expect(ff.count).toBe(3);
        const removed = ff.remove(2);
        expect(removed).toBe(true);

        // El record sigue contando hasta compact
        ff.compact();

        const results = ff.search("second");
        expect(results.length).toBe(0);
      });

      it("should return false for non-existent id", () => {
        const removed = ff.remove(999);
        expect(removed).toBe(false);
      });
    });

    describe("Serialization", () => {
      it("should return null (not implemented yet)", () => {
        ff.add([
          { id: 1, name: "Test Item" },
          { id: 2, name: "Another Item" },
        ]);

        // Serialization not implemented in WASM yet
        const serialized = ff.serialize();
        expect(serialized).toBeNull();

        // Restore also returns false
        const restored = ff.restore(new Uint8Array(0));
        expect(restored).toBe(false);
      });
    });

    describe("Statistics", () => {
      it("should return correct stats", () => {
        ff.add([
          { id: 1, name: "Short" },
          { id: 2, name: "A much longer text for testing" },
        ]);

        const stats = ff.stats();

        expect(stats.recordCount).toBe(2);
        expect(stats.usedMemory).toBeGreaterThan(0);
        expect(stats.availableMemory).toBeGreaterThan(0);
        expect(stats.stringPoolUsed).toBeGreaterThan(0);
      });
    });

    describe("Configuration", () => {
      it("should respect maxResults setting", () => {
        // Añadir muchos registros similares
        const records = Array.from({ length: 100 }, (_, i) => ({
          id: i,
          name: `Product item ${i}`,
        }));
        ff.add(records);

        ff.setMaxResults(10);
        const results = ff.search("product");

        expect(results.length).toBeLessThanOrEqual(10);
      });

      it("should respect threshold setting", () => {
        ff.add([{ id: 1, name: "abcdefgh" }]);

        ff.setThreshold(0.9); // Alto threshold
        const strictResults = ff.search("abcd"); // Match parcial

        ff.setThreshold(0.2); // Bajo threshold
        const looseResults = ff.search("abcd");

        expect(looseResults.length).toBeGreaterThanOrEqual(
          strictResults.length
        );
      });

      it("should only find exact matches when maxErrors=0", () => {
        ff.reset();
        ff.add([
          { id: 1, name: "Apple iPhone Pro Max" },
          { id: 2, name: "Samsung Galaxy Ultra" },
          { id: 3, name: "Google Pixel Phone" },
          { id: 4, name: "Microsoft Surface Laptop" },
          { id: 5, name: "Dell XPS Desktop" },
          { id: 6, name: "MegaByte Plus Console" },
          { id: 7, name: "UltraCore Hyper Fan" },
          { id: 8, name: "CoreLogic Headphones" },
          { id: 9, name: "TechMax Digital Keyboard" },
        ]);

        ff.setMaxErrors(0);
        ff.setThreshold(0.9);
        ff.setMaxResults(50);

        const results = ff.search("core");

        // Should only find items that contain "core" as substring
        // Items 7 (UltraCore) and 8 (CoreLogic) contain "core"
        // Item 9 (TechMax Digital Keyboard) should NOT match
        expect(results.length).toBe(2);

        const ids = results.map(r => r.id);
        expect(ids).toContain(7); // UltraCore
        expect(ids).toContain(8); // CoreLogic
        expect(ids).not.toContain(9); // TechMax should NOT be in results

        // Verify positions are correct (should span 4 characters for "core")
        for (const result of results) {
          const matchLen = result.matches._default[0][1] - result.matches._default[0][0];
          expect(matchLen).toBeGreaterThanOrEqual(4); // "core" is 4 chars
        }
      });

      it("should not match when pattern not in text with maxErrors=0", () => {
        ff.reset();
        ff.add([
          { id: 1, name: "TechMax Digital Keyboard" },
        ]);

        ff.setMaxErrors(0);
        ff.setThreshold(0.5);

        const results = ff.search("core");

        // "TechMax Digital Keyboard" does NOT contain "core"
        expect(results.length).toBe(0);
      });

      it("should work correctly with addBatch and maxErrors=0", () => {
        ff.reset();

        // Simulate demo data structure with multiple fields
        const records = [
          { id: 1, name: "Apple iPhone Pro Max", category: "Electronics", price: "999.99" },
          { id: 2, name: "Samsung Galaxy Ultra", category: "Electronics", price: "899.99" },
          { id: 3, name: "UltraCore Hyper Fan", category: "Home", price: "49.99" },
          { id: 4, name: "CoreLogic Headphones", category: "Audio", price: "149.99" },
          { id: 5, name: "EliteWare Modern TV", category: "Electronics", price: "1299.99" },
          { id: 6, name: "TechMax Digital Keyboard", category: "Computers", price: "79.99" },
          { id: 7, name: "PowerTech Super Light", category: "Home", price: "29.99" },
          { id: 8, name: "MegaByte Plus Console", category: "Gaming", price: "499.99" },
        ];

        // Use addBatch like the demo does
        const added = ff.addBatch(records);
        expect(added).toBe(8);

        ff.setMaxErrors(0);
        ff.setThreshold(0.5);

        const results = ff.search("core");

        // Should only find items containing "core": UltraCore (3) and CoreLogic (4)
        expect(results.length).toBe(2);

        const ids = results.map(r => r.id);
        expect(ids).toContain(3); // UltraCore
        expect(ids).toContain(4); // CoreLogic
        expect(ids).not.toContain(5); // EliteWare should NOT match
        expect(ids).not.toContain(6); // TechMax should NOT match
      });

      it("should work correctly with large batch and maxErrors=0", () => {
        ff.reset();

        // Create 1000 records, only some with "core"
        const records = [];
        for (let i = 0; i < 1000; i++) {
          if (i === 500) {
            records.push({ id: i, name: "UltraCore Special Item", category: "Test" });
          } else if (i === 750) {
            records.push({ id: i, name: "CoreLogic Premium Device", category: "Test" });
          } else {
            records.push({ id: i, name: `Product Item ${i}`, category: "General" });
          }
        }

        const added = ff.addBatch(records);
        expect(added).toBe(1000);

        ff.setMaxErrors(0);
        ff.setThreshold(0.5);

        const results = ff.search("core");

        // Should only find IDs 500 and 750
        expect(results.length).toBe(2);

        const ids = results.map(r => r.id);
        expect(ids).toContain(500);
        expect(ids).toContain(750);
      });
    });
  });
});

describe("Performance Benchmarks", () => {
  it.skipIf(!mockWasmExists)("should handle 10k records", async () => {
    const ff = await FlashFuzzy.init({
      wasmUrl: path.join(__dirname, "../dist/flash-fuzzy.wasm"),
      initialMemoryPages: 128, // 8MB
    });

    const records = Array.from({ length: 10000 }, (_, i) => ({
      id: i,
      name: `Product ${i} with some description text`,
    }));

    const startIndex = performance.now();
    ff.addBatch(records);
    const indexTime = performance.now() - startIndex;

    expect(ff.count).toBe(10000);
    console.log(`Indexing 10k records: ${indexTime.toFixed(2)}ms`);

    const startSearch = performance.now();
    const results = ff.search("product 500");
    const searchTime = performance.now() - startSearch;

    expect(results.length).toBeGreaterThan(0);
    console.log(`Search in 10k records: ${searchTime.toFixed(2)}ms`);

    // Asegurar rendimiento razonable
    expect(indexTime).toBeLessThan(1000); // < 1s
    expect(searchTime).toBeLessThan(50); // < 50ms
  });
});
