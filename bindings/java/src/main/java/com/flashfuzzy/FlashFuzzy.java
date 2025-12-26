package com.flashfuzzy;

import java.io.*;
import java.nio.file.*;
import java.util.*;

/**
 * High-performance fuzzy search engine powered by Rust.
 *
 * <pre>{@code
 * FlashFuzzy ff = new FlashFuzzy();
 * ff.add(1, "Wireless Headphones");
 * ff.add(2, "Mechanical Keyboard");
 *
 * SearchResult[] results = ff.search("keybord"); // typo
 * for (SearchResult r : results) {
 *     System.out.println("ID: " + r.getId() + ", Score: " + r.getScore());
 * }
 * }</pre>
 */
public class FlashFuzzy implements AutoCloseable {

    private static boolean libraryLoaded = false;

    static {
        loadNativeLibrary();
    }

    private static void loadNativeLibrary() {
        if (libraryLoaded) return;

        String osName = System.getProperty("os.name").toLowerCase();
        String osArch = System.getProperty("os.arch").toLowerCase();

        String libName;
        String libExt;

        if (osName.contains("win")) {
            libName = "flash_fuzzy_jni";
            libExt = ".dll";
        } else if (osName.contains("mac")) {
            libName = "libflash_fuzzy_jni";
            libExt = ".dylib";
        } else {
            libName = "libflash_fuzzy_jni";
            libExt = ".so";
        }

        // Try loading from system path first
        try {
            System.loadLibrary("flash_fuzzy_jni");
            libraryLoaded = true;
            return;
        } catch (UnsatisfiedLinkError e) {
            // Try loading from resources
        }

        // Try loading from JAR resources
        String resourcePath = "/native/" + osName + "-" + osArch + "/" + libName + libExt;
        try (InputStream is = FlashFuzzy.class.getResourceAsStream(resourcePath)) {
            if (is != null) {
                Path tempFile = Files.createTempFile("flash_fuzzy_jni", libExt);
                Files.copy(is, tempFile, StandardCopyOption.REPLACE_EXISTING);
                System.load(tempFile.toAbsolutePath().toString());
                tempFile.toFile().deleteOnExit();
                libraryLoaded = true;
                return;
            }
        } catch (IOException e) {
            throw new RuntimeException("Failed to load native library", e);
        }

        throw new RuntimeException(
            "Native library not found. Please ensure flash_fuzzy_jni is in java.library.path " +
            "or bundled in resources at: " + resourcePath
        );
    }

    // Native methods
    private static native void nativeInit(float threshold, int maxErrors, int maxResults);
    private static native boolean nativeAdd(int id, String text);
    private static native SearchResult[] nativeSearch(String query);
    private static native boolean nativeRemove(int id);
    private static native void nativeReset();
    private static native int nativeGetCount();
    private static native void nativeSetThreshold(float threshold);
    private static native void nativeSetMaxErrors(int maxErrors);
    private static native void nativeSetMaxResults(int maxResults);

    private float threshold;
    private int maxErrors;
    private int maxResults;

    /**
     * Create a new FlashFuzzy instance with default options.
     */
    public FlashFuzzy() {
        this(0.25f, 2, 50);
    }

    /**
     * Create a new FlashFuzzy instance with custom options.
     *
     * @param threshold Minimum score (0.0-1.0), default 0.25
     * @param maxErrors Maximum edit distance (0-3), default 2
     * @param maxResults Maximum results to return, default 50
     */
    public FlashFuzzy(float threshold, int maxErrors, int maxResults) {
        if (threshold < 0 || threshold > 1) {
            throw new IllegalArgumentException("threshold must be between 0 and 1");
        }
        if (maxErrors < 0 || maxErrors > 3) {
            throw new IllegalArgumentException("maxErrors must be between 0 and 3");
        }
        if (maxResults < 1 || maxResults > 100) {
            throw new IllegalArgumentException("maxResults must be between 1 and 100");
        }

        this.threshold = threshold;
        this.maxErrors = maxErrors;
        this.maxResults = maxResults;

        nativeInit(threshold, maxErrors, maxResults);
    }

    /**
     * Add a record to the search index.
     *
     * @param id Unique record ID
     * @param text Text to index
     * @return true if added successfully
     */
    public boolean add(int id, String text) {
        if (text == null || text.isEmpty()) {
            return false;
        }
        return nativeAdd(id, text);
    }

    /**
     * Add multiple records efficiently.
     *
     * @param records Map of ID to text
     * @return Number of records added
     */
    public int addAll(Map<Integer, String> records) {
        int added = 0;
        for (Map.Entry<Integer, String> entry : records.entrySet()) {
            if (add(entry.getKey(), entry.getValue())) {
                added++;
            }
        }
        return added;
    }

    /**
     * Search for matching records.
     *
     * @param query Search query
     * @return Array of search results sorted by score (descending)
     */
    public SearchResult[] search(String query) {
        if (query == null || query.isEmpty()) {
            return new SearchResult[0];
        }
        SearchResult[] results = nativeSearch(query);
        return results != null ? results : new SearchResult[0];
    }

    /**
     * Remove a record by ID.
     *
     * @param id Record ID to remove
     * @return true if found and removed
     */
    public boolean remove(int id) {
        return nativeRemove(id);
    }

    /**
     * Clear all records.
     */
    public void reset() {
        nativeReset();
    }

    /**
     * Get the number of records in the index.
     */
    public int getCount() {
        return nativeGetCount();
    }

    /**
     * Set the minimum score threshold.
     */
    public void setThreshold(float threshold) {
        if (threshold < 0 || threshold > 1) {
            throw new IllegalArgumentException("threshold must be between 0 and 1");
        }
        this.threshold = threshold;
        nativeSetThreshold(threshold);
    }

    /**
     * Set the maximum number of errors allowed.
     */
    public void setMaxErrors(int maxErrors) {
        if (maxErrors < 0 || maxErrors > 3) {
            throw new IllegalArgumentException("maxErrors must be between 0 and 3");
        }
        this.maxErrors = maxErrors;
        nativeSetMaxErrors(maxErrors);
    }

    /**
     * Set the maximum number of results.
     */
    public void setMaxResults(int maxResults) {
        if (maxResults < 1 || maxResults > 100) {
            throw new IllegalArgumentException("maxResults must be between 1 and 100");
        }
        this.maxResults = maxResults;
        nativeSetMaxResults(maxResults);
    }

    public float getThreshold() { return threshold; }
    public int getMaxErrors() { return maxErrors; }
    public int getMaxResults() { return maxResults; }

    @Override
    public void close() {
        reset();
    }

    @Override
    public String toString() {
        return String.format("FlashFuzzy(count=%d, threshold=%.2f, maxErrors=%d)",
            getCount(), threshold, maxErrors);
    }
}
