package com.flashfuzzy;

import org.junit.jupiter.api.*;
import static org.junit.jupiter.api.Assertions.*;

class FlashFuzzyTest {

    private FlashFuzzy ff;

    @BeforeEach
    void setUp() {
        ff = new FlashFuzzy();
    }

    @AfterEach
    void tearDown() {
        ff.close();
    }

    @Test
    void testCreate() {
        assertNotNull(ff);
        assertEquals(0, ff.getCount());
    }

    @Test
    void testCreateWithOptions() {
        FlashFuzzy custom = new FlashFuzzy(0.5f, 1, 10);
        assertEquals(0.5f, custom.getThreshold(), 0.01);
        assertEquals(1, custom.getMaxErrors());
        assertEquals(10, custom.getMaxResults());
        custom.close();
    }

    @Test
    void testAdd() {
        assertTrue(ff.add(1, "Hello World"));
        assertEquals(1, ff.getCount());
    }

    @Test
    void testAddEmpty() {
        assertFalse(ff.add(1, ""));
        assertFalse(ff.add(1, null));
    }

    @Test
    void testSearchExact() {
        ff.add(1, "Wireless Headphones");
        ff.add(2, "Mechanical Keyboard");

        SearchResult[] results = ff.search("keyboard");

        assertTrue(results.length > 0);
        assertEquals(2, results[0].getId());
    }

    @Test
    void testSearchFuzzy() {
        ff.add(1, "Wireless Headphones");
        ff.add(2, "Mechanical Keyboard");

        SearchResult[] results = ff.search("keybord"); // typo

        assertTrue(results.length > 0);
    }

    @Test
    void testSearchCaseInsensitive() {
        ff.add(1, "Hello World");

        SearchResult[] results = ff.search("HELLO");

        assertTrue(results.length > 0);
        assertEquals(1, results[0].getId());
    }

    @Test
    void testSearchEmpty() {
        ff.add(1, "Test");

        SearchResult[] results = ff.search("");

        assertEquals(0, results.length);
    }

    @Test
    void testSearchNoMatch() {
        ff.add(1, "Hello World");

        SearchResult[] results = ff.search("xyz123");

        assertEquals(0, results.length);
    }

    @Test
    void testRemove() {
        ff.add(1, "First");
        ff.add(2, "Second");

        assertTrue(ff.remove(1));
        assertEquals(1, ff.getCount());

        SearchResult[] results = ff.search("first");
        assertEquals(0, results.length);
    }

    @Test
    void testRemoveNonExistent() {
        assertFalse(ff.remove(999));
    }

    @Test
    void testReset() {
        ff.add(1, "Test");
        assertEquals(1, ff.getCount());

        ff.reset();
        assertEquals(0, ff.getCount());
    }

    @Test
    void testInvalidThreshold() {
        assertThrows(IllegalArgumentException.class, () -> new FlashFuzzy(1.5f, 2, 50));
        assertThrows(IllegalArgumentException.class, () -> new FlashFuzzy(-0.1f, 2, 50));
    }

    @Test
    void testInvalidMaxErrors() {
        assertThrows(IllegalArgumentException.class, () -> new FlashFuzzy(0.25f, 5, 50));
    }

    @Test
    void testInvalidMaxResults() {
        assertThrows(IllegalArgumentException.class, () -> new FlashFuzzy(0.25f, 2, 1000));
    }

    @Test
    void testSearchResultProperties() {
        ff.add(42, "Hello World");

        SearchResult[] results = ff.search("hello");

        assertEquals(1, results.length);
        SearchResult r = results[0];
        assertEquals(42, r.getId());
        assertTrue(r.getScore() >= 0 && r.getScore() <= 1);
        assertTrue(r.getStart() >= 0);
        assertTrue(r.getEnd() > r.getStart());
    }

    @Test
    void testPerformance() {
        // Add 10k records
        long startAdd = System.currentTimeMillis();
        for (int i = 0; i < 10000; i++) {
            ff.add(i, "Product item with description " + i);
        }
        long addTime = System.currentTimeMillis() - startAdd;

        assertEquals(10000, ff.getCount());
        System.out.println("Add 10k records: " + addTime + "ms");

        // Search
        long startSearch = System.currentTimeMillis();
        SearchResult[] results = ff.search("product");
        long searchTime = System.currentTimeMillis() - startSearch;

        assertTrue(results.length > 0);
        System.out.println("Search in 10k: " + searchTime + "ms");

        // Performance assertions
        assertTrue(addTime < 1000, "Adding should take < 1s");
        assertTrue(searchTime < 50, "Search should take < 50ms");
    }
}
