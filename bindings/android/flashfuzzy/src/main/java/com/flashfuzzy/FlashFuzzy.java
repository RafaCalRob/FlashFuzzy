package com.flashfuzzy;

import android.content.Context;
import java.io.*;
import java.util.*;

/**
 * High-performance fuzzy search engine for Android, powered by Rust.
 *
 * <pre>{@code
 * FlashFuzzy ff = new FlashFuzzy(context);
 * ff.add(1, "Wireless Headphones");
 * ff.add(2, "Mechanical Keyboard");
 *
 * SearchResult[] results = ff.search("keybord");
 * }</pre>
 */
public class FlashFuzzy implements AutoCloseable {

    private static boolean libraryLoaded = false;

    static {
        try {
            System.loadLibrary("flash_fuzzy_jni");
            libraryLoaded = true;
        } catch (UnsatisfiedLinkError e) {
            android.util.Log.e("FlashFuzzy", "Failed to load native library", e);
        }
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
     * Check if the native library is loaded.
     */
    public static boolean isLibraryLoaded() {
        return libraryLoaded;
    }

    /**
     * Create a new FlashFuzzy instance with default options.
     */
    public FlashFuzzy() {
        this(0.25f, 2, 50);
    }

    /**
     * Create a new FlashFuzzy instance with custom options.
     */
    public FlashFuzzy(float threshold, int maxErrors, int maxResults) {
        if (!libraryLoaded) {
            throw new IllegalStateException("Native library not loaded");
        }

        this.threshold = Math.max(0, Math.min(1, threshold));
        this.maxErrors = Math.max(0, Math.min(3, maxErrors));
        this.maxResults = Math.max(1, Math.min(100, maxResults));

        nativeInit(this.threshold, this.maxErrors, this.maxResults);
    }

    /**
     * Add a record to the search index.
     */
    public boolean add(int id, String text) {
        if (text == null || text.isEmpty()) {
            return false;
        }
        return nativeAdd(id, text);
    }

    /**
     * Add multiple records.
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
     * Get record count.
     */
    public int getCount() {
        return nativeGetCount();
    }

    public void setThreshold(float threshold) {
        this.threshold = Math.max(0, Math.min(1, threshold));
        nativeSetThreshold(this.threshold);
    }

    public void setMaxErrors(int maxErrors) {
        this.maxErrors = Math.max(0, Math.min(3, maxErrors));
        nativeSetMaxErrors(this.maxErrors);
    }

    public void setMaxResults(int maxResults) {
        this.maxResults = Math.max(1, Math.min(100, maxResults));
        nativeSetMaxResults(this.maxResults);
    }

    public float getThreshold() { return threshold; }
    public int getMaxErrors() { return maxErrors; }
    public int getMaxResults() { return maxResults; }

    @Override
    public void close() {
        reset();
    }
}
